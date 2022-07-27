//! uses `assembler` to get a list of `Line`s to execute, that changes the status of register values 

use crate::microcontroller::assembler::*;

const TEST_EQUAL : i16 = 0b001;
const TEST_LESS_THAN : i16 = 0b010;
const TEST_GREATER_THAN : i16 = 0b100;

#[derive(Copy, Clone)]
struct ProgramLineState {
    pub op1 : Option<i16>,
    pub op2 : Option<i16>,
}

impl ProgramLineState {
    fn new(op1 : Option<i16>, op2 : Option<i16>) -> Self {
        ProgramLineState { op1, op2 }
    }
}

/// Simulates a fake assembly language program, made up of lines of instructions
pub struct Program {
    code : Vec<Line>,
    pc : i16,
    r1 : i16,
    r2 : i16,
    rt : i16,
    rio : [i16 ; IO_REGISTER_COUNT],
    active_io_reg : usize,
    out_to_read : bool,
    in_to_read : bool,
    temp_state : Option<ProgramLineState>,
    halted : bool,
    //last_line : usize,
}

impl Program {
    /// make a program from source code, returns a code error and the line where the error occured if there is a syntax issue
    pub fn new(program_code : &str) -> Result<Self, CodeError> {
        Ok(Program {
            code : get_program_instructions(program_code)?,
            pc : 0,
            r1 : 0,
            r2 : 0,
            rt : 0,
            active_io_reg : 0,
            out_to_read : false,
            rio : [0 ; IO_REGISTER_COUNT],
            in_to_read : false,
            temp_state : None,
            halted : false,
            //last_line : 0,
        })
    }
/// return a blank `Program` that is halted
    pub fn blank() -> Self {
        Program {
            code: vec![Line {  instr: Instruction::HLT, op1 : None, op2: None, op3: None}],
            pc: 0, r1: 0, r2: 0, rt: 0, active_io_reg : 0, out_to_read : false,
            in_to_read : false, rio : [0 ; IO_REGISTER_COUNT], temp_state : None,  halted: true,
        }
    }
    /// get io register count
    pub fn io_reg_count(&self) -> usize {
        self.rio.len()
    }
/// return an `Option<i16>` that is `None` if these is no value to read out from the indicated IO register
    pub fn read_out(&mut self, index : usize) -> Option<i16> {
        assert!(index < IO_REGISTER_COUNT, "io register out of range!");
        if self.out_to_read && self.active_io_reg == index {
            self.out_to_read = false;
            self.active_io_reg = IO_REGISTER_COUNT;
            Some(self.rio[index])
        } else {
            None
        }
    }
/// return `true` if there is a value to be read out from IO register
    pub fn read_out_ready(&self, index : usize) -> bool {
        self.out_to_read && self.active_io_reg == index
    }
/// read in a value to an IO register at that index, if not accepting return `Err`
    pub fn read_in(&mut self, value : i16, index : usize) -> Result<(), ()> {
        assert!(index < IO_REGISTER_COUNT, "io register out of range!");
        if self.in_to_read  && self.active_io_reg == index {
            self.rio[index] = value;
            self.in_to_read = false;
            self.active_io_reg = IO_REGISTER_COUNT;
            Ok(())
        } else {
            Err(())
        }
    }
/// return `true` if a value can be read in to an IO register
    pub fn read_in_ready(&self, index : usize) -> bool {
        self.in_to_read && self.active_io_reg == index
    }
/// step the `Program` forward by one `Line`
    pub fn step(&mut self) {
        if self.halted || self.out_to_read || self.in_to_read { return }
        if self.pc as usize >= self.code.len() {
            self.halted = true;
            return;
        }
        
        let current_line = self.code[self.pc as usize];
        self.pc += 1;

        match current_line.instr {
            Instruction::ADD |
            Instruction::SUB |
            Instruction::MUL |
            Instruction::DIV => {
                let (op1, op2) = match self.get_two_ops(current_line) {
                    Ok(v) => v,
                    Err(_) => {
                        return;
                    }
                };

                self.temp_state = None;
                let op1 = op1.unwrap();
                let op2 = op2.unwrap();
           
                let result = math_instruction(current_line.instr, op1, op2);
                
                match current_line.op3.unwrap() {
                    Operand::Reg(reg) => self.set_register_value(reg, result),
                    _ => panic!("shouldn't be able to assign to non-register"),
            }},
            Instruction::CMP => {
                self.rt = 0;
                 let (op1, op2) = match self.get_two_ops(current_line) {
                    Ok(v) => v,
                    Err(_) => {
                        return;
                    }
                 };

                self.temp_state = None;
                let op1 = op1.unwrap();
                let op2 = op2.unwrap();

                if op1 == op2 { self.rt |= TEST_EQUAL; }
                if op1 >  op2 { self.rt |= TEST_GREATER_THAN; }
                if op1 <  op2 { self.rt |= TEST_LESS_THAN; }
            },
            Instruction::BRC => {
                self.pc = self.get_operand_value(current_line.op1.unwrap()).unwrap();
            }
            Instruction::BEQ => {
                if (self.rt & TEST_EQUAL) != 0 {
                    self.pc = self.get_operand_value(current_line.op1.unwrap()).unwrap();
                }
            }
            Instruction::BGT => {
                if (self.rt & TEST_GREATER_THAN) != 0 {
                    self.pc = self.get_operand_value(current_line.op1.unwrap()).unwrap();
                }
            }
            Instruction::BLT => {
                if (self.rt & TEST_LESS_THAN) != 0 {
                    self.pc = self.get_operand_value(current_line.op1.unwrap()).unwrap();
                }
            }
            Instruction::HLT => {
                self.halted = true;
            },
            Instruction::NOP => (),
       } 
    }

