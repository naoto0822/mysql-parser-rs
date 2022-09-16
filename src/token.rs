use crate::token_type::TokenType;
use crate::token_type_map::KEYWORD_TOKEN_TYPE_MAP;

pub const CHAR_EOF: char = '0';

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self { token_type }
    }

    pub fn lookup_indent(ident: String) -> TokenType {
        match KEYWORD_TOKEN_TYPE_MAP.get(ident.as_str()) {
            Some(v) => v.clone(),
            None => TokenType::Ident(ident),
        }
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

    #[test]
    fn test_lookup_ident() {
        let in1 = "id".to_string();
        let want1 = TokenType::Ident("id".to_string());
        let got1 = Token::lookup_indent(in1);
        assert_eq!(want1, got1);

        let in2 = "SELECT".to_string();
        let want2 = TokenType::SelectKwd;
        let got2 = Token::lookup_indent(in2);
        assert_eq!(want2, got2);
    }
}
