#[derive(Debug)]
pub struct Unit {
    id: usize,
    name: String,
    unit_class: u8,
    unit_function: u8,
}

impl Unit {
    pub fn new(id: usize, name: String, unit_class: u8, unit_function: u8) -> Self {
        Self {
            id,
            name,
            unit_class,
            unit_function,
        }
    }
}
