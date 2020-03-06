extern crate num_derive;
use num::{FromPrimitive};

pub struct VM {
    pub chunk: Option<crate::chunk::Chunk>,
    pub ip: usize,
    pub stack: Vec<crate::value::ValueType>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VM {
    pub fn peek(&'a self, ix: usize) -> &'a crate::value::ValueType {
        self.stack.get(self.stack.len()-(1 + ix)).unwrap()
    }
}

macro_rules! read_byte {
    ($vm:expr) => {{
        $vm.ip += 1;
        $vm.chunk.as_ref().unwrap().code[$vm.ip - 1]
    }};
}

macro_rules! read_constant {
    ($vm:expr) => {
        $vm.chunk.
            as_ref()
            .unwrap()
            .constants
            .values[read_byte!($vm) as usize]
    };
}

// for numbers
macro_rules! binary_op {
    ($vm:expr, $op:tt) => {{
        // only numbers in binary ops
        if !(is_number!($vm.peek(0)) &&
             is_number!($vm.peek(1))) {
            runtime_error("Operands to binary ops must be numbers");
            return InterpretResult::RuntimeError
        }
        let b = $vm.stack.pop().unwrap();
        let a = $vm.stack.pop().unwrap();
        $vm.stack.push(number_val!(as_number!(a) $op as_number!(b)));
    }};
}

pub fn init_vm() -> VM {
    VM {
        chunk: None,
        ip: 0,
        stack: Vec::new(),
    }
}

pub fn free_vm() {}

pub fn interpret(vm: &mut VM, source: &str) -> InterpretResult {

    let mut chunk = crate::chunk::init_chunk();

    if !crate::compiler::compile(source, &mut chunk) {
        return InterpretResult::CompileError
    };

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
            crate::value::print_value(s);
            print!(" ]");
        }
        println!("");
        crate::debug::disassemble_instruction(&vm.chunk.as_ref().unwrap(), vm.ip);

        let instruction: Option<crate::chunk::Opcode> =
            crate::chunk::Opcode::from_u8(read_byte!(vm));

        match instruction {
            Some(crate::chunk::Opcode::OPRETURN) => {
                crate::value::print_value(&vm.stack.pop().unwrap());
                println!();
                return InterpretResult::Ok},
            // Some(crate::chunk::Opcode::OPNEGATE) =>
            //     if let Some(last) = vm.stack.last_mut() {
            //         *last *= -1.0;
            //     },
            Some(crate::chunk::Opcode::OPADD) => binary_op!(vm, +),
            Some(crate::chunk::Opcode::OPSUBTRACT) => binary_op!(vm, -),
            Some(crate::chunk::Opcode::OPMULTIPLY) => binary_op!(vm, *),
            Some(crate::chunk::Opcode::OPDIVIDE) => binary_op!(vm, /),
            Some(crate::chunk::Opcode::OPCONSTANT) => {
                let constant = read_constant!(vm);
                vm.stack.push(crate::value::ValueType::NUMBER(constant));
            }
            _ => return InterpretResult::CompileError,
        }

    }
}

fn runtime_error(msg: &str) {
    println!("There was an error: {}", msg);
}
