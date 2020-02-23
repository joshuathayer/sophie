#[macro_use]
mod alloc;

mod chunk;
mod debug;
mod value;
mod vm;

extern crate num;

#[macro_use]
extern crate num_derive;

fn main() {
    let mut vm = crate::vm::init_vm();

    let mut ch = chunk::init_chunk();
    let constant = chunk::add_constant(&mut ch, 99.2);
    let constant2 = chunk::add_constant(&mut ch, 100.1);
    chunk::write_chunk(&mut ch, chunk::Opcode::OPCONSTANT as u8, 123);
    chunk::write_chunk(&mut ch, constant as u8, 123);
    chunk::write_chunk(&mut ch, chunk::Opcode::OPCONSTANT as u8, 124);
    chunk::write_chunk(&mut ch, constant2 as u8, 124);
    chunk::write_chunk(&mut ch, chunk::Opcode::OPADD as u8, 125);
    chunk::write_chunk(&mut ch, chunk::Opcode::OPNEGATE as u8, 125);
    chunk::write_chunk(&mut ch, chunk::Opcode::OPCONSTANT as u8, 126);
    chunk::write_chunk(&mut ch, constant2 as u8, 125);
    chunk::write_chunk(&mut ch, chunk::Opcode::OPDIVIDE as u8, 126);
    chunk::write_chunk(&mut ch, chunk::Opcode::OPRETURN as u8, 127);

    crate::vm::interpret(&mut vm, &ch);
}
