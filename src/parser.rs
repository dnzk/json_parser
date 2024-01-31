use crate::token::Token;

#[derive(Debug)]
struct Pair {
    left: Option<Token>,
    right: Option<Token>,
}

impl Pair {
    fn is_valid(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn from(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn is_valid(self) -> bool {
        let mut pair = Pair {
            left: None,
            right: None,
        };
        dbg!(&self.tokens);
        for token in self.tokens {
            match token {
                Token::LeftBrace(_) => {
                    pair.left = Some(token);
                }
                Token::RightBrace(_) => {
                    pair.right = Some(token);
                }
                _ => (),
            }
        }

        pair.is_valid()
    }
}
