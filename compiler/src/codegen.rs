use nasm_codegen::Module;

use crate::ast::{Ability, AstCompound, AstFunction, AstRoot, AstRootCompound, AstStatement};

struct CodeGen {
    module: Module,
}

impl CodeGen {
    fn new() -> Self {
        Self {
            module: Module::new(),
        }
    }

    fn function(&mut self, function: AstFunction) {
        if function.name == "main" {
            self.module.global("decide");
            self.module.label("decide");
        } else {
            self.module.label(function.name.as_ref());
        }

        // TODO PROLOG
        self.compound(function.compound);
        // TODO EPILOG
    }

    fn compound(&mut self, compound: AstCompound) {
        compound
            .nodes
            .into_iter()
            .for_each(|statement| self.statement(statement));
    }

    fn statement(&mut self, statement: AstStatement) {
        match statement {
            AstStatement::Compound(compound) => self.compound(compound),
            AstStatement::Continue => self.statement_continue(),
            AstStatement::End => self.statement_end(),
            AstStatement::TryUse(ability) => self.statement_try_use(ability),
        }
    }

    fn statement_continue(&mut self) {
        self.module.instruction("mov", &["eax", "0"]);
        self.module.instruction("ret", &[]);
    }

    fn statement_end(&mut self) {
        self.module.instruction("mov", &["eax", "1"]);
        self.module.instruction("ret", &[]);
    }

    fn statement_try_use(&mut self, ability: Ability) {
        match ability {
            _ => todo!(),
        }
    }

    fn ast_root_compound(&mut self, ast_root_compound: AstRootCompound) {
        ast_root_compound
            .into_iter()
            .for_each(|ast_root| match ast_root {
                AstRoot::Function(ast_function) => self.function(ast_function),
            });
    }

    fn into_nasm(self) -> String {
        self.module.into_generated_nasm()
    }
}

pub fn generate_nasm_from_ast(ast_root_compound: AstRootCompound) -> String {
    let mut codegen = CodeGen::new();

    codegen.ast_root_compound(ast_root_compound);

    codegen.into_nasm()
}
