use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Instruction {
    ADD,
    SUB,
    MUL,
    DIV,
    CMP,
    BRC,
    BGT,
    BLT,
    HLT,
}

enum Operand {
    R1,
    R2,
    RT,
    RJ,
    Direct(u16),
    Lable,
}

#[derive(Debug)]
pub enum CodeError {
    TooManyOps(u16),
    UnknownOp(u16),
    UnknownInst(u16),
    MissingLable(u16),
    MisformedLable(u16),
    UnknownNumber(u16),
    TooManySpaces(u16),
    JumpNeedsLable(u16),
    InstAfterLable(u16),
    TooFewOps(u16),
    InvalidOp(u16),
    OutOfInstructions,
}

struct Line {
    lable : Option<String>,
    instr : Option<Instruction>,
    op1   : Option<Operand>,
    op2   : Option<Operand>,
    op3   : Option<Operand>,
}

impl Line {
    pub fn new() -> Self {
        Line {
            lable : None,
            instr : None,
            op1 : None,
            op2 : None,
            op3 : None,
        }
    }
}

pub struct Program {
    code : Vec<Line>,
    lables : HashMap<String, u16>,

    r1 : u16,
    r2 : u16,
    rt : u16,
    rj : u16,
}

impl Program {
    pub fn new(program_code : &str) -> Result<Self, CodeError> {
        let code = get_lines(program_code)?;
        Ok(Program {
            code,
            lables : HashMap::new(),

            r1 : 0,
            r2 : 0,
            rt : 0,
            rj : 0,
        })
    }
}


fn get_operand(word : &str, line_index : usize) -> Result<Operand, CodeError> {
    Ok(
        match word.to_uppercase().as_str() {
            "R1" => Operand::R1,
            "R2" => Operand::R2,
            "RT" => Operand::RT,
            "RJ" => Operand::RJ,
            _ => {
                if word.starts_with("#") {
                    match word.split_at(1).1.parse::<u16>() {
                         Ok(n) => Operand::Direct(n),
                         _ => { return Err(CodeError::UnknownNumber(line_index as u16)); }
                    }
                } else { Operand::Lable }
            }
        }
    )
}

fn check_line(line : &Line, line_index : u16) -> Result<(), CodeError> {
    if line.instr.is_none()  {
        if line.op1.is_some() || line.op2.is_some() || line.op3.is_some() {
            Err(CodeError::InstAfterLable(line_index))
        } else {  Ok(()) }
    } else {
        match line.instr.unwrap() {
            Instruction::HLT => if line.op1.is_some() || line.op2.is_some() || line.op3.is_some() {
                                    Err(CodeError::TooManyOps(line_index))
                                } else {
                                    Ok(())
                                }
            Instruction::BRC |
            Instruction::BGT |
            Instruction::BLT => {
                match line.op1 {
                    Some(op) => {
                        match op {
                            Operand::Lable =>
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
            Instruction::CMP => match line.op1 {
                Some(s) =>  match s {
                    Operand::Direct(..) |
                    Operand::Lable => Err(CodeError::InvalidOp(line_index)),
                    _ => {
                        match line.op2 {
                            Some(s) =>  match s {
                                Operand::Direct(..) |
                                Operand::Lable => Err(CodeError::InvalidOp(line_index)),
                                _ => if line.op3.is_none() { Err(CodeError::TooManyOps(line_index)) } else { Ok(()) }
                                },
                            _ => Err(CodeError::TooFewOps(line_index)),
                        }
                    }
                }
                _ => Err(CodeError::TooFewOps(line_index)),
            }
            Instruction::ADD |
            Instruction::SUB |
            Instruction::MUL |
            Instruction::DIV => {
                if line.op1.is_some()  {
                } else { return Err(CodeError::TooFewOps(line_index)); }
                if line.op2.is_some()  {

                } else { return Err(CodeError::TooFewOps(line_index)); }
                if line.op3.is_some()  {
                    match line.op3.as_ref().unwrap() {
                        Operand::Lable |
                        Operand::Direct(..) => Err(CodeError::InvalidOp(line_index)),
                        _ => Ok(()),
                    }
                } else { Err(CodeError::TooFewOps(line_index)) }
            }
        }
    }
}

fn get_lines(program_code : &str) -> Result<Vec<Line>, CodeError> {
    let mut lines = Vec::new();
    for (line_index, l) in program_code.split('\n').enumerate() {
        if line_index > u16::max as usize {
            return Err(CodeError::OutOfInstructions);
        }
        let mut line = Line::new();
        for (i, w) in l.split(" ").enumerate() {
            if w.len() == 0 {
                 return Err(CodeError::TooManySpaces(line_index as u16));
            }
            if w.starts_with(";") {
                break;
            }
             match i {
                0 => {
                    if w.ends_with(":") {
                          if w.len() > 1 {
                            line.lable = Some(w[0..w.len()-1].to_string());
                            println!("added lable {}", line.lable.unwrap());
                          } else {
                             return Err(CodeError::MisformedLable(line_index as u16));
                        }
                    } else {
                          line.instr = Some( match w.to_uppercase().as_str() {
                            "ADD" => Instruction::ADD,
                            "SUB" => Instruction::SUB,
                            "MUL" => Instruction::MUL,
                            "DIV" => Instruction::DIV,
                            "CMP" => Instruction::CMP,
                            "BRC" => Instruction::BRC,
                            "BGT" => Instruction::BGT,
                            "BLT" => Instruction::BLT,
                            "HLT" => Instruction::HLT,
                            _ => {
                                return Err(CodeError::UnknownInst(line_index as u16));
                            }
                        });
                    }
                },
                1 => {
                    line.op1 = Some(get_operand(w, line_index)?);
                    if matches!(line.op1.unwrap(),Operand::Lable)  {
                        line.lable = Some(w.to_string());
                    }
                }
                2 => {
                    line.op2 = Some(get_operand(w, line_index)?);
                    if matches!(line.op2.unwrap(),Operand::Lable)  {
                        line.lable = Some(w.to_string());
                    }
                }
                3 => {
                    line.op3 = Some(get_operand(w, line_index)?);
                    if matches!(line.op3.unwrap(),Operand::Lable)  {
                        line.lable = Some(w.to_string());
                    }
                }
                _ => {
                        return Err(CodeError::TooManyOps(line_index as u16));
                 }
            }
        }
        check_line(&line, line_index as u16)?;
        lines.push(line);
    }

    Ok(lines)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_into_lines_test1() {
        let code = "ADD #10 #0 R1\nADD #12 #0 R2\nADD R1 R2 R1\nCMP R1 R2\nBGT end\nend:\nHLT";

        let lines = get_lines(code).unwrap();

        assert!(lines.len() == 7);
    }

}
