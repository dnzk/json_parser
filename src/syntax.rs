use crate::token::Token;

pub struct SyntaxChecker {
    tokens: Vec<Token>,
}

impl SyntaxChecker {
    pub fn from(tokens: Vec<Token>) -> Self {
        SyntaxChecker { tokens }
    }

    pub fn valid(&self) -> bool {
        let mut index = 0;
        let mut valid = true;
        while index < self.tokens.len() {
            if index + 1 == self.tokens.len() {
                match self.tokens[index] {
                    Token::RightBrace(_) => (),
                    _ => valid = false,
                }
                break;
            }
            let next_token = &self.tokens[index + 1];
            match self.tokens[index] {
                Token::LeftBrace(_) => valid = valid && after_left_brace(next_token),
                Token::RightBrace(_) => valid = valid && after_right_brace(next_token),
                Token::LeftBracket(_) => valid = valid && after_left_bracket(next_token),
                Token::RightBracket(_) => valid = valid && after_right_bracket(next_token),
                Token::Null(_) => valid = valid && after_null(next_token),
                Token::Key(_, _) => valid = valid && after_key(next_token),
                Token::String(_, _) => valid = valid && after_string(next_token),
                Token::Colon(_) => valid = valid && after_colon(next_token),
                Token::Comma(_) => valid = valid && after_comma(next_token),
                Token::True(_) | Token::False(_) => valid = valid && after_boolean(next_token),
                Token::Number(_, _) => valid = valid && after_number(next_token),
                _ => (),
            }
            index += 1;
        }
        valid
    }
}

fn after_left_brace(token: &Token) -> bool {
    matches!(token, Token::RightBrace(_) | Token::Key(_, _))
}

fn after_right_brace(token: &Token) -> bool {
    after_value(token)
}

fn after_left_bracket(token: &Token) -> bool {
    matches!(
        token,
        Token::RightBracket(_)
            | Token::LeftBrace(_)
            | Token::Number(_, _)
            | Token::String(_, _)
            | Token::True(_)
            | Token::False(_)
            | Token::Null(_)
    )
}

fn after_right_bracket(token: &Token) -> bool {
    matches!(token, Token::Comma(_) | Token::RightBrace(_))
}

fn after_key(token: &Token) -> bool {
    matches!(token, Token::Colon(_))
}

fn after_colon(token: &Token) -> bool {
    matches!(
        token,
        Token::LeftBrace(_)
            | Token::LeftBracket(_)
            | Token::Number(_, _)
            | Token::String(_, _)
            | Token::True(_)
            | Token::False(_)
            | Token::Null(_)
    )
}

fn after_comma(token: &Token) -> bool {
    matches!(
        token,
        Token::Key(_, _)
            | Token::Number(_, _)
            | Token::String(_, _)
            | Token::True(_)
            | Token::False(_)
            | Token::Null(_)
    )
}

fn after_number(token: &Token) -> bool {
    after_value(token)
}

fn after_string(token: &Token) -> bool {
    after_value(token)
}

fn after_boolean(token: &Token) -> bool {
    after_value(token)
}

fn after_null(token: &Token) -> bool {
    after_value(token)
}

fn after_value(token: &Token) -> bool {
    matches!(
        token,
        Token::Comma(_) | Token::RightBrace(_) | Token::RightBracket(_)
    )
}
