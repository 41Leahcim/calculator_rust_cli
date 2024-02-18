#![warn(clippy::pedantic, clippy::nursery)]

use std::iter::Peekable;

#[derive(Debug)]
pub enum Error {
    UnexpectedDot,
    InvalidChar(char),
    IndexOutOfBounds,
    UnexpectedNumber(char),
    MissingParenthesis(usize),
    UnexpectedParenthesis(char),
}

// Checks whether a character is an operator
pub const fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/'
}

// Reads and returns a number
fn number(
    sum: &str,
    iter: &mut Peekable<impl Iterator<Item = (usize, char)>>,
) -> Result<f64, Error> {
    // The number to multiply the decimals with
    let mut found_dot = false;

    // Return an error, if the function was called with an index outside the sum
    let (start, c) = iter.next().ok_or(Error::IndexOutOfBounds)?;

    // A number can't start with any character other than a digit or decimal point, or -
    if c == '.' {
        found_dot = true;
    } else if !c.is_ascii_digit() && c != '-' {
        return Err(Error::InvalidChar(c));
    }

    let mut end = start;
    // For each character from index i onward
    while let Some((index, c)) = iter.peek().copied() {
        if c == '.' {
            if found_dot {
                return Err(Error::UnexpectedDot);
            }
            found_dot = true;
        } else if !c.is_ascii_digit() {
            end = index;
            break;
        }
        iter.next();
    }

    // Take the part of the sum that contains the number and parse it to f64
    let number = if iter.peek().is_none() {
        &sum[start..]
    } else {
        &sum[start..end]
    }
    .parse::<f64>()
    .unwrap();
    Ok(number)
}

fn parentheses(sum: &str, iter: &mut impl Iterator<Item = (usize, char)>) -> Result<f64, Error> {
    // Store the start of this section of the sum
    let start = iter.nth(1).unwrap().0;

    // Find the closing parenthesis
    let mut depth: usize = 1;
    let mut end = start;
    for (index, c) in iter.by_ref() {
        match c {
            ')' if depth > 1 => depth -= 1,
            ')' => {
                depth -= 1;
                end = index;
                break;
            }
            '(' => depth += 1,
            _ => {}
        }
    }

    // If we got more opening than closing parenthesis, return an error
    if depth > 0 {
        return Err(Error::MissingParenthesis(depth));
    }

    // Split the sum in operators and numbers
    let (mut operators, mut numbers) = self::sum(&sum[start..end])?;

    // Evaluate the sum to return a number or an error
    Ok(crate::evaluate(&mut operators, &mut numbers))
}

pub fn sum(sum: &str) -> Result<(Vec<char>, Vec<f64>), Error> {
    let mut operators = Vec::new();
    let mut numbers = Vec::new();
    let mut iter = sum.char_indices().peekable();

    // Continue while there are chars left to process
    while let Some((_, c)) = iter.peek().copied() {
        // If the current char is an opening parenthesis
        if c == '(' {
            // Evaluate the sum between parentheses, and add the result to the numbers Vec
            numbers.push(parentheses(sum, &mut iter)?);
        } else {
            // Otherwise, try to parse the next char(s) as a number and add it to the numbers Vec
            numbers.push(self::number(sum, &mut iter)?);
        }

        // Get the next char, break if there is none
        let Some((_, c)) = iter.next() else {
            break;
        };

        // If the next char is an operator, add it to the operators Vec
        // Otherwise, return an error
        if is_operator(c) {
            operators.push(c);
        } else if c.is_ascii_digit() || c == '.' {
            // An unexpected number error for dot and digits
            return Err(Error::UnexpectedNumber(c));
        } else if c == '(' || c == ')' {
            // Unexpected parenthesis for opening/closing parentheses
            return Err(Error::UnexpectedParenthesis(c));
        } else {
            // Invalid char for everything else
            return Err(Error::InvalidChar(c));
        }
    }

    // Add a number, if we miss one
    if numbers.len() == operators.len() {
        numbers.push(self::number(sum, &mut iter)?);
    }

    // Return the operators and numbers
    Ok((operators, numbers))
}
