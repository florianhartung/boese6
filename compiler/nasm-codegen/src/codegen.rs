use crate::module::{AsmCodeLine, Module};
use std::iter;

struct CodeGenerator {
    final_code: String,
}
impl CodeGenerator {
    fn new() -> Self {
        Self {
            final_code: String::new(),
        }
    }

    pub fn label(&mut self, label: &str) {
        self.write(format!("{label}:").as_str());
        self.write_newline();
    }

    pub fn instruction<S: AsRef<str>>(&mut self, instruction: S, instruction_args: &[S]) {
        let combined_args: String =
            itertools::intersperse(instruction_args.iter().map(S::as_ref), ", ").collect();

        self.write_tabs(1);
        if combined_args.len() == 0 {
            self.write_aligned(iter::once(instruction));
        } else {
            self.write_aligned([instruction.as_ref(), combined_args.as_ref()].iter());
        }
        self.write_newline();
    }

    pub fn write_block<F: FnOnce(&mut CodeGenerator)>(&mut self, block_write: F) {
        let len_before = self.final_code.len();

        block_write(self);

        let len_after = self.final_code.len();

        if len_after != len_before {
            self.write_newline()
        }
    }

    fn write_newline(&mut self) {
        self.write("\r\n");
    }

    fn write(&mut self, text: &str) {
        self.final_code.push_str(text.as_ref());
    }

    fn write_aligned<S: AsRef<str>, T: Iterator<Item = S>>(&mut self, text_columns: T) {
        const COLUMN_LENGTH: usize = 10;

        let mut last_col: Option<S> = None;
        text_columns.for_each(|col| {
            if let Some(last_col) = &last_col {
                let num_needed_padding_spaces = COLUMN_LENGTH
                    .checked_sub(last_col.as_ref().len())
                    .unwrap_or(1);
                self.write(" ".repeat(num_needed_padding_spaces).as_str());
            }
            self.write(col.as_ref());
            last_col = Some(col);
        });
    }

    fn write_tabs(&mut self, num: usize) {
        self.write("\t".repeat(num).as_str());
    }

    fn into_final_code(self) -> String {
        self.final_code
    }
}

pub fn from_module(module: Module) -> String {
    let mut gen = CodeGenerator::new();

    gen.write_block(|gen| {
        module.globals.into_iter().for_each(|g_name| {
            gen.write(format!("global {g_name}").as_str());
            gen.write_newline();
        });
    });

    gen.write_block(|gen| {
        module.externals.into_iter().for_each(|g_name| {
            gen.write(format!("extern {g_name}").as_str());
            gen.write_newline();
        });
    });

    module.asm_code.into_iter().for_each(|instruction| {
        match instruction {
            AsmCodeLine::Label(name) => gen.label(name.as_str()),
            AsmCodeLine::Instruction(instruction, args) => {
                let combined_args =
                    itertools::intersperse(args.iter().map(String::as_str), ", ").collect();
                gen.instruction(instruction, &[combined_args]);
            }
        };
    });

    gen.into_final_code()
}
