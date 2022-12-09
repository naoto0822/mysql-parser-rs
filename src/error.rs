/// LexErrorType
#[derive(Clone, Debug, PartialEq)]
pub enum LexErrorType {
    InvalidChar(String),
}

/// LexError
#[derive(Clone, Debug, PartialEq)]
pub struct LexError {
    pub error_type: LexErrorType,
}

impl LexError {
    pub fn new(t: LexErrorType) -> Self {
        Self { error_type: t }
    }

    pub fn invalid_char(c: String) -> Self {
        LexError::new(LexErrorType::InvalidChar(c))
    }
}
