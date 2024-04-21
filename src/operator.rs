use crate::sum_error::SumError;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<char> for Operator {
    type Error = SumError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        // Return the right operator for the correct characters.
        // Return an error for characters that aren't operators.
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            _ => Err(SumError::InvalidOperator(value)),
        }
    }
}
