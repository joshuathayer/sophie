#[macro_use]

pub enum ConstantType {
    NUMBER(f64),
    STRING(String)
}

pub enum ValueType<'a> {
    BOOL(bool),
    NIL,
    NUMBER(f64),
    STRING(&'a String)
}

pub struct Values {
    pub values: Vec<ConstantType>
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
        crate::value::ValueType::NIL
    };
}

#[macro_export]
macro_rules! number_val {
    ($value:expr) => {
        crate::value::ValueType::NUMBER($value)
    };
}

#[macro_export]
macro_rules! string_val {
    ($value:expr) => {
        crate::value::ValueType::STRING($value)
    };
}


// -- sophie vals -> rust vals
macro_rules! as_bool {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::BOOL(b) => b,
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

macro_rules! as_string {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::STRING(n) =>  n,
            _ => ""
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
            crate::value::ValueType::NIL => true,
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

macro_rules! is_string {
    ($value:expr) => {{
        match $value {
            crate::value::ValueType::STRING(_) => true,
            _ => false
        }
    }}
}


#[allow(dead_code)]
pub fn init_values() -> Values {
    Values {
        values: vec![],
    }
}


impl Values {
    pub fn write_values(&mut self, value: ConstantType) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
}

pub fn print_value(value: &ValueType) {
    match value {
        ValueType::NUMBER(_) => print!("{}", as_number!(*value)),
        ValueType::NIL => print!("nil"),
        ValueType::BOOL(b) =>
            if *b {print!("true")} else {print!("false")},
        ValueType::STRING(_) => print!("{}", as_string!(*value))
    }

}
