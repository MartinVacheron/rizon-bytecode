use crate::{chunk::{Chunk, Op}, value::{FnUpValue, Value}};

pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("\t\t== {} ==", name);

    for idx in 0..chunk.code.len() {
        disassemble_instruction(chunk, idx);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset - 1] == chunk.lines[offset] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset] + 1);
    }

    match &chunk.code[offset] {
        Op::Return => println!("OP_RETURN"),
        Op::Constant(i) => constant("OP_CONSTANT", chunk, *i),
        Op::Negate => println!("OP_NEGATE"),
        Op::Add => println!("OP_ADD"),
        Op::Subtract => println!("OP_SUBTRACT"),
        Op::Multiply => println!("OP_MULTIPLY"),
        Op::Divide => println!("OP_DIVIDE"),
        Op::Null => println!("OP_NULL"),
        Op::True => println!("OP_TRUE"),
        Op::False => println!("OP_FALSE"),
        Op::Not => println!("OP_NOT"),
        Op::Equal => println!("OP_EQUAL"),
        Op::Greater => println!("OP_GREATER"),
        Op::Less => println!("OP_LESS"),
        Op::Print => println!("OP_PRINT"),
        Op::Pop => println!("OP_POP"),
        Op::DefineGlobal(i) => constant("OP_DEFINE_GLOBAL", chunk, *i),
        Op::GetGlobal(i) => constant("OP_GET_GLOBAL", chunk, *i),
        Op::SetGlobal(i) => constant("OP_SET_GLOBAL", chunk, *i),
        Op::GetLocal(i) => byte_instruction("OP_GET_LOCAL", *i),
        Op::SetLocal(i) => byte_instruction("OP_SET_LOCAL", *i),
        Op::JumpIfFalse(i) => jump_instruction("JUMP_IF_FALSE", offset as i32, *i),
        Op::Jump(i) => jump_instruction("OP_JUMP", offset as i32, *i),
        Op::Loop(i) => loop_instruction("OP_LOOP", offset as i32, *i),
        Op::CreateIter => println!("OP_CREATE_ITER"),
        Op::ForIter(_, i) => jump_instruction("OP_FOR_ITER", offset as i32, *i),
        Op::Call(i) => byte_instruction("OP_CALL", *i),
        Op::Closure(i) => closure_instruction(offset, chunk, *i),
        Op::GetUpValue(i) => byte_instruction("OP_GET_UPVALUE", *i),
        Op::SetUpValue(i) => byte_instruction("OP_SET_UPVALUE", *i),
        Op::CloseUpValue => println!("OP_CLOSE_VALUE"),
        Op::Struct(i) => constant("OP_STRUCT", chunk, *i),
        Op::GetProperty(i) => constant("OP_GET_PROPERTY", chunk, *i),
        Op::SetProperty(i) => constant("OP_SET_PROPERTY", chunk, *i),
        Op::Method(i) => constant("OP_METHOD", chunk, *i),
    }
}

fn constant(name: &str, chunk: &Chunk, idx: u8) {
    println!("{:16} {} '{}'", name, idx, chunk.constants[idx as usize]);
}

fn byte_instruction(name: &str, slot: u8) {
    println!("{:16} {}", name, slot);
}

fn jump_instruction(name: &str, start: i32, offset: u16) {
    println!("{:16} {} -> {}", name, start, start + offset as i32 + 1);
}

// +1 is ok, it's because the Op::Loop is taken into account
fn loop_instruction(name: &str, start: i32, offset: u16) {
    println!("{:16} {} -> {}", name, start, start + 1 + offset as i32 * -1);
}

fn closure_instruction(offset: usize, chunk: &Chunk, idx: u8) {
    let closure = &chunk.constants[idx as usize];
    println!("{:16} {} '{}'", "OP_CLOSURE", idx, closure);

    if let Value::ClosureFn(c) = closure {
        for (i, FnUpValue { index, is_local }) in c.function.upvalues.iter().enumerate() {
            let local = if *is_local { "local" } else { "upvalue" };
            println!("{:04}      |                     {} {}", offset + i, local, index);
        }
    }
}