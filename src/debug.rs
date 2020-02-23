extern crate num_derive;
use num::{FromPrimitive};

pub fn disassemble_chunk(ref ch: &crate::chunk::Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;

    while offset < ch.count {
        offset = disassemble_instruction(ch, offset);
    }

}

pub fn disassemble_instruction(ref ch: &crate::chunk::Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && ch.lines[offset] == ch.lines[offset-1] {
        print!("   | ");
    } else {
        print!("{:04} ", ch.lines[offset])
    }

    let instruction: Option<crate::chunk::Opcode> =
        crate::chunk::Opcode::from_u8(ch.code[offset]);

    match instruction {
        Some(crate::chunk::Opcode::OPRETURN) => simple_instruction("OP_RETURN", offset),
        Some(crate::chunk::Opcode::OPNEGATE) => simple_instruction("OP_NEGATE", offset),
        Some(crate::chunk::Opcode::OPADD) => simple_instruction("OP_ADD", offset),
        Some(crate::chunk::Opcode::OPSUBTRACT) => simple_instruction("OP_SUBTRACT", offset),
        Some(crate::chunk::Opcode::OPMULTIPLY) => simple_instruction("OP_MULTIPLY", offset),
        Some(crate::chunk::Opcode::OPDIVIDE) => simple_instruction("OP_DIVIDE", offset),
        Some(crate::chunk::Opcode::OPCONSTANT) => constant_instruction("OP_CONSTANT", ch, offset),

        _ => simple_instruction("UNKNOWN OPCODE", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{}\n", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &crate::chunk::Chunk, offset: usize) -> usize {
    let constant: usize = chunk.code[offset + 1] as usize;
    print!("{:-16} {:4} ", name, constant);
    crate::value::print_value(chunk.constants.values[constant]);
    println!("");
    offset + 2
}
