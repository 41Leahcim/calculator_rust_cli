#![warn(clippy::pedantic, clippy::nursery)]

#[derive(Debug)]
pub enum Error {
    MissingClosingParentheses,
    UnmatchedParenthesis,
    InvalidOperator,
    InvalidChar(char),
    MissingNumber,
}

fn character(
    sum: &[char],
    i: usize,
    current_char: char,
    parentheses: &mut usize,
    last_was_operator: &mut bool,
) -> Result<(), Error> {
    match current_char {
        // If the current char is an opening parenthesis
        '(' => {
            // Increment the number of parentheses
            *parentheses += 1;
        }
        // If the current char is a closing parenthesis
        ')' => {
            // Decrement the number of parenthesis if possible, return an error otherwise
            if let Some(depth) = parentheses.checked_sub(1) {
                *parentheses = depth;
            } else {
                return Err(Error::UnmatchedParenthesis);
            }
        }
        // if the current char is a + or -
        '+' | '-' => {
            // Make sure the last character wasn't an operator
            if *last_was_operator {
                // If it was, this can be used as sign
                // Return an error if the character before the previous, wasn't an operator
                if let Some(second_last_char) = sum.get(i.saturating_sub(2)) {
                    match second_last_char {
                        '+' | '-' | '*' | '/' => return Err(Error::InvalidOperator),
                        _ => {}
                    }
                }
            } else {
                // If the last character wasn't an operator, remember that this char is one
                *last_was_operator = true;
            }
        }
        // If the current char is a * or /
        '*' | '/' => {
            // Return an error if the previous character was an operator
            if *last_was_operator {
                return Err(Error::InvalidOperator);
            }

            // Remember that this is an operator, if it wasn't
            *last_was_operator = true;
        }
        // If the current character is a digit or decimal point, remember that it isn't an operator
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
            *last_was_operator = false;
        }
        // Return an error for invalid characters
        _ => return Err(Error::InvalidChar(current_char)),
    }
    Ok(())
}

pub fn sum(sum: &[char]) -> Result<(), Error> {
    // Counts the number of still open parentheses (also called the depth)
    let mut parentheses: usize = 0;

    // Stores whether the last character was an operator
    let mut last_was_operator = false;

    // Check every character
    for (i, &current_char) in sum.iter().enumerate() {
        character(
            sum,
            i,
            current_char,
            &mut parentheses,
            &mut last_was_operator,
        )?;
    }

    // Check whether there were any parentheses left open or whether the last char was an operator
    // Return an error if needed.
    if parentheses > 0 {
        Err(Error::MissingClosingParentheses)
    } else if last_was_operator {
        Err(Error::MissingNumber)
    } else {
        Ok(())
    }
}
