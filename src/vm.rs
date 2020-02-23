extern crate num_derive;
use num::{FromPrimitive};

pub struct VM<'a> {
    pub chunk: Option<&'a crate::chunk::Chunk>,
    pub ip: usize,
    pub stack: Vec<crate::value::Value>,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}


macro_rules! read_byte {
    ($vm:expr) => {{
        $vm.ip += 1;
        $vm.chunk.unwrap().code[$vm.ip - 1]
    }};
}

macro_rules! read_constant {
    ($vm:expr) => {
        $vm.chunk.unwrap().constants.values[read_byte!($vm) as usize]
    };
}

macro_rules! binary_op {
    ($vm:expr, $op:tt) => {{
        let b = $vm.stack.pop().unwrap();
        let a = $vm.stack.pop().unwrap();
        $vm.stack.push(a $op b);
    }};
}
pub fn init_vm() -> VM<'static> {
    VM {
        chunk: None,
        ip: 0,
        stack: Vec::new(),
    }
}

pub fn free_vm() {}

pub fn interpret<'a>(vm: &mut VM<'a>,
                     chunk: &'a crate::chunk::Chunk)
                     -> InterpretResult {
    vm.chunk = Some(chunk);
    vm.ip = 0;

    run(vm)
}

fn run(vm: &mut VM) -> InterpretResult {
    loop {

        // debug
        print!("        ");
        for s in vm.stack.iter() {
            print!("[ ");
            crate::value::print_value(*s);
            print!(" ]");
        }
        println!("");
        crate::debug::disassemble_instruction(vm.chunk.unwrap(), vm.ip);

        let instruction: Option<crate::chunk::Opcode> =
            crate::chunk::Opcode::from_u8(read_byte!(vm));

        match instruction {
            Some(crate::chunk::Opcode::OPRETURN) => {
                crate::value::print_value(vm.stack.pop().unwrap());
                println!();
                return InterpretResult::InterpretOk},
            Some(crate::chunk::Opcode::OPNEGATE) =>
                if let Some(last) = vm.stack.last_mut() {
                    *last *= -1.0;
                },
            Some(crate::chunk::Opcode::OPADD) => binary_op!(vm, +),
            Some(crate::chunk::Opcode::OPSUBTRACT) => binary_op!(vm, -),
            Some(crate::chunk::Opcode::OPMULTIPLY) => binary_op!(vm, *),
            Some(crate::chunk::Opcode::OPDIVIDE) => binary_op!(vm, /),
            Some(crate::chunk::Opcode::OPCONSTANT) => {
                let constant = read_constant!(vm);
                vm.stack.push(constant);
            }
            _ => return InterpretResult::InterpretCompileError,
        }

    }
}
