use crate::error::LexError;
use crate::token_type::TokenType;
use crate::token::{Token, CHAR_ZERO_VALUE, CHAR_EOF};

pub struct Lexer {
    query: String,
    query_length: usize,
    current_position: usize,
    current_char: char,
    next_position: usize,
    is_peek_position: bool,
}

impl Lexer {

    pub fn new(query: String) -> Lexer {
        let query_length = query.chars().count();

        let mut lexer = Lexer {
            query,
            query_length,
            current_position: 0,
            next_position: 0,
            current_char: CHAR_ZERO_VALUE,
            is_peek_position: false,
        };

        // to first char
        lexer.read_char();

        lexer
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(tk) = self.next_token() {
            match tk.token_type {
                TokenType::EOF => break,
                TokenType::Illegal(c) => {
                    return Err(LexError::invalid_char(c));
                }
                _ => {
                    tokens.push(tk);
                }
            };
        }

        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.current_char {
            // Synbol Char
            '&' => Token::new(TokenType::Ampersand),
            '@' => Token::new(TokenType::AtMark),
            '*' => Token::new(TokenType::Ast),
            '!' => Token::new(TokenType::Bang),
            ',' => Token::new(TokenType::Comma),
            '$' => Token::new(TokenType::DollerSign),
            '"' => Token::new(TokenType::DoubleQuote),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::EqEqOp)
                } else {
                    Token::new(TokenType::EqOp)
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::GraterThanEqOp)
                } else if self.peek_char() == '>' {
                    self.read_char();
                    Token::new(TokenType::RightShiftOp)
                } else {
                    Token::new(TokenType::GreaterOp)
                }
            },
            '[' => Token::new(TokenType::LeftBra),
            '{' => Token::new(TokenType::LeftBrace),
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::LessThanEqOp)
                } else if self.peek_char() == '<' {
                    Token::new(TokenType::LeftShiftOp)
                } else {
                    Token::new(TokenType::LessOp)
                }
            },
            '(' => Token::new(TokenType::Lparen),
            '-' => Token::new(TokenType::Minus),
            '%' => Token::new(TokenType::Percent),
            '.' => Token::new(TokenType::Period),
            '+' => Token::new(TokenType::Plus),
            '\'' => Token::new(TokenType::Quote),
            ')' => Token::new(TokenType::Rparen),
            ']' => Token::new(TokenType::RightBra),
            '}' => Token::new(TokenType::RightBrace),
            ';' => Token::new(TokenType::SemiColon),
            '/' => Token::new(TokenType::Slash),
            '_' => Token::new(TokenType::UnderScore),
            '|' => Token::new(TokenType::VerticalBar),

            // EOF
            '0' => Token::new(TokenType::EOF),

            // identifier or number or illegal
            _ => {
                if self.is_letter() {
                    self.is_peek_position = true;
                    let ident = self.read_identifier();
                    let token_type = Token::lookup_ident(ident.clone());
                    Token::new(token_type)
                } else if self.is_number() {
                    self.is_peek_position = true;
                    let number = self.read_number();
                    Token::new(TokenType::NumberType(number))
                } else {
                    Token::new(TokenType::Illegal(self.current_char.to_string()))
                }
            }
        };

        // current position is plus one when token is identifier, number...
        if self.is_peek_position {
            self.is_peek_position = false;
            return Some(token);
        }

        self.read_char();
        Some(token)
    }

    fn is_over_next_position(&self) -> bool {
        self.next_position >= self.query_length
    }

    fn read_char(&mut self) {
        if self.is_over_next_position() {
            // '0' is EOF
            self.current_char = CHAR_EOF;
        } else {
            self.current_char = self.query.as_bytes()[self.next_position] as char;
        }

        self.current_position = self.next_position;
        self.next_position += 1;
    }

    fn peek_char(&mut self) -> char {
        if self.is_over_next_position() {
            return CHAR_EOF;
        }

        self.query.as_bytes()[self.next_position] as char
    }

    fn is_letter(&self) -> bool {
        self.current_char.is_ascii_alphabetic()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.current_position;
        while self.is_letter() {
            self.read_char()
        }

        // FIXME: like index range access...  query[position:self.current_position]
        let cap = self.current_position - position;
        let mut collected: Vec<u8> = Vec::with_capacity(cap as usize);
        for (i, q) in self.query.as_bytes().iter().enumerate() {
            if position <= i && i < self.current_position {
                collected.push(q.clone());
            }
        }

        // TODO: error handling
        String::from_utf8(collected).unwrap()
    }

    fn is_number(&self) -> bool {
        self.current_char.is_ascii_digit()
    }

    fn read_number(&mut self) -> i64 {
        let position = self.current_position;
        while self.is_number() {
            self.read_char();
        }

        // FIXME: like index range access...  query[position:self.current_position]
        let cap = self.current_position - position;
        let mut collected: Vec<u8> = Vec::with_capacity(cap as usize);
        for (i, q) in self.query.as_bytes().iter().enumerate() {
            if position <= i && i < self.current_position {
                collected.push(q.clone());
            }
        }

        // TODO: error handling
        let joined = String::from_utf8(collected).unwrap();
        joined.parse::<i64>().unwrap()
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.current_char == ' ' || self.current_char == '\n' || self.current_char == '\t' {
                self.read_char()
            } else {
                break;
            }
        }
    }

    pub fn dump_raw_query(&self) {
        println!("query: {}", self.query)
    }
}

#[cfg(test)]
mod tests_lexer {
    use crate::token::Token;
    use crate::lexer::Lexer;
    use crate::token_type::TokenType;

    #[test]
    fn test_get_tokens() {
        let in1 = "SELECT * FROM user WHERE id = 1;".to_string();
        let mut lexer = Lexer::new(in1);
        let got1 = lexer.get_tokens();
        match got1 {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e)
        }
    }
}
