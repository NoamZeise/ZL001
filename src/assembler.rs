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

enum Register {
    R1,
    R2,
    RT,
    RJ,
}

enum InterimOp {
    Reg(Register),
    Direct(u16),
    Lable(String),
}

enum Operand {
    Reg(Register),
    Direct(u16)
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

struct Line {
    instr : Instruction,
    op1   : Option<InterimOp>,
    op2   : Option<InterimOp>,
    op3   : Option<InterimOp>,
}

pub struct Program {
    code : Vec<InterimLine>,
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


fn get_operand(word : &str, line_index : usize) -> Result<InterimOp, CodeError> {
    Ok(
        match word.to_uppercase().as_str() {
            "R1" => InterimOp::Reg(Register::R1),
            "R2" => InterimOp::Reg(Register::R2),
            "RT" => InterimOp::Reg(Register::RT),
            "RJ" => InterimOp::Reg(Register::RJ),
            _ => {
                if word.starts_with("#") {
                    match word.split_at(1).1.parse::<u16>() {
                         Ok(n) => InterimOp::Direct(n),
                         _ => { return Err(CodeError::UnknownNumber(line_index as u16)); }
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
                            "BGT" => Ok(Instruction::BGT),
                            "BLT" => Ok(Instruction::BLT),
                            "HLT" => Ok(Instruction::HLT),
                            _     => Err(())
    }
}

fn check_line(line : &InterimLine, line_index : u16) -> Result<(), CodeError> {
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
                if line.op3.is_some()  {
                    match line.op3.as_ref().unwrap() {
                        InterimOp::Lable(..) |
                        InterimOp::Direct(..) => Err(CodeError::InvalidOp(line_index)),
                        _ => Ok(()),
                    }
                } else { Err(CodeError::TooFewOps(line_index)) }
            }
        }
    }
}

fn get_lines(program_code : &str) -> Result<Vec<InterimLine>, CodeError> {
    let mut lines = Vec::new();
    let mut line = InterimLine::new();
    for (line_index, l) in program_code.split('\n').enumerate() {
        for (i, w) in l.split(" ").enumerate() {
            if w.len() == 0 {
                 continue;
            }
            if w.starts_with(";") {
                break;
            }
        }
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
