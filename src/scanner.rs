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
        self.clean_tokens()
    }

    fn clean_tokens(&mut self) -> Vec<Token> {
        let mut my_tokens: Vec<Token> = vec![];
        for t in self.source.by_ref() {
            match t {
                Token::Unused => (),
                Token::Newline => (),
                token => my_tokens.push(token),
            }
        }
        my_tokens
    }
}
