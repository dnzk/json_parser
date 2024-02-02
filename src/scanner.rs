use crate::source::Source;
use crate::token::Token;

pub struct Scanner {
    source: Source,
}

impl Scanner {
    pub fn from(source: String) -> Self {
        Scanner {
            source: Source::from(source),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        for t in self.source.by_ref() {
            match t {
                Token::Unused => (),
                Token::Newline => (),
                token => tokens.push(token),
            }
        }
        tokens
    }
}
