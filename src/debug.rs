extern crate num_derive;
use num::{FromPrimitive};

pub fn disassemble_chunk(ref ch: &crate::chunk::Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;

    while offset < ch.code.len() {
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
        Some(crate::chunk::Opcode::OPTRUE) => simple_instruction("OP_TRUE",  offset),
        Some(crate::chunk::Opcode::OPFALSE) => simple_instruction("OP_FALSE",  offset),
        Some(crate::chunk::Opcode::OPNIL) => simple_instruction("OP_NIL",  offset),
        Some(crate::chunk::Opcode::OPNOT) => simple_instruction("OP_NOT",  offset),
        Some(crate::chunk::Opcode::OPEQUAL) => simple_instruction("OP_EQUAL",  offset),
        Some(crate::chunk::Opcode::OPLT) => simple_instruction("OP_LT",  offset),
        Some(crate::chunk::Opcode::OPLTE) => simple_instruction("OP_LTE",  offset),
        Some(crate::chunk::Opcode::OPGT) => simple_instruction("OP_GT",  offset),
        Some(crate::chunk::Opcode::OPGTE) => simple_instruction("OP_GTE",  offset),
        Some(crate::chunk::Opcode::OPLEN) => simple_instruction("OP_LEN",  offset),

        _ => simple_instruction("UNKNOWN OPCODE", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{}\n", name);
    offset + 1
}

fn constant_instruction(name: &str,
                        chunk: &crate::chunk::Chunk,
                        offset: usize) -> usize {
    let constant: usize = chunk.code[offset + 1] as usize;
    print!("{:-16} {:4} ", name, constant);

    // print_value needs a value, not a constant
    let vt = match &chunk.constants.values[constant] {
        crate::value::ConstantType::NUMBER(n) =>
            crate::value::ValueType::NUMBER(*n),
        crate::value::ConstantType::STRING(s) =>
            crate::value::ValueType::STRING(s)
    };

    crate::value::print_value(&vt);
    println!("");
    offset + 2
}
