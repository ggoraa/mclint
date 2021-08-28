#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    As,
    And,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Default,
    Do,
    Else,
    Enum,
    Extends,
    Finally,
    For,
    Function,
    Has,
    Hidden,
    If,
    InstanceOf,
    Import,
    Me,
    Module,
    Private,
    Protected,
    Public,
    Or,
    Return,
    /// I just added a random character at the
    /// end so the compiler won't complain
    /// because of Self keyword usage
    SelfK,
    Static,
    Switch,
    Throw,
    Try,
    Using,
    Var,
    While,
    Bool,
    String,
    Int,
    Long,
    Float,
    Double,
    Null,
    Char,
    Nan,
    New,
    Identifier,
    Semicolon,
    Comma,
    /// This one: {
    OpeningBracket,
    /// This one: }
    ClosingBracket,
    /// This one: (
    OpeningBrace,
    /// This one: )
    ClosingBrace,
    Asterisk,
    Percent,
    Assign,
    Bang,
    Tilde,
    Plus,
    Minus,
    Slash,
    /// This one: &
    Ampersand,
    LessThan,
    GreaterThan,
    /// This one: ^
    Caret,
    VerticalBar
}
