pub type Value = f64;

pub struct Values {
    pub capacity: usize,
    pub count: usize,
    pub values: Vec<Value>
}

#[allow(dead_code)]
pub fn init_values() -> Values {
    Values {
        capacity: 8,
        count: 0,
        values: vec![0.0; 8],
    }
}

#[allow(dead_code)]
pub fn write_values(values: &mut Values, value: Value)  {
    if values.capacity < values.count + 1 {
        values.capacity = grow_capacity!(values.capacity);
        grow_array!(values.values, values.capacity, 0.0);
    }

    values.values[values.count] = value;
    values.count += 1;
}

pub fn print_value(value: Value) {
    print!("{}", value)
}
