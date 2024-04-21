use core::fmt;
use std::num::ParseFloatError;

#[derive(Debug, thiserror::Error)]
pub enum SumError {
    InvalidOperator(char),
    ParseFloatError(#[from] ParseFloatError),
    MissingOpeningParenthesis,
    MissingClosingParenthesis,
    NotMoreOperatorsThanNumbers,
    NumberTooLong,
    UnexpectedEndOfSum,
}

impl fmt::Display for SumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(clippy::pattern_type_mismatch)]
        match self {
            Self::InvalidOperator(ch) => writeln!(
                f,
                "Invalid character where number or parenthesis was expected: {ch}"
            ),
            Self::ParseFloatError(error) => writeln!(f, "Failed to parse float: {error}"),
            Self::MissingOpeningParenthesis => {
                writeln!(f, "Tried evaluating parentheses where none where found")
            }
            Self::MissingClosingParenthesis => writeln!(f, "Missing closing parenthesis"),
            Self::NotMoreOperatorsThanNumbers => {
                writeln!(f, "There should be more numbers than operators")
            }
            Self::NumberTooLong => writeln!(f, "Number too long"),
            Self::UnexpectedEndOfSum => writeln!(f, "Unexpected end of sum"),
        }
    }
}
