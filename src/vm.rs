extern crate num_derive;
use num::{FromPrimitive};

pub struct VM<'a> {
    // pub chunk: Option<crate::chunk::Chunk>,
    pub ip: usize,
    pub stack: Vec<crate::value::ValueType<'a>>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VM<'a> {
    pub fn peek(&'a self, ix: usize) -> &'a crate::value::ValueType {
        self.stack.get(self.stack.len()-(1 + ix)).unwrap()
    }
}

macro_rules! read_byte {
    ($vm:expr, $chunk:expr) => {{
        $vm.ip += 1;
        $chunk.code[$vm.ip - 1]
    }};
}

// macro_rules! read_constant {
//     ($vm:expr, $chunk:expr) => {
//         &$chunk
//             .constants
//             .values[read_byte!($vm, $chunk) as usize]
//     };
// }

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
        // chunk: Some(crate::chunk::init_chunk()),
        ip: 0,
        stack: Vec::new(),
    }
}

pub fn free_vm() {}

impl<'a> VM<'a> {
    pub fn interpret(&'a mut self,
                     source: &str,
                     chunk: &'a mut crate::chunk::Chunk) -> InterpretResult {
        if !crate::compiler::compile(source, chunk) {
            return InterpretResult::CompileError
        };

        self.ip = 0;

        self.run(chunk)
    }

    // we move chunk to run, borrow stuff from within it, but then
    // when run finishes, chunk gets cleaned up and we're no longer
    // able to ref wiithin it.
    // so... pass a ref to chunk?
    fn run(&'a mut self, chunk: &'a mut crate::chunk::Chunk) -> InterpretResult {
        loop {
            // debug
            print!("        ");
            for s in self.stack.iter() {
                print!("[ ");
                crate::value::print_value(s);
                print!(" ]");
            }
            println!("");

            crate::debug::disassemble_instruction(
                // &self.chunk.as_ref().unwrap(),
                &chunk,
                self.ip);

            let instruction: Option<crate::chunk::Opcode> =
                crate::chunk::Opcode::from_u8(
                    read_byte!(self, chunk)
                );

            match instruction {
                Some(crate::chunk::Opcode::OPRETURN) => {
                    crate::value::print_value(&self.stack.pop().unwrap());
                    println!();
                    return InterpretResult::Ok },
                Some(crate::chunk::Opcode::OPADD) =>
                    binary_op!(self, +, number_val),
                Some(crate::chunk::Opcode::OPSUBTRACT) =>
                    binary_op!(self, -, number_val),
                Some(crate::chunk::Opcode::OPMULTIPLY) =>
                    binary_op!(self, *, number_val),
                Some(crate::chunk::Opcode::OPDIVIDE) =>
                    binary_op!(self, /, number_val),
                Some(crate::chunk::Opcode::OPLT) =>
                    binary_op!(self, <, bool_val),
                Some(crate::chunk::Opcode::OPGT) =>
                    binary_op!(self, >, bool_val),
                Some(crate::chunk::Opcode::OPLTE) =>
                    binary_op!(self, <=, bool_val),
                Some(crate::chunk::Opcode::OPGTE) =>
                    binary_op!(self, >=, bool_val),
                Some(crate::chunk::Opcode::OPNOT) => {
                    let v = &self.stack.pop().unwrap();
                    self.stack.push(
                        crate::value::ValueType::BOOL(is_falsey(v))
                    )
                },
                Some(crate::chunk::Opcode::OPCONSTANT) => {


                    //let constant = read_constant!(self, chunk);
                    // borrow a ConstantType from chunk
                    let constant = &chunk
                        .constants
                        .values[read_byte!(self, chunk) as usize];

                    // Make a new ValueType, which points to the value
                    // in the ConstantType. Push that onto our stack.
                    self.stack.push(
                        match constant {
                            crate::value::ConstantType::NUMBER(n) =>
                                crate::value::ValueType::NUMBER(*n),
                            crate::value::ConstantType::STRING(s) =>
                                crate::value::ValueType::STRING(&s)
                        }
                    );

                },
                Some(crate::chunk::Opcode::OPNIL) =>
                    self.stack.push(crate::value::ValueType::NIL),
                Some(crate::chunk::Opcode::OPTRUE) =>
                    self.stack.push(crate::value::ValueType::BOOL(true)),
                Some(crate::chunk::Opcode::OPFALSE) =>
                    self.stack.push(crate::value::ValueType::BOOL(false)),
                Some(crate::chunk::Opcode::OPEQUAL) => {
                    let l = &self.stack.pop().unwrap();
                    let r = &self.stack.pop().unwrap();
                    self.stack.push(
                        crate::value::ValueType::BOOL(values_equal(l, r))
                    )
                }

                Some(crate::chunk::Opcode::OPLEN) => {

                    let v = match self.stack.pop().unwrap() {
                        crate::value::ValueType::STRING(s) => {
                            crate::value::ValueType::NUMBER(s.len() as f64)
                        },
                        _ => {
                            crate::value::ValueType::NIL
                        }
                    };

                    self.stack.push(v)
                }
                _ => return InterpretResult::CompileError,
            }

        }
    }
    // chunk.constants.values dropped here while still borrowed...
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
