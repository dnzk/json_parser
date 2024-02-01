use crate::syntax::SyntaxChecker;
use crate::token::Token;

pub struct Validator {
    tokens: Vec<Token>,
}

impl Validator {
    pub fn from(tokens: Vec<Token>) -> Self {
        Validator {
            tokens: Validator::clean_tokens(tokens),
        }
    }

    pub fn valid(&self) -> bool {
        self.tokens.len() > 0 && self.valid_braces() && self.valid_brackets() && self.valid_syntax()
    }

    fn clean_tokens(tokens: Vec<Token>) -> Vec<Token> {
        let mut my_tokens: Vec<Token> = vec![];
        for t in tokens {
            match t {
                Token::Unused => (),
                token => my_tokens.push(token),
            }
        }
        my_tokens
    }

    fn valid_braces(&self) -> bool {
        let mut balance = 0;
        for t in &self.tokens {
            match t {
                Token::LeftBrace(_) => balance -= 1,
                Token::RightBrace(_) => balance += 1,
                _ => (),
            }
        }
        balance == 0
    }

    fn valid_brackets(&self) -> bool {
        let mut balance = 0;
        for t in &self.tokens {
            match t {
                Token::LeftBracket(_) => balance -= 1,
                Token::RightBracket(_) => balance += 1,
                _ => (),
            }
        }
        balance == 0
    }

    fn valid_syntax(&self) -> bool {
        let syntax_checker = SyntaxChecker::from(self.tokens.clone());
        syntax_checker.valid()
    }
}
