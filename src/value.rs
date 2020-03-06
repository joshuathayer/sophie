#[macro_use]

pub type Value = f64;

pub enum ValueType {
    BOOL(bool),
    NIL(()),
    NUMBER(f64),
}

// rust vals -> sophie vals
#[macro_export]
macro_rules! bool_val {
    ($value:expr) => {
        crate::value::ValueType::BOOL($value)
    };
}

#[macro_export]
macro_rules! nil_val {
    () => {
        crate::value::ValueType::NIL()
    };
}

#[macro_export]
macro_rules! number_val {
    ($value:expr) => {
        crate::value::ValueType::NUMBER($value)
    };
}

// -- sophie vals -> rust vals
macro_rules! as_bool {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::BOOL(b) => &b,
            _ => false
        }
    }}
}

macro_rules! as_number {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::NUMBER(n) =>  n,
            _ => 0.0
        }
    }}
}
// --

macro_rules! is_bool {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::BOOL(_) => true,
            _ => false
        }
    }}
}

macro_rules! is_nil {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::NIL() => true,
            _ => false
        }
    }}
}


macro_rules! is_number {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::NUMBER(_) => true,
            _ => false
        }
    }}
}



pub struct Values {
    pub values: Vec<Value>
}

#[allow(dead_code)]
pub fn init_values() -> Values {
    Values {
        values: vec![0.0; 0],
    }
}


impl Values {
    pub fn write_values(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
}

pub fn print_value(value: &ValueType) {
    print!("{}", as_number!(*value))
}
