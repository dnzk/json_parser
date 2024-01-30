use crate::token::{Token, TokenData};

struct Source {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Source {
    fn from(source: String) -> Self {
        Source {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self, token: Token) -> Option<Token> {
        self.current += 1;
        Some(token)
    }
}

impl Iterator for Source {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        if let Some(c) = self.source.chars().nth(self.current) {
            return match c {
                '{' => self.advance(Token::LeftBrace(TokenData {
                    lexeme: None,
                    line: self.line,
                })),
                '}' => self.advance(Token::RightBrace(TokenData {
                    lexeme: None,
                    line: self.line,
                })),
                '\n' => {
                    self.line += 1;
                    self.advance(Token::Newline)
                }
                _ => None,
            };
        }
        None
    }
}

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
        while let Some(t) = self.source.next() {
            tokens.push(t);
        }

        tokens
    }
}
