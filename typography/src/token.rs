use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    StringLiteral(&'input str),
    CharLiteral(&'input str),
    Lifetime(&'input str),

    Semicolon,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenParenthesis,
    CloseParenthesis,
    Assign,
    ArrowAssign,
    Arrow,
    Question,
    Colon,
    Add,
    Subtract,
    Mul,
    Divide,
    Modulo,
    Not,
    OpenBracket,
    CloseBracket,
    Member,
    Comma,
    Complement,
    More,
    Less,

    Options,
    Spec,
    Typo,
    Default,
    Tokenizer,
    Node,
    Ast,
    Template,
    Impl,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            StringLiteral(id) => write!(f, "{}", id),
            CharLiteral(id) => write!(f, "{}", id),
            Lifetime(id) => write!(f, "{}", id),

            Semicolon => write!(f, ";"),
            Comma => write!(f, ","),
            OpenParenthesis => write!(f, "("),
            CloseParenthesis => write!(f, ")"),
            OpenCurlyBrace => write!(f, "{{"),
            CloseCurlyBrace => write!(f, "}}"),
            Add => write!(f, "+"),
            Subtract => write!(f, "-"),
            Mul => write!(f, "*"),
            Divide => write!(f, "/"),
            Modulo => write!(f, "%"),
            Assign => write!(f, "="),
            Not => write!(f, "!"),
            More => write!(f, ">"),
            Member => write!(f, "."),
            Colon => write!(f, ":"),
            OpenBracket => write!(f, "["),
            CloseBracket => write!(f, "]"),
            Complement => write!(f, "~"),
            Question => write!(f, "?"),
            Less => write!(f, "<"),
            ArrowAssign => write!(f, "->"),
            Arrow => write!(f, "=>"),

            Options => write!(f, "options"),
            Spec => write!(f, "spec"),
            Typo => write!(f, "typo"),
            Default => write!(f, "default"),
            Tokenizer => write!(f, "tokenizer"),
            Node => write!(f, "node"),
            Ast => write!(f, "ast"),
            Template => write!(f, "template"),
            Impl => write!(f, "impl"),
        }
    }
}
