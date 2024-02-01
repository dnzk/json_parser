use crate::token::Token;

pub struct TokenCleaner {
    tokens: Vec<Token>,
}

impl TokenCleaner {
    pub fn from(tokens: Vec<Token>) -> Result<Self, String> {
        if tokens.len() == 0 {
            return Err("JSON is empty".to_string());
        }
        let mut my_tokens: Vec<Token> = vec![];
        for t in tokens {
            match t {
                Token::Unused => (),
                token => my_tokens.push(token),
            }
        }

        Ok(TokenCleaner { tokens: my_tokens })
    }
}

pub struct BraceValidator {
    balance: i64,
    tokens: Vec<Token>,
}

impl BraceValidator {
    pub fn from(token_cleaner: TokenCleaner) -> Self {
        let mut balance = 0;
        for t in &token_cleaner.tokens {
            match t {
                Token::LeftBrace(_) => balance -= 1,
                Token::RightBrace(_) => balance += 1,
                _ => (),
            }
        }

        BraceValidator {
            balance,
            tokens: token_cleaner.tokens,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.balance == 0
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}

pub struct SyntaxValidator {
    tokens: Vec<Token>,
}

impl SyntaxValidator {
    pub fn from(tokens: Vec<Token>) -> Self {
        SyntaxValidator { tokens }
    }

    pub fn validate(&self) -> bool {
        let mut index = 0;
        let mut valid = true;
        while index < self.tokens.len() {
            if index + 1 == self.tokens.len() {
                match self.tokens[index] {
                    Token::RightBrace(_) => valid = valid && true,
                    _ => valid = false,
                }
                break;
            }
            let next_token = &self.tokens[index + 1];
            match self.tokens[index] {
                Token::LeftBrace(_) => {
                    valid = valid && SyntaxValidator::after_left_brace(next_token)
                }
                Token::RightBrace(_) => {
                    valid = valid && SyntaxValidator::after_right_brace(next_token)
                }
                Token::Key(_, _) => valid = valid && SyntaxValidator::after_key(next_token),
                Token::String(_, _) => valid = valid && SyntaxValidator::after_string(next_token),
                Token::Colon(_) => valid = valid && SyntaxValidator::after_colon(next_token),
                Token::Comma(_) => valid = valid && SyntaxValidator::after_comma(next_token),
                _ => (),
            }
            index += 1;
        }
        valid
    }

    fn after_left_brace(token: &Token) -> bool {
        match token {
            Token::RightBrace(_) | Token::Key(_, _) => true,
            _ => false,
        }
    }

    fn after_right_brace(token: &Token) -> bool {
        match token {
            Token::Comma(_) => true,
            Token::RightBrace(_) => true,
            Token::RightBracket(_) => true,
            _ => false,
        }
    }

    fn after_left_bracket(token: &Token) -> bool {
        match token {
            Token::RightBracket(_) => true,
            Token::LeftBrace(_) => true,
            Token::Number(_, _) => true,
            Token::String(_, _) => true,
            Token::Boolean(_) => true,
            Token::Null(_) => true,
            _ => false,
        }
    }

    fn after_right_bracket(token: &Token) -> bool {
        match token {
            Token::Comma(_) => true,
            _ => false,
        }
    }

    fn after_key(token: &Token) -> bool {
        match token {
            Token::Colon(_) => true,
            _ => false,
        }
    }

    fn after_colon(token: &Token) -> bool {
        match token {
            Token::LeftBrace(_) => true,
            Token::LeftBracket(_) => true,
            Token::Number(_, _) => true,
            Token::String(_, _) => true,
            Token::Boolean(_) => true,
            Token::Null(_) => true,
            _ => false,
        }
    }

    fn after_comma(token: &Token) -> bool {
        match token {
            Token::Key(_, _) => true,
            Token::Number(_, _) => true,
            Token::String(_, _) => true,
            Token::Boolean(_) => true,
            Token::Null(_) => true,
            _ => false,
        }
    }

    fn after_number(token: &Token) -> bool {
        SyntaxValidator::after_value(token)
    }

    fn after_string(token: &Token) -> bool {
        SyntaxValidator::after_value(token)
    }

    fn after_boolean(token: &Token) -> bool {
        SyntaxValidator::after_value(token)
    }

    fn after_null(token: &Token) -> bool {
        SyntaxValidator::after_value(token)
    }

    fn after_value(token: &Token) -> bool {
        match token {
            Token::Comma(_) => true,
            Token::RightBrace(_) => true,
            Token::RightBracket(_) => true,
            _ => false,
        }
    }
}
