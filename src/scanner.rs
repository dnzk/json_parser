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

    fn peek_next(&self) -> Option<char> {
        let peek_index = self.current + 1;
        if peek_index <= self.source.len() {
            return self.source.chars().nth(peek_index);
        }
        None
    }

    fn advance(&mut self, token: Token) -> Option<Token> {
        self.current += 1;
        Some(token)
    }

    fn string(&mut self) -> Option<Token> {
        let mut string_content = String::new();
        let mut is_key = false;
        while let Some(next) = self.peek_next() {
            if next == '"' {
                self.current += 1;
                if let Some(next) = self.peek_next() {
                    if next == ':' {
                        is_key = true;
                        string_content = self.source[self.start + 1..self.current].to_string();
                        self.current += 1;
                        break;
                    }
                }
                string_content = self.source[self.start + 1..self.current].to_string();
                self.current += 1;
                break;
            } else {
                self.current += 1;
            }
        }
        if is_key {
            return Some(Token::Key(
                TokenData {
                    lexeme: Some(string_content.clone()),
                    line: self.line,
                },
                string_content,
            ));
        }
        Some(Token::String(
            TokenData {
                lexeme: Some(string_content.clone()),
                line: self.line,
            },
            string_content,
        ))
    }

    fn maybe_boolean(&mut self, is_true: bool) -> Option<Token> {
        if is_true {
            if &self.source[self.current..self.current + 4] == "true" {
                self.current += 4;
                return Some(Token::True(TokenData {
                    lexeme: Some(String::from("true")),
                    line: self.line,
                }));
            }
        }
        if &self.source[self.current..self.current + 5] == "false" {
            self.current += 5;
            return Some(Token::False(TokenData {
                lexeme: Some(String::from("false")),
                line: self.line,
            }));
        }
        self.current += 1;
        Some(Token::Unused)
    }

    fn maybe_null(&mut self) -> Option<Token> {
        if &self.source[self.current..self.current + 4] == "null" {
            self.current += 4;
            return Some(Token::Null(TokenData {
                lexeme: Some(String::from("null")),
                line: self.line,
            }));
        }
        self.current += 1;
        Some(Token::Unused)
    }

    fn number(&mut self) -> Option<Token> {
        let mut result = String::new();
        self.current += 1;
        while let Some(next) = self.peek_next() {
            if next.is_ascii_digit() {
                self.current += 1;
                result = self.source[self.start..self.current + 1].to_string();
            } else {
                self.current += 1;
                break;
            }
        }
        Some(Token::Number(
            TokenData {
                lexeme: Some(result.to_string()),
                line: self.line,
            },
            result.parse::<f64>().unwrap(),
        ))
    }
}

impl Iterator for Source {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        if let Some(c) = self.source.chars().nth(self.current) {
            return match c {
                '{' => self.advance(Token::LeftBrace(TokenData {
                    lexeme: Some(String::from("{")),
                    line: self.line,
                })),
                '}' => self.advance(Token::RightBrace(TokenData {
                    lexeme: Some(String::from("}")),
                    line: self.line,
                })),
                '\n' => {
                    self.line += 1;
                    self.advance(Token::Newline)
                }
                '"' => self.string(),
                ':' => self.advance(Token::Colon(TokenData {
                    lexeme: Some(String::from(":")),
                    line: self.line,
                })),
                ',' => self.advance(Token::Comma(TokenData {
                    lexeme: Some(String::from(",")),
                    line: self.line,
                })),
                't' => self.maybe_boolean(true),
                'f' => self.maybe_boolean(false),
                'n' => self.maybe_null(),
                n if n.is_ascii_digit() => self.number(),
                w if w.is_ascii_whitespace() => {
                    self.current += 1;
                    Some(Token::Unused)
                }
                _ => {
                    self.current += 1;
                    Some(Token::Unused)
                }
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
            match t {
                Token::Unused => (),
                Token::Newline => (),
                token => tokens.push(token),
            }
        }

        dbg!(&tokens);
        tokens
    }
}
