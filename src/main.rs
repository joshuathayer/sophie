#[macro_use]
mod alloc;

mod chunk;
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;

use std::env;
use std::fs;

extern crate num;

#[macro_use]
extern crate num_derive;

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut vm = crate::vm::init_vm();

    run_file(&mut vm, &filename);

    // let mut ch = chunk::init_chunk();
    // let constant = chunk::add_constant(&mut ch, 99.2);
    // let constant2 = chunk::add_constant(&mut ch, 100.1);
    // chunk::write_chunk(&mut ch, chunk::Opcode::OPCONSTANT as u8, 123);
    // chunk::write_chunk(&mut ch, constant as u8, 123);
    // chunk::write_chunk(&mut ch, chunk::Opcode::OPCONSTANT as u8, 123);
    // chunk::write_chunk(&mut ch, constant as u8, 123);
    // chunk::write_chunk(&mut ch, chunk::Opcode::OPCONSTANT as u8, 123);
    // chunk::write_chunk(&mut ch, constant as u8, 123);
    // chunk::write_chunk(&mut ch, chunk::Opcode::OPADD as u8, 125);
    // chunk::write_chunk(&mut ch, chunk::Opcode::OPADD as u8, 125);
    // chunk::write_chunk(&mut ch, chunk::Opcode::OPRETURN as u8, 127);

    // crate::vm::interpret(&mut vm, &ch);
}

fn run_file(mut vm: &mut vm::VM, filename: &str) {

    let contents = fs::read_to_string(filename)
        .expect("Failed to read source");

    let result: vm::InterpretResult = crate::vm::interpret(&mut vm, &contents);

    // if (result == INTERPRET_COMPILE_ERROR) exit(65);
    // if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}
