use core::iter::Peekable;
use itertools::Itertools;

use crate::{evaluate::evaluate, operator::Operator, sum_error::SumError};

#[allow(clippy::significant_drop_tightening)]
fn read_number<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Result<f64, SumError> {
    // Take the first character
    let c = chars.next().ok_or(SumError::UnexpectedEndOfSum)?;

    // Create a new string on the stack to save a heap allocation
    let buffer = chars
        .peeking_take_while(|&c| c.is_ascii_digit() || c == '.')
        .fold(c.to_string(), |mut out, c| {
            out.push(c);
            out
        });

    // Parse the number and return the result
    buffer.parse::<f64>().map_err(SumError::from)
}

fn evaluate_parenthesis<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
) -> Result<f64, SumError> {
    // Make sure the first character returned by the iterator is an opening parenthesis
    if chars.next() != Some('(') {
        return Err(SumError::MissingOpeningParenthesis);
    }

    // Parse the sum between
    let (mut numbers, mut operators) = parse_inner(chars, true)?;

    // Make sure a closing parenthesis was found
    if chars.next() != Some(')') {
        return Err(SumError::MissingClosingParenthesis);
    }

    // Evaluate the sum
    Ok(evaluate(&mut numbers, &mut operators))
}

fn parse_inner<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
    in_parentheses: bool,
) -> Result<(Vec<f64>, Vec<Operator>), SumError> {
    // Create buffers for the operators and numbers
    let mut operators = vec![];
    let mut numbers = vec![];

    // parse the sum
    loop {
        // Peek for the first character
        let Some(ch) = chars.peek() else {
            break;
        };

        match *ch {
            // If it is a numeric character, read the full number
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' | '.' => {
                numbers.push(read_number(chars)?);
            }

            // If it is an opening parenthesis, evaluate the sum between the parentheses
            '(' => numbers.push(evaluate_parenthesis(chars)?),

            // If a closing parenthesis is found and the iterator is between parentheses,
            // break out of the loop.
            ')' if in_parentheses => break,

            // Otherwise, throw an error
            _ => return Err(SumError::InvalidOperator(*ch)),
        }

        // Check the next character
        let Some(ch) = chars.peek() else {
            break;
        };

        // If it is a closing parenthesis and the iterator is between parentheses,
        // break out of the loop.
        if in_parentheses && *ch == ')' {
            break;
        }

        // Try to convert the character to an operator and add it to the operator buffer.
        // Otherwise, throw an error
        operators.push(Operator::try_from(*ch)?);

        // Skip to the next character
        chars.next();
    }

    // Make sure there are no numbers or more numbers than operators
    if numbers.is_empty() || numbers.len() > operators.len() {
        return Err(SumError::NotMoreOperatorsThanNumbers);
    }
    Ok((numbers, operators))
}

pub fn parse<T: Iterator<Item = char>>(chars: T) -> Result<(Vec<f64>, Vec<Operator>), SumError> {
    // Parse the sum with a peekable operator
    parse_inner(chars.peekable().by_ref(), false)
}
