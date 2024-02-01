use crate::token::Token;
use crate::validator::{BraceValidator, SyntaxValidator, TokenCleaner};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn from(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn is_valid(&self) -> bool {
        match TokenCleaner::from(self.tokens.clone()) {
            Ok(token_cleaner) => {
                let brace_validator = BraceValidator::from(token_cleaner);
                if brace_validator.is_valid() {
                    let syntax_validator = SyntaxValidator::from(brace_validator.get_tokens());
                    return syntax_validator.validate();
                }
            }
            Err(_error) => return false,
        }
        false
    }
}
