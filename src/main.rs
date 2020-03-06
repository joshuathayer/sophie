#[macro_use]
mod alloc;

#[macro_use]
mod value;
mod chunk;
mod debug;
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
}

fn run_file(mut vm: &mut vm::VM, filename: &str) {

    let contents = fs::read_to_string(filename)
        .expect("Failed to read source");

    let result: vm::InterpretResult = crate::vm::interpret(&mut vm, &contents);

    // if (result == INTERPRET_COMPILE_ERROR) exit(65);
    // if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}
