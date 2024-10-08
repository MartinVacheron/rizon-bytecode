// use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::{gc::GcRef, value::Value};

// #[derive(IntoPrimitive, TryFromPrimitive)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Op {
    Return,
    Constant(u8),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Null,
    True,
    False,
    Not,
    Equal, // TODO: implement the three others: https://craftinginterpreters.com/types-of-values.html#equality-and-comparison-operators
    Greater,
    Less,
    Print,
    Pop,
    DefineGlobal(u8),
    GetGlobal(u8),
    SetGlobal(u8),
    GetLocal(u8),
    SetLocal(u8),
    JumpIfFalse(u16),
    Jump(u16),
    Loop(u16),
    CreateIter,
    ForIter(u8, u16), // Local index of the iterator variable as first arg
    Call(u8),
    Closure(u8),
    GetUpValue(u8),
    SetUpValue(u8),
    CloseUpValue,
    Struct(u8),
    GetProperty(u8),
    SetProperty(u8),
    Method(u8),
    Invoke((u8, u8)),
}

#[derive(Default, Clone, Debug)]
pub struct Chunk {
    pub code: Vec<Op>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn write(&mut self, byte: Op, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);

        self.constants.len() - 1
    }

    pub fn read_constant(&self, index: u8) -> Value {
        self.constants[index as usize]
    }

    pub fn read_string(&self, index: u8) -> GcRef<String> {
        if let Value::Str(string) = self.read_constant(index) {
            string
        } else {
            panic!("constant is not a string")
        }
    }
}
