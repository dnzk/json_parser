use crate::token::Token;

pub struct SyntaxChecker {
    tokens: Vec<Token>,
}

impl SyntaxChecker {
    pub fn from(tokens: Vec<Token>) -> Self {
        SyntaxChecker { tokens }
    }

    pub fn valid(&self) -> bool {
        let mut left = 0;
        let mut right = self.tokens.len() - 1;
        while left < right {
            let valid_left = valid_sequence(&self.tokens[left], &self.tokens[left + 1]);
            if !valid_left {
                return false;
            }
            let valid_right = valid_sequence(&self.tokens[right - 1], &self.tokens[right]);
            if !valid_right {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

/// Validates a sequence of two tokens
fn valid_sequence(token: &Token, next_token: &Token) -> bool {
    match token {
        Token::LeftBrace(_) => after_left_brace(next_token),
        Token::RightBrace(_) => after_right_brace(next_token),
        Token::LeftBracket(_) => after_left_bracket(next_token),
        Token::RightBracket(_) => after_right_bracket(next_token),
        Token::Null(_) => after_null(next_token),
        Token::Key(_, _) => after_key(next_token),
        Token::String(_, _) => after_string(next_token),
        Token::Colon(_) => after_colon(next_token),
        Token::Comma(_) => after_comma(next_token),
        Token::True(_) | Token::False(_) => after_boolean(next_token),
        Token::Number(_, _) => after_number(next_token),
        _ => false,
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
