use std::io;
use crate::bytecode::OpCode;

// This is a stack-based virtual machine. It is intended to be used to
// execute bytecodes that produced by the compiler.
//
// One thing that worth noting about the function calling convention:
// Every time the machine execute the CALL opcode, it will push 3 values
// to the stack:
// - the Frame Pointer: to keep track of the current call frame
// - the Return Address: so we know where to jump back on return
// - the Argument Count: so we can clean up the arguments in the stack
// after return.
//

pub struct VirtualMachine {
    program: Vec<i32>,
    ip: usize,
    sp: usize,
    fp: usize,
    stack: Vec<i32>,
    globals: Vec<i32>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            ip: 0,
            sp: 0,
            fp: 0,
            stack: vec![0; 1024],
            globals: vec![0; 1024],
            program: vec![],
        }
    }

    pub fn load_program(&mut self, program: Vec<i32>, entrypoint: usize) {
        self.program = program;
        self.ip = entrypoint;
    }

    pub fn pop_stack(&mut self) -> i32 {
        self.sp -= 1;
        return self.stack[self.sp];
    }

    pub fn push_stack(&mut self, val: i32) {
        self.stack[self.sp] = val;
        self.sp += 1;
    }

    pub fn next_operand(&mut self) -> i32 {
        self.ip += 1;
        return self.program[self.ip];
    }

    pub fn run(&mut self, stdout: &mut dyn io::Write) {
        loop {
            let opcode = OpCode::from(self.program[self.ip]);
            match opcode {
                OpCode::HALT => {
                    if let Err(_) = writeln!(stdout, "BYE!") {
                        println!("ERROR: Could not write to output device!");
                    }
                    break;
                },
                OpCode::PUSH => {
                    let val = self.next_operand();
                    self.push_stack(val);
                },
                OpCode::ADD => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    self.push_stack(a + b);
                },
                OpCode::SUB => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    self.push_stack(a - b);
                },
                OpCode::MUL => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    self.push_stack(a * b);
                },
                OpCode::DIV => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    self.push_stack(a / b);
                },
                OpCode::PRINT => {
                    let val = self.pop_stack();
                    if let Err(_) = writeln!(stdout, "{}", val) {
                        println!("ERROR: Could not write to output device!");
                    }
                },
                OpCode::GSTORE => {
                    let addr = self.next_operand();
                    let val = self.pop_stack();
                    self.globals[addr as usize] = val;
                },
                OpCode::GLOAD => {
                    let addr = self.next_operand();
                    self.push_stack(self.globals[addr as usize]);
                },
                OpCode::LLOAD => {
                    let addr = self.next_operand();
                    self.push_stack(self.stack[(self.fp as i32 + addr) as usize]);
                },
                OpCode::LSTORE => {
                    let addr = self.next_operand();
                    let val = self.pop_stack();
                    self.stack[(self.fp as i32 + addr) as usize] = val;
                },
                OpCode::CALL => {
                    let fn_addr = self.next_operand() as usize;
                    let fn_argc = self.next_operand();
                    let ret_addr = self.ip + 1;
                    // When CALL, push 3 values to the stack:
                    // - The current frame pointer
                    self.push_stack(self.fp as i32);
                    // - The return address
                    self.push_stack(ret_addr as i32);
                    // - The number of args
                    self.push_stack(fn_argc);
                    // make a jump
                    self.fp = self.sp;
                    self.ip = fn_addr;
                    continue;
                },
                OpCode::RET => {
                    let ret_val = self.pop_stack();
                    self.sp = self.fp;
                    let fn_argc = self.pop_stack();
                    let ret_addr = self.pop_stack() as usize;
                    let prev_fp = self.pop_stack() as usize;
                    self.fp = prev_fp;
                    self.sp -= fn_argc as usize;
                    self.push_stack(ret_val);
                    self.ip = ret_addr;
                    continue;
                },
                OpCode::POP => {
                    self.pop_stack();
                }
                OpCode::EQ => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    let ret = if a == b { 1 } else { 0 };
                    self.push_stack(ret);
                },
                OpCode::NE => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    let ret = if a != b { 1 } else { 0 };
                    self.push_stack(ret);
                },
                OpCode::GT => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    let ret = if a > b { 1 } else { 0 };
                    self.push_stack(ret);
                },
                OpCode::LT => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    let ret = if a < b { 1 } else { 0 };
                    self.push_stack(ret);
                },
                OpCode::GE => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    let ret = if a >= b { 1 } else { 0 };
                    self.push_stack(ret);
                },
                OpCode::LE => {
                    let a = self.pop_stack();
                    let b = self.pop_stack();
                    let ret = if a <= b { 1 } else { 0 };
                    self.push_stack(ret);
                },
                OpCode::JMP => {
                    self.ip = self.next_operand() as usize;
                    continue;
                },
                OpCode::JMP0 => {
                    let addr = self.next_operand() as usize;
                    let val = self.pop_stack();
                    if val == 0 {
                        self.ip = addr;
                        continue;
                    }
                },
                OpCode::JMP1 => {
                    let addr = self.next_operand() as usize;
                    let val = self.pop_stack();
                    if val == 1 {
                        self.ip = addr;
                        continue;
                    }
                },
            }
            self.ip += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::{OpCode, FUNC_PARAM_OFFSET};
    use super::VirtualMachine;

    #[test]
    fn test_simple_program() {
        let mut stdout = vec![];
        let program = vec![
	        // print(10 + 5)
            OpCode::PUSH as i32, 10,            // 000
            OpCode::PUSH as i32, 5,             // 002
            OpCode::ADD as i32,                 // 003
            OpCode::PRINT as i32,               // 004
            OpCode::HALT as i32,                // 005
        ];
        let mut vm = VirtualMachine::new();
        vm.load_program(program, 0);
        vm.run(&mut stdout);

        let stdout_str = std::str::from_utf8(&stdout).unwrap();
        assert_eq!(stdout_str, "15\nBYE!\n");
    }

    #[test]
    fn test_simple_if_program() {
        let mut stdout = vec![];
        let program = vec![
            // if 10 >= 5 then
            OpCode::PUSH as i32, 10,            // 000
            OpCode::PUSH as i32, 5,             // 002
            OpCode::GE as i32,                  // 004
            // jump to line 11 if true
            OpCode::JMP1 as i32, 11,            // 005
            // false branch, print 0 and halt
            OpCode::PUSH as i32, 0,             // 007
            OpCode::PRINT as i32,               // 009
            OpCode::HALT as i32,                // 010
            // true branch, print 1 and jump back
            OpCode::PUSH as i32, 1,             // 011
            OpCode::PRINT as i32,               // 013
            // jump back to halt line
            OpCode::JMP as i32, 10              // 014
        ];
        let mut vm = VirtualMachine::new();
        vm.load_program(program, 0);
        vm.run(&mut stdout);

        let stdout_str = std::str::from_utf8(&stdout).unwrap();
        assert_eq!(stdout_str, "0\nBYE!\n");
    }

    #[test]
    fn test_simple_program_with_function() {
        let mut stdout = vec![];
        let program = vec![
            // fn calc(a, b) -> (a + b) * 2
            OpCode::LLOAD as i32, -(FUNC_PARAM_OFFSET + 2),   // 000
            OpCode::LLOAD as i32, -(FUNC_PARAM_OFFSET + 1),   // 002
            OpCode::ADD as i32,                                       // 004
            OpCode::PUSH as i32, 2,                                   // 005
            OpCode::MUL as i32,                                       // 007
            OpCode::RET as i32,                                       // 008
            // main func - entry point                                //
            // let $v0 = 19                                           //
            OpCode::PUSH as i32, 19,                                  // 009
            OpCode::GSTORE as i32, 0,                                 // 011
            // let $v1 = 8                                            //
            OpCode::PUSH as i32, 8,                                   // 013
            OpCode::GSTORE as i32, 1,                                 // 015
            // call 0, 2   ;; 0 = address of calc, 2 = 2 params       //
            OpCode::GLOAD as i32, 0,                                  // 017
            OpCode::GLOAD as i32, 1,                                  // 019
            OpCode::CALL as i32, 0, 2,                                // 021
            OpCode::PRINT as i32,                                     // 024
            OpCode::HALT as i32,                                      // 025
        ];
        let mut vm = VirtualMachine::new();
        vm.load_program(program, 9);
        vm.run(&mut stdout);

        let stdout_str = std::str::from_utf8(&stdout).unwrap();
        assert_eq!(stdout_str, "54\nBYE!\n");
    }
}
