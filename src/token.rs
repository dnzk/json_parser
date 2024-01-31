#[derive(Debug, Clone)]
pub struct TokenData {
    pub lexeme: Option<String>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Token {
    LeftBrace(TokenData),
    RightBrace(TokenData),
    LeftBracket(TokenData),
    RightBracket(TokenData),
    Number(TokenData, f64),
    String(TokenData, String),
    Key(TokenData, String),
    Boolean(TokenData),
    Null(TokenData),
    Colon(TokenData),
    Comma(TokenData),
    Newline,
    Unused,
}
