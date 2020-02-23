#[derive(FromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    OPCONSTANT,
    OPNEGATE,
    OPADD,
    OPSUBTRACT,
    OPMULTIPLY,
    OPDIVIDE,
    OPRETURN,
}

pub struct Chunk {
    pub capacity: usize,
    pub count: usize,
    pub code: Vec<u8>,
    pub lines: Vec<u16>,
    pub constants: crate::value::Values,
}

pub fn init_chunk() -> Chunk {
    Chunk {
        capacity: 8,
        count: 0,
        code: vec![0; 8],
        lines: vec![0; 8],
        constants: crate::value::init_values(),
    }
}

pub fn write_chunk(chunk: &mut Chunk, byte: u8, line: u16)  {
    if chunk.capacity < chunk.count + 1 {
        chunk.capacity = grow_capacity!(chunk.capacity);
        grow_array!(chunk.code, chunk.capacity, 0);
        grow_array!(chunk.lines, chunk.capacity, 0);
    }

    chunk.code[chunk.count] = byte as u8;
    chunk.lines[chunk.count] = line;
    chunk.count += 1;
}

pub fn add_constant(chunk: &mut Chunk,
                    value: crate::value::Value) -> usize {
    crate::value::write_values(&mut chunk.constants, value);
    chunk.constants.count - 1
}
