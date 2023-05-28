pub type AstRootCompound = Vec<AstRoot>;

#[derive(Debug)]
pub enum AstRoot {
    //TODO Constant
    Function(AstFunction),
}

#[derive(Debug)]
pub struct AstFunction {
    pub name: String,
    pub compound: AstCompound,
}

#[derive(Debug)]
pub enum AstStatement {
    Compound(AstCompound),
    Continue,
    End,
    TryUse(Ability),
    // Declaration(Declaration),
    // Assignment(Assignment),
}

// pub enum VariableType {
//     INT,
//     BOOL,
// }

#[derive(Debug)]
pub enum Ability {
    D,
    H,
    B,
    W,
}

#[derive(Debug)]
pub struct AstCompound {
    pub nodes: Vec<AstStatement>,
}
