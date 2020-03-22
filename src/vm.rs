extern crate num_derive;
use num::{FromPrimitive};

pub struct VM<'a> {
    pub chunk: Option<crate::chunk::Chunk<'a>>,
    pub ip: usize,
    pub stack: Vec<crate::value::ValueType<'a>>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VM<'_> {
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


// only numbers in binary ops
macro_rules! binary_op {
    ($vm:expr, $op:tt, $as_val:tt) => {{
        if !(is_number!($vm.peek(0)) &&
             is_number!($vm.peek(1))) {
            runtime_error("Operands to binary ops must be numbers");
            return InterpretResult::RuntimeError
        }
        let b = $vm.stack.pop().unwrap();
        let a = $vm.stack.pop().unwrap();
        $vm.stack.push($as_val!(as_number!(a) $op as_number!(b)));
    }};
}

pub fn init_vm<'a>() -> VM<'a> {
    VM {
        chunk: None,
        ip: 0,
        stack: Vec::new(),
    }
}

pub fn free_vm() {}

pub fn interpret(vm: &mut VM<'static>, source: &str) -> InterpretResult {

    let mut chunk = crate::chunk::init_chunk();

    if !crate::compiler::compile(source, &mut chunk) {
        return InterpretResult::CompileError
    };

    vm.chunk = Some(chunk);
    vm.ip = 0;

    run(vm)
}

fn run<'a>(vm: &mut VM<'static>) -> InterpretResult {
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
                return InterpretResult::Ok },
            Some(crate::chunk::Opcode::OPADD) =>
                binary_op!(vm, +, number_val),
            Some(crate::chunk::Opcode::OPSUBTRACT) =>
                binary_op!(vm, -, number_val),
            Some(crate::chunk::Opcode::OPMULTIPLY) =>
                binary_op!(vm, *, number_val),
            Some(crate::chunk::Opcode::OPDIVIDE) =>
                binary_op!(vm, /, number_val),
            Some(crate::chunk::Opcode::OPLT) =>
                binary_op!(vm, <, bool_val),
            Some(crate::chunk::Opcode::OPGT) =>
                binary_op!(vm, >, bool_val),
            Some(crate::chunk::Opcode::OPLTE) =>
                binary_op!(vm, <=, bool_val),
            Some(crate::chunk::Opcode::OPGTE) =>
                binary_op!(vm, >=, bool_val),
            Some(crate::chunk::Opcode::OPNOT) => {
                let v = &vm.stack.pop().unwrap();
                vm.stack.push(
                    crate::value::ValueType::BOOL(is_falsey(v))
                )
            },
            Some(crate::chunk::Opcode::OPCONSTANT) => {
                let constant = read_constant!(vm);
                // vm.stack.push(crate::value::ValueType::NUMBER(constant));
                vm.stack.push(constant);
            },
            Some(crate::chunk::Opcode::OPNIL) =>
                vm.stack.push(crate::value::ValueType::NIL),
            Some(crate::chunk::Opcode::OPTRUE) =>
                vm.stack.push(crate::value::ValueType::BOOL(true)),
            Some(crate::chunk::Opcode::OPFALSE) =>
                vm.stack.push(crate::value::ValueType::BOOL(false)),
            Some(crate::chunk::Opcode::OPEQUAL) => {
                let l = &vm.stack.pop().unwrap();
                let r = &vm.stack.pop().unwrap();
                vm.stack.push(
                    crate::value::ValueType::BOOL(values_equal(l, r))
                )
            }


            _ => return InterpretResult::CompileError,
        }

    }
}

fn is_falsey(v: &crate::value::ValueType) -> bool {
    is_nil!(*v) || (is_bool!(*v) && !(as_bool!(*v)))
}

fn values_equal(l: &crate::value::ValueType,
                r: &crate::value::ValueType) -> bool {

    match (l,r) {
        (crate::value::ValueType::BOOL(lv),
         crate::value::ValueType::BOOL(rv)) => { lv == rv },
        (crate::value::ValueType::NIL,
         crate::value::ValueType::NIL) => true,
        (crate::value::ValueType::NUMBER(lv),
         crate::value::ValueType::NUMBER(rv)) => { lv == rv },
        (_,_) => false
    }

}

fn runtime_error(msg: &str) {
    println!("There was an error: {}", msg);
}
