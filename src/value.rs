pub type Value = f64;

pub struct Values {
    pub values: Vec<Value>
}

#[allow(dead_code)]
pub fn init_values() -> Values {
    Values {
        values: vec![0.0; 8],
    }
}


impl Values {
    pub fn write_values(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
}

pub fn print_value(value: Value) {
    print!("{}", value)
}
