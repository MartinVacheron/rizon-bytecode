use std::collections::HashMap;

use thiserror::Error;

use crate::chunk::{Chunk, Op};
use crate::compiler::Compiler;
use crate::debug::disassemble_instruction;
use crate::value::Value;

#[derive(Default)]
pub struct VmFlags {
    pub disassemble_compiled: bool,
    pub disassemble_instructions: bool,
    pub print_stack: bool,
}

#[derive(Error, Debug)]
pub enum VmErr {
    #[error("")]
    Compile,

    #[error("")]
    Runtime,
}

pub type VmRes = Result<Value, VmErr>;

#[derive(Default)]
pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    flags: VmFlags,
    globals: HashMap<String, Value>,
}

impl Vm {
    pub fn new(flags: VmFlags) -> Self {
        Self {
            flags,
            ..Default::default()
        }
    }

    pub fn interpret(&mut self, code: &str) -> VmRes {
        let mut compiler = Compiler::new(code, &mut self.chunk);

        compiler.compile(self.flags.disassemble_compiled);

        self.run()
    }

    fn run(&mut self) -> VmRes {
        loop {
            if self.flags.disassemble_instructions {
                if self.flags.print_stack {
                    print!("          ");
                    self.stack.iter().for_each(|v| print!("[ {} ] ", v));
                    println!();
                }

                disassemble_instruction(&self.chunk, self.ip);
            }

            let op = self.eat().clone();

            match op {
                Op::Return => return Ok(Value::Int(0)),
                Op::Constant(idx) => {
                    let val = self.chunk.constants[idx as usize].clone();
                    self.push(val);
                }
                Op::Negate => {
                    if let Err(e) = self.peek_mut(0).negate() {
                        self.runtime_err(&e.to_string())
                    }
                }
                Op::Add => self.binop(|a, b| a.add(b)),
                Op::Subtract => self.binop(|a, b| a.sub(b)),
                Op::Multiply => self.binop(|a, b| a.mul(b)),
                Op::Divide => self.binop(|a, b| a.div(b)),
                Op::True => self.push(Value::Bool(true)),
                Op::False => self.push(Value::Bool(false)),
                Op::Null => self.push(Value::Null),
                Op::Not => {
                    if let Err(e) = self.peek_mut(0).not() {
                        self.runtime_err(&e.to_string())
                    }
                }
                Op::Equal => self.binop(|a, b| a.eq(b)),
                Op::Greater => self.binop(|a, b| a.gt(b)),
                Op::Less => self.binop(|a, b| a.lt(b)),
                Op::Print => println!("{}", self.pop()),
                Op::Pop => {
                    self.pop();
                }
                Op::DefineGlobal(idx) => match &self.chunk.constants[idx as usize] {
                    Value::Str(s) => {
                        let name = *s.clone();
                        let value = self.pop();
                        self.globals.insert(name, value);
                    }
                    _ => panic!("Internal error, using non-string operand to OP_DEFINE_GLOBAL"),
                },
                Op::GetGlobal(idx) => match &self.chunk.constants[idx as usize] {
                    Value::Str(s) => match self.globals.get(s.as_ref()) {
                        Some(glob) => self.push(glob.clone()),
                        None => {
                            self.runtime_err(&format!("Undefined variable '{}'", s));
                            return Err(VmErr::Runtime);
                        }
                    },
                    _ => panic!("Internal error, using non-string operand to OP_DEFINE_GLOBAL"),
                },
                Op::SetGlobal(idx) => match &self.chunk.constants[idx as usize] {
                    Value::Str(s) => {
                        let name = *s.clone();

                        if self.globals.insert(name, self.peek(0).clone()).is_none() {
                            self.runtime_err(&format!("Undefined variable '{}'", s));
                            return Err(VmErr::Runtime);
                        }
                    }
                    _ => panic!("Internal error, using non-string operand to OP_DEFINE_GLOBAL"),
                },
                Op::GetLocal(idx) => {
                    self.push(self.stack[idx as usize].clone());
                }
                Op::SetLocal(idx) => {
                    self.stack[idx as usize] = self.peek(0).clone();
                },
                Op::JumpIfFalse(idx) => {
                    if let Value::Bool(b) = self.peek(0) {
                        if !b {
                            self.ip += idx as usize;
                        }
                    }
                },
                Op::Jump(idx) => self.ip += idx as usize,
                Op::Loop(idx) => self.ip -= idx as usize,
                Op::CreateIter => {
                    if let Value::Int(i) = self.pop() {
                        // The placeholder value (same as local idx)
                        self.push(Value::Int(0));
                        self.push(Value::Iter(0..i));
                    } else {
                        self.runtime_err("Range must be an integer");
                        return Err(VmErr::Runtime);
                    }
                },
                Op::ForIter(idx) => {
                    if let Value::Iter(iter) = self.peek_mut(0) {
                        match iter.next() {
                            Some(v) => {
                                if let Value::Int(i) = self.peek_mut(1) {
                                    *i = v;
                                }
                            },
                            None => {
                                self.pop();
                                self.ip += idx as usize
                            },
                        }
                    }
                },
            }
        }
    }

    fn binop(&mut self, operation: fn(Value, Value) -> Option<Value>) {
        // Pop backward as it is a stack
        let (rhs, lhs) = (self.pop(), self.pop());

        match operation(lhs, rhs) {
            Some(res) => self.push(res),
            None => self.runtime_err("Operation not allowed")
        }
    }

    fn eat(&mut self) -> &Op {
        let op = &self.chunk.code[self.ip];
        self.ip += 1;

        op
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - distance - 1]
    }

    fn peek_mut(&mut self, distance: usize) -> &mut Value {
        let idx = self.stack.len() - distance - 1;
        &mut self.stack[idx]
    }

    fn runtime_err(&self, msg: &str) {
        eprintln!(
            "[line {}] Error: {}",
            self.chunk.lines[self.chunk.code.len() - 1],
            msg
        );
    }
}
