use crate::i;

pub struct Function {
    name: String,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: format!("func_{}", name),
        }
    }
}
