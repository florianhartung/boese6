#[derive(Clone, Debug)]
pub enum Token {
    Newline,
    Keyword(Keyword),
    Identifier(String),
    Equals,
    DoubleEquals,
    Plus,
    Minus,
    Number(String),
    Less,
    Greater,
    LessEq,
    GreaterEq,
    LogicalOr,
    LogicalAnd,
    NumericLiteral(u32),
    LParen,
    RParen,
    LBrace,
    RBrace,
}

#[derive(Clone, Debug)]
pub enum Keyword {
    Function,
    Const,
    Bool,
    Int,
    TryUse,
    End,
    ContinueIf,
    Continue,
}

impl Keyword {
    pub fn try_from_str(s: &str) -> Option<Self> {
        let keyword = match s {
            "function" => Keyword::Function,
            "const" => Keyword::Const,
            "bool" => Keyword::Bool,
            "int" => Keyword::Int,
            "tryuse" => Keyword::TryUse,
            "end" => Keyword::End,
            "continueif" => Keyword::ContinueIf,
            "continue" => Keyword::Continue,
            _ => {
                return None;
            }
        };
        Some(keyword)
    }
}
