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

    // let vm = crate::vm::init_vm();

    // let mut vm = crate::vm::VM {
    //    chunk: Some(crate::chunk::init_chunk()),
    //    ip: 0,
    //    stack: Vec::new(),
    // };

   run_file(&filename);
}

fn run_file<'a>(filename: &str) {

    let mut vm = crate::vm::VM {
       chunk: Some(crate::chunk::init_chunk()),
       ip: 0,
       stack: Vec::new(),
    };

    let contents = fs::read_to_string(filename)
        .expect("Failed to read source");

    let _result: vm::InterpretResult = vm.interpret(&contents);

    // if (result == INTERPRET_COMPILE_ERROR) exit(65);
    // if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}
