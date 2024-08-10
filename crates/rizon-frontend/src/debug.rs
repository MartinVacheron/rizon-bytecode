use crate::chunk::{Chunk, Op};

pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    for idx in 0..chunk.code.len() {
        disassemble_instruction(chunk, idx);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset - 1] == chunk.lines[offset] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    match &chunk.code[offset] {
        Op::Return => println!("OP_RETURN"),
        Op::Constant(i) => disassemble_constant(chunk, *i as usize),
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
    }
}

fn disassemble_constant(chunk: &Chunk, idx: usize) {
    println!("{:16} {} '{}'", "OP_CONSTANT", idx, chunk.constants[idx]);
}
