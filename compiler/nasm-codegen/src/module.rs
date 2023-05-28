use crate::codegen;
use std::collections::HashSet;

#[macro_export]
macro_rules! i {
    ($module:expr) => {
        let module: &mut Module = $writer; // Check type

        writer.instruction("call", &["hi"]);
    };
}

pub struct Module {
    pub(crate) globals: HashSet<String>,
    pub(crate) externals: HashSet<String>,
    pub(crate) asm_code: Vec<AsmCodeLine>,
}

pub enum AsmCodeLine {
    Label(String),
    Instruction(String, Vec<String>),
}

impl Module {
    pub fn new() -> Self {
        Self {
            globals: HashSet::new(),
            externals: HashSet::new(),
            asm_code: Vec::new(),
        }
    }

    pub fn into_generated_nasm(self) -> String {
        codegen::from_module(self)
    }

    pub fn r#extern(&mut self, name: &str) {
        if !self.externals.contains(name) {
            self.externals.insert(name.to_owned());
        }
    }

    pub fn global(&mut self, name: &str) {
        if !self.globals.contains(name) {
            self.globals.insert(name.to_owned());
        }
    }

    pub fn label(&mut self, label: &str) {
        self.asm_code.push(AsmCodeLine::Label(label.to_owned()));
    }

    pub fn instruction(&mut self, instruction: &str, instruction_args: &[&str]) {
        self.asm_code.push(AsmCodeLine::Instruction(
            instruction.to_owned(),
            instruction_args
                .iter()
                .copied()
                .map(&str::to_owned)
                .collect(),
        ));
    }
}
