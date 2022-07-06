use crate::assembler::{Instruction, Line, Register, CodeError, get_program_instructions, Operand};

pub struct Program {
    code : Vec<Line>,
    pc : i16,
    r1 : i16,
    r2 : i16,
    rt : i16,
    rj : i16,
    halted : bool,
}

impl Program {
    pub fn new(program_code : &str) -> Result<Self, CodeError> {
        Ok(Program {
            code : get_program_instructions(program_code)?,
            pc : 0,
            r1 : 0,
            r2 : 0,
            rt : 0,
            rj : 0,
            halted : false,
        })
    }

    fn step(&mut self) {
        if self.halted { return }
        
        let current_line = &self.code[self.pc as usize];
        self.pc += 1;

        match current_line.instr {
            Instruction::ADD |
            Instruction::SUB |
            Instruction::MUL |
            Instruction::DIV => {
                let op1 = self.get_operand_value(current_line.op1.unwrap());
                let op2 = self.get_operand_value(current_line.op2.unwrap());
           
                let result = math_instruction(current_line.instr, op1, op2);
                match current_line.op3.unwrap() {
                    Operand::Reg(reg) => self.set_register_value(reg, result),
                    _ => panic!("shouldn't be able to assign to non-register"),
            }},
            Instruction::CMP => (),
            _ => ()
       } 
    }

    fn get_operand_value(&self, op : Operand) -> i16 {
        match op {
            Operand::Direct(num) => num,
            Operand::Reg(reg) => self.get_register_value(reg),
        }
    }

    fn get_register_value(&self, reg : Register) -> i16 {
        match reg {
            Register::PC => self.pc,
            Register::R1 => self.r1,
            Register::R2 => self.r2,
            Register::RT => self.rt,
            Register::RJ => self.rj,
        }
    }

    fn set_register_value(&mut self, reg : Register, value : i16) {
        match reg {
            Register::PC => self.pc = value,
            Register::R1 => self.r1 = value,
            Register::R2 => self.r2 = value,
            Register::RT => self.rt = value,
            Register::RJ => self.rj = value,
        }
    }
        
}

fn math_instruction(instr : Instruction, op1 : i16, op2 : i16) -> i16 {
    match instr {
        Instruction::ADD => op1 + op2,
        Instruction::SUB => op1 - op2,
        Instruction::MUL => op1 * op2,
        Instruction::DIV => op1 / op2,
        _ => panic!("only acccepts math instructions!"),
    }
}
