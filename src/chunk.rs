#[derive(FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum Opcode {
    OPCONSTANT,
    OPNEGATE, // unused...
    OPADD,
    OPSUBTRACT,
    OPMULTIPLY,
    OPDIVIDE,
    OPRETURN,
    OPNIL,
    OPTRUE,
    OPFALSE,
    OPNOT,
    OPEQUAL,
    OPLT,
    OPGT,
    OPLTE,
    OPGTE,
    OPLEN
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u16>,
    pub constants: crate::value::Values,
}

pub fn init_chunk() -> Chunk {
    Chunk {
        code: vec![0; 0],
        lines: vec![0; 0],
        constants: crate::value::init_values(),
    }
}

impl Chunk {
    pub fn write_chunk(&mut self, byte: u8, line: u16)  {
        self.code.push(byte as u8);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self,
                        value: crate::value::ConstantType) -> usize {
        self.constants.write_values(value)
    }
}