    fn get_operand_value(&self, op : Operand) -> Option<i16> {
        match op {
            Operand::Direct(num) => Some(num),
            Operand::Reg(reg) => self.get_register_value(reg),
        }
    }

    fn get_two_ops(&mut self, current_line : Line) -> Result<(Option<i16>, Option<i16>), ()> {
               let mut op1 =  match self.temp_state {
                    None => self.get_operand_value(current_line.op1.unwrap()),
                    Some(s) => s.op1,
               };
                let mut op2 =  match self.temp_state {
                    None => self.get_operand_value(current_line.op2.unwrap()),
                    Some(s) => s.op2,
                };

                if op1.is_none() || op2.is_none() {
                    if self.temp_state.is_some() {
                        if op1.is_none() {
                            
                            op1 = Some(self.rio[get_io_index(current_line.op1.unwrap())]);
                        } else {
                            op2 = Some(self.rio[get_io_index(current_line.op2.unwrap())]);
                        }
                    }
                    
                    if op1.is_none() || op2.is_none() {
                        if op1.is_none() {
                            self.active_io_reg = get_io_index(current_line.op1.unwrap());
                        } else {
                             self.active_io_reg = get_io_index(current_line.op2.unwrap());
                        }
                        self.in_to_read = true;
                        self.pc -= 1;
                        self.temp_state = Some(ProgramLineState::new(op1, op2));
                        return Err(())
                    };
                }
        Ok((op1, op2))
    }
/// returns `None` if an IO register is requested
    pub fn get_register_value(&self, reg : Register) -> Option<i16> {
        match reg {
            Register::PC => Some(self.pc),
            Register::R1 => Some(self.r1),
            Register::R2 => Some(self.r2),
            Register::RT => Some(self.rt),
            Register::RIO(_) => None,
        }
    }

    fn set_register_value(&mut self, reg : Register, value : i16) {
        match reg {
            Register::PC => self.pc = value,
            Register::R1 => self.r1 = value,
            Register::R2 => self.r2 = value,
            Register::RT => self.rt = value,
            Register::RIO(io_index) => {
                self.out_to_read = true;
                self.active_io_reg = io_index;
                self.rio[io_index] = value
            },
        }
    }
/// check if the program has been halted
    pub fn halted(&self) -> bool {
        self.halted
    }

   // pub fn get_last_line(&self) -> Line {
   //     self.code[self.last_line].clone()
   // }
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

fn get_io_index(op : Operand) -> usize {
    match op {
        Operand::Reg(Register::RIO(index)) => index,
        _ => panic!("tried to get index of non io register!")
    }
}
