use crate::token_type::TokenType;

pub const CHAR_EOF: char = '0';

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self { token_type }
    }
}

#[cfg(test)]
mod tests_token {
    use crate::token::Token;
    use crate::token_type::TokenType;

    #[test]
    fn test_new_token() {
        let want = Token {
            token_type: TokenType::StartToken,
        };
        let got = Token::new(TokenType::StartToken);
        assert_eq!(want, got);
    }
}
