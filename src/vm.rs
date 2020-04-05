extern crate num_derive;
use num::{FromPrimitive};
use std::collections::{HashMap};

pub struct VM<'a> {
    // pub chunk: Option<crate::chunk::Chunk>,
    pub ip: usize,
    pub stack: Vec<crate::value::ValueType<'a>>,
    pub symbols: HashMap<&'a str, crate::value::ValueType<'a>>
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

macro_rules! bool_op {
    ($vm:expr, $op:tt) => {{

        let l = $vm.stack.pop().unwrap();
        let r = $vm.stack.pop().unwrap();

        $vm.stack.push(
            match (l, r) {
                (crate::value::ValueType::INT(lv),
                 crate::value::ValueType::INT(rv)) => {
                    bool_val!(lv $op rv)
                }

                (crate::value::ValueType::FLOAT(lv),
                 crate::value::ValueType::INT(rv)) => {
                    bool_val!(lv $op (rv as f64))
                }

                (crate::value::ValueType::INT(lv),
                 crate::value::ValueType::FLOAT(rv)) => {
                    bool_val!((lv as f64) $op rv)
                }
                (crate::value::ValueType::FLOAT(lv),
                 crate::value::ValueType::FLOAT(rv)) => {
                    bool_val!(lv $op rv)
                }
                _ => {
                    runtime_error("Operands to bool ops must be numbers");
                    return InterpretResult::RuntimeError
                }
            }
        );
    }};
}

// may push a float or an int
macro_rules! number_op {
    ($vm:expr, $op:tt) => {{
        let r = $vm.stack.pop().unwrap();
        let l = $vm.stack.pop().unwrap();

        $vm.stack.push(
            match (l, r) {
                (crate::value::ValueType::INT(lv),
                 crate::value::ValueType::INT(rv)) => {
                    int_val!(lv $op rv)
                }

                (crate::value::ValueType::FLOAT(lv),
                 crate::value::ValueType::INT(rv)) => {
                    float_val!(lv $op (rv as f64))
                }

                (crate::value::ValueType::INT(lv),
                 crate::value::ValueType::FLOAT(rv)) => {
                    float_val!((lv as f64) $op rv)
                }
                (crate::value::ValueType::FLOAT(lv),
                 crate::value::ValueType::FLOAT(rv)) => {
                    float_val!(lv $op rv)
                }
                _ => {
                    runtime_error("Operands to number ops must be numbers");
                    return InterpretResult::RuntimeError
                }
            }
        );
    }};
}


pub fn init_vm<'a>() -> VM<'a> {
    VM {
        ip: 0,
        stack: Vec::new(),
        symbols: HashMap::new()
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

                // binary ops
                Some(crate::chunk::Opcode::OPADD) =>
                    number_op!(self, +),
                Some(crate::chunk::Opcode::OPSUBTRACT) =>
                    number_op!(self, -),
                Some(crate::chunk::Opcode::OPMULTIPLY) =>
                    number_op!(self, *),
                Some(crate::chunk::Opcode::OPDIVIDE) =>
                    number_op!(self, /),
                Some(crate::chunk::Opcode::OPLT) =>
                    bool_op!(self, <),
                Some(crate::chunk::Opcode::OPGT) =>
                    bool_op!(self, >),
                Some(crate::chunk::Opcode::OPLTE) =>
                    bool_op!(self, <=),
                Some(crate::chunk::Opcode::OPGTE) =>
                    bool_op!(self, >=),

                Some(crate::chunk::Opcode::OPNOT) => {
                    let v = &self.stack.pop().unwrap();
                    self.stack.push(
                        crate::value::ValueType::BOOL(is_falsey(v))
                    )
                },
                Some(crate::chunk::Opcode::OPCONSTANT) => {

                    // borrow a ConstantType from chunk
                    let constant = &chunk
                        .constants
                        .values[read_byte!(self, chunk) as usize];

                    // Make a new ValueType, which points to the value
                    // in the ConstantType. Push that onto our stack.
                    self.stack.push(
                        match constant {
                            crate::value::ConstantType::INT(n) =>
                                crate::value::ValueType::INT(*n),
                            crate::value::ConstantType::FLOAT(n) =>
                                crate::value::ValueType::FLOAT(*n),
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
                            crate::value::ValueType::INT(s.len() as i64)
                        },
                        _ => {
                            crate::value::ValueType::NIL
                        }
                    };

                    self.stack.push(v)
                }

                Some(crate::chunk::Opcode::OPPRINT) => {
                    let v = &self.stack.pop().unwrap();
                    print!("printing: {:?}", v);
                    self.stack.push(
                        crate::value::ValueType::NIL
                    )
                }

                Some(crate::chunk::Opcode::OPPOP) => {
                    self.stack.pop();
                }

                Some(crate::chunk::Opcode::OPDEF) => {
                    let v = self.stack.pop().unwrap();
                    let s = &self.stack.pop().unwrap();

                    match s {
                        crate::value::ValueType::STRING(sym) =>{
                            self.symbols.insert(sym, v);
                        }
                        _ => {
                            print!("Symbols must be strings\n")
                        }
                    }

                    self.stack.push(
                        crate::value::ValueType::NIL
                    )
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
        (crate::value::ValueType::INT(lv),
         crate::value::ValueType::INT(rv)) => { lv == rv },
        (crate::value::ValueType::FLOAT(lv),
         crate::value::ValueType::FLOAT(rv)) => { lv == rv },
        (_,_) => false
    }

}

fn runtime_error(msg: &str) {
    println!("There was an error: {}", msg);
}
