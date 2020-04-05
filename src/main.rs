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
use std::collections::HashMap;

extern crate num;

#[macro_use]
extern crate num_derive;

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // let vm = crate::vm::init_vm();
    let mut chunk = crate::chunk::init_chunk();

    let mut vm = crate::vm::VM {
       ip: 0,
        stack: Vec::new(),
        symbols: HashMap::new()
    };

    let _res = run_file(&filename, &mut vm, &mut chunk);
    // res
}

fn run_file<'a>(filename: &'a str,
                vm: &'a mut crate::vm::VM<'a>,
                chunk: &'a mut crate::chunk::Chunk)  {

    let contents = fs::read_to_string(filename)
        .expect("Failed to read source");

    vm.interpret(&contents, chunk);

    //if (result == INTERPRET_COMPILE_ERROR) exit(65);
    //if (result == INTERPRET_RUNTIME_ERROR) exit(70);
}
