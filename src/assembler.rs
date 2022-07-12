use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Instruction {
    ADD,
    SUB,
    MUL,
    DIV,
    CMP,
    BRC,
    BEQ,
    BGT,
    BLT,
    NOP,
    HLT,
}

#[derive(Copy, Clone)]
pub enum Register {
    PC,
    R1,
    R2,
    RT,
    RO,
    RI,
}


enum InterimOp {
    Reg(Register),
    Direct(i16),
    Lable(String),
}

#[derive(Debug)]
pub enum CodeError {
    TooManyOps(usize),
    UnknownOp(usize),
    UnknownInst(usize),
    MissingLable(usize),
    MisformedLable(usize),
    UnknownNumber(usize),
    TooManySpaces(usize),
    JumpNeedsLable(usize),
    InstAfterLable(usize),
    TooFewOps(usize),
    InvalidOp(usize),
}

struct InterimLine {
    lable : Option<String>,
    instr : Option<Instruction>,
    op1   : Option<InterimOp>,
    op2   : Option<InterimOp>,
    op3   : Option<InterimOp>,
}

impl InterimLine {
    pub fn new() -> Self {
        InterimLine {
            lable : None,
            instr : None,
            op1 : None,
            op2 : None,
            op3 : None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Operand {
    Reg(Register),
    Direct(i16)
}

#[derive(Copy, Clone)]
pub struct Line {
    pub instr : Instruction,
    pub op1   : Option<Operand>,
    pub op2   : Option<Operand>,
    pub op3   : Option<Operand>,
}


fn get_operand(word : &str, line_index : usize) -> Result<InterimOp, CodeError> {
    Ok(
        match word.to_uppercase().as_str() {
            "PC" => InterimOp::Reg(Register::PC),
            "R1" => InterimOp::Reg(Register::R1),
            "R2" => InterimOp::Reg(Register::R2),
            "RT" => InterimOp::Reg(Register::RT),
            "RO" => InterimOp::Reg(Register::RO),
            "RI" => InterimOp::Reg(Register::RI),
            _ => {
                if word.starts_with("#") {
                    match word.split_at(1).1.parse::<u16>() {
                         Ok(n) => InterimOp::Direct(n as i16),
                         _ => { return Err(CodeError::UnknownNumber(line_index)); }
                    }
                } else { InterimOp::Lable(word.to_string()) }
            }
        }
    )
}

fn get_instruction(text: &str) -> Result<Instruction, ()> {
    match text.to_uppercase().as_str() {
                            "ADD" => Ok(Instruction::ADD),
                            "SUB" => Ok(Instruction::SUB),
                            "MUL" => Ok(Instruction::MUL),
                            "DIV" => Ok(Instruction::DIV),
                            "CMP" => Ok(Instruction::CMP),
                            "BRC" => Ok(Instruction::BRC),
                            "BEQ" => Ok(Instruction::BEQ),
                            "BGT" => Ok(Instruction::BGT),
                            "BLT" => Ok(Instruction::BLT),
                            "NOP" => Ok(Instruction::NOP),
                            "HLT" => Ok(Instruction::HLT),
                            _     => Err(())
    }
}

fn check_line(line : &InterimLine, line_index : usize) -> Result<(), CodeError> {
    if line.instr.is_none()  {
        if line.op1.is_some() || line.op2.is_some() || line.op3.is_some() {
            Err(CodeError::InstAfterLable(line_index))
        } else {  Ok(()) }
    } else {
        match line.instr.unwrap() {
            Instruction::HLT |
            Instruction::NOP => if line.op1.is_some() || line.op2.is_some() || line.op3.is_some() {
                                    Err(CodeError::TooManyOps(line_index))
                                } else {
                                    Ok(())
                                }
            Instruction::BRC |
            Instruction::BEQ |
            Instruction::BGT |
            Instruction::BLT => {
                match &line.op1 {
                    Some(op) => {
                        match op {
                            InterimOp::Lable(..) =>
                                if  line.op2.is_some() || line.op3.is_some() {
                                    Err(CodeError::TooManyOps(line_index))
                                } else {
                                    Ok(())
                                },
                            _ => Err(CodeError::MissingLable(line_index))
                        }
                    },
                    _ => Err(CodeError::MissingLable(line_index))
                }
            }
            Instruction::CMP => {
                        if line.op1.is_none() || line.op2.is_none() {
                            return Err(CodeError::TooFewOps(line_index));
                        }
                        is_out_register(&line.op1, line_index)?;
                        is_out_register(&line.op2, line_index)?;                
                        if line.op3.is_some() {
                            return Err(CodeError::TooManyOps(line_index));
                        }
                        Ok(())
                    },
            Instruction::ADD |
            Instruction::SUB |
            Instruction::MUL |
            Instruction::DIV => {
                if line.op1.is_none() || line.op2.is_none()  {
                    return Err(CodeError::TooFewOps(line_index));
                }
                is_out_register(&line.op1, line_index)?;
                is_out_register(&line.op2, line_index)?;
                if line.op3.is_some()  {
                    match line.op3.as_ref().unwrap() {
                        InterimOp::Lable(..) |
                        InterimOp::Direct(..) => Err(CodeError::InvalidOp(line_index)),
                        InterimOp::Reg(Register::RI) => Err(CodeError::InvalidOp(line_index)),
                        _ => Ok(()),
                    }
                } else { Err(CodeError::TooFewOps(line_index)) }
            }
        }
    }
}

fn is_out_register(op : &Option<InterimOp>, line_index : usize) -> Result<(), CodeError> {
match op {
                    Some(InterimOp::Reg(Register::RO)) => return Err(CodeError::InvalidOp(line_index)),
                    _ => Ok(()),
                }
}

fn get_lines(program_code : &str) -> Result<Vec<InterimLine>, CodeError> {
    let mut lines = Vec::new();
    let mut line = InterimLine::new();

    for (line_index, l) in program_code.split('\n').enumerate() {
        for w in l.split(" ") {
            if w.len() == 0 {
                 continue;
            }
            let w : Vec<&str> = w.split(";").collect();
            if w.len() != 1 {
                break;
            }
            let w = w.as_slice()[0];
            if w.len() == 0 {
                continue;
            }

            //add to line
            match line.instr {
                //add lable or instruction
                None => line.instr = match get_instruction(w) {
                    Err(_) => {
                        if !w.ends_with(":") {
                            return Err(CodeError::UnknownInst(line_index));
                        } else {
                            if line.lable.is_some() {
                                line.instr = Some(Instruction::NOP);
                                lines.push(line);
                                line = InterimLine::new();   
                            }
                            line.lable = Some(w[0..w.len()-1].to_string());
                            None
                        }
                    }
                    Ok(instr) => Some(instr),
                },
                //add Operand
                _ => {
                    if line.op1.is_none() {
                        line.op1 = Some(get_operand(w, line_index)?);
                    } else if line.op2.is_none() {
                        line.op2 = Some(get_operand(w, line_index)?);
                    } else if line.op3.is_none() {
                        line.op3 = Some(get_operand(w, line_index)?);
                    } else {
                        return Err(CodeError::TooManyOps(line_index));
                    }
                }
            }

        }
        match line.instr {
            Some(_) => {
                check_line(&line, line_index)?;
                lines.push(line);
                line = InterimLine::new();
            },
            None => (),
        }
    }

    Ok(lines)
}

fn to_final_op(op : &Option<InterimOp>, lable_hash : &HashMap<String, u16>) -> Result<Option<Operand>, CodeError> {
    Ok(match op {
        Some(int_op) => match int_op {
            InterimOp::Reg(reg) => Some(Operand::Reg(*reg)),
            InterimOp::Direct(num) => Some(Operand::Direct(*num as i16)),
            InterimOp::Lable(lable) => {
                if lable_hash.contains_key(lable) {
                    Some(Operand::Direct(lable_hash[lable] as i16))
                } else {
                    return Err(CodeError::MissingLable(0));
                }
            }
   
            },
        None => None,
    })
}


fn to_final_lines(lines: Vec<InterimLine>) -> Result<Vec<Line>, CodeError> {
    let mut lable_hash : HashMap<String, u16> = HashMap::new();

    //build label hash map
    for (i, l) in lines.iter().enumerate() {
        if l.lable.is_some() {
            lable_hash.insert(l.lable.as_ref().unwrap().to_string(), i as u16);
        }
    }

    //replace lables with line numbers
    let mut final_lines = Vec::new();

    for l in lines {
        let new_line = Line {
            instr : l.instr.unwrap(),
            op1   : to_final_op(&l.op1, &lable_hash)?,
            op2   : to_final_op(&l.op2, &lable_hash)?,
            op3   : to_final_op(&l.op3, &lable_hash)?,
        };
        final_lines.push(new_line);
    }
    
    Ok(final_lines)
}


pub fn get_program_instructions(text_input : &str) -> Result<Vec<Line>, CodeError> {
    let interim_lines = get_lines(text_input)?;
    to_final_lines(interim_lines)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_line_cmp() {
        let line = InterimLine {
            lable : None,
            instr : Some(Instruction::CMP),
            op1 : Some(InterimOp::Reg(Register::R1)),
            op2 : Some(InterimOp::Direct(65)),
            op3 : None
        };
        assert!(check_line(&line, 0).is_ok());
    }

     #[test]
    fn check_line_add_err() {
        let line = InterimLine {
            lable : None,
            instr : Some(Instruction::ADD),
            op1 : Some(InterimOp::Reg(Register::R1)),
            op2 : Some(InterimOp::Reg(Register::R2)),
            op3 : Some(InterimOp::Direct(1000)),
        };
        assert!(check_line(&line, 0).is_err());
    }

    #[test]
    fn parse_into_lines_test1() {
        let code =
"
ADD #10 #0 R1
ADD #12 #0 R2
ADD R1 R2 R1
CMP R1 R2
BGT end
end:
HLT
";

        let lines = get_lines(code).unwrap();
        println!("lines: {}", lines.len());
        assert!(lines.len() == 6);
        println!("end lable {}", lines[5].lable.as_ref().unwrap());
        assert!(lines[5].lable == Some(String::from("end")));
        assert!(matches!(lines[0].op1.as_ref().unwrap(),InterimOp::Direct(10)));
        assert!(matches!(lines[2].instr.as_ref().unwrap(), Instruction::ADD));
        assert!(matches!(lines[4].instr.as_ref().unwrap(), Instruction::BGT));

        let final_lines = to_final_lines(lines).unwrap();
        assert!(matches!(final_lines[4].op1.as_ref().unwrap(), Operand::Direct(5)));
        assert!(matches!(final_lines[0].op3.as_ref().unwrap(), Operand::Reg(Register::R1)));
    }

    #[test]
    fn parse_into_lines_test_empty_lable() {
        let code =
"
ADD #10 #0 R1
ADD #12 #0 R2
ADD R1 R2 R1
lable1:
lable2:
CMP R1 R2
BGT end
HLT
end:
";

        let lines = get_lines(code);
        assert!(lines.is_ok());
    }
    #[test]
        fn test_io_registers() {
        let code =
"
ADD #10 #0 RO
ADD RI #0 R2
ADD R1 R2 RI
HLT
";

        assert!(get_lines(code).is_err());
        }
    #[test]
        fn test_io_registers2() {
        let code =
"
ADD #10 #0 RO
ADD RI #0 R2
CMP RO RI
HLT
";

        assert!(get_lines(code).is_err());
        }
    #[test]
            fn test_io_registers3() {
        let code =
"
ADD #10 #0 RO
ADD RI #0 R2
CMP RI RI
HLT
";

        assert!(get_lines(code).is_ok());
    }

}
