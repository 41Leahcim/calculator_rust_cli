#![warn(clippy::pedantic, clippy::nursery)]

#[derive(Debug)]
pub enum Error {
    UnexpectedDot,
    InvalidChar(char),
    UnexpectedOperator(char),
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
fn number(sum: &[char], i: &mut usize) -> Result<f64, Error> {
    // Stores the number
    let mut number = 0.0;

    // The number to multiply the decimals with
    let mut modifier: f64 = 1.0;

    // Return an error, if the function was called with an index outside the sum
    if *i >= sum.len() {
        return Err(Error::IndexOutOfBounds);
    }

    if sum[*i] == '-' {
        // A - sign would make the number negative.
        modifier = -1.0;
        *i += 1;
    } else if sum[*i] == '+' {
        // A + sign would make the number positive, which is the default.
        *i += 1;
    } else if is_operator(sum[*i]) {
        // A number can't contain an operator
        return Err(Error::UnexpectedOperator(sum[*i]));
    }

    // A number can't start with any character other than a digit or decimal point, -, or +
    if sum[*i] != '.' && !sum[*i].is_ascii_digit() {
        return Err(Error::InvalidChar(sum[*i]));
    }

    // For each character from index i onward
    for &c in sum.iter().skip(*i) {
        // If the current character is a digit, add it to the number:
        if let Some(digit) = c.to_digit(10) {
            // Convert it to an f64
            let digit: f64 = f64::from(digit);

            // If the modifier is smaller than 1.0, the digit will be after the decimal point.
            // Otherwise it will be just before the decimal dot.
            // The second case needs to be split into 2 for positive and negative numbers.
            if modifier.abs() < 1.0 {
                number += digit * modifier;
                modifier /= 10.0;
            } else if modifier < 0.0 {
                number = number.mul_add(10.0, -digit);
            } else {
                number = number.mul_add(10.0, digit);
            }
        } else if c == '.' {
            // If the current character is a decimal dot:
            // Return an error if there was a decimal dot before in this number.
            // Set the modifier to -0.1 if it is negative or 0.1 if it is positive.
            if modifier.abs() < 1.0 {
                return Err(Error::UnexpectedDot);
            } else if modifier < 0.0 {
                modifier = -0.1;
            } else {
                modifier = 0.1;
            }
        } else {
            // Break out of the loop, if it is neither
            break;
        }
        // Increment the index here, to use the index outside the loop
        *i += 1;
    }
    Ok(number)
}

fn parentheses(i: &mut usize, sum: &[char]) -> Result<f64, Error> {
    // Increment i
    *i += 1;

    // Store the start of this section of the sum
    let start = *i;

    // Find the closing parenthesis
    let mut depth: usize = 1;
    while *i < sum.len() && depth > 0 {
        //
        if sum[*i] == ')' {
            depth -= 1;
        } else if sum[*i] == '(' {
            depth += 1;
        }
        *i += 1;
    }

    // If we got more opening than closing parenthesis, return an error
    if depth > 0 {
        return Err(Error::MissingParenthesis(depth));
    }

    // Split the sum in operators and numbers
    let (mut operators, mut numbers) = self::sum(&sum[start..*i - 1])?;

    // Evaluate the sum to return a number or an error
    Ok(crate::evaluate(&mut operators, &mut numbers))
}

pub fn sum(sum: &[char]) -> Result<(Vec<char>, Vec<f64>), Error> {
    let mut operators = Vec::new();
    let mut numbers = Vec::new();
    let mut i = 0;

    // Continue while there are chars left to process
    while let Some(&c) = sum.get(i) {
        // If the current digit is an opening parenthesis
        if c == '(' {
            // Evaluate the sum between parentheses, and add the result to the numbers Vec
            numbers.push(parentheses(&mut i, sum)?);
        } else {
            // Otherwise, try to parse the next char(s) as a number and add it to the numbers Vec
            numbers.push(self::number(sum, &mut i)?);
        }

        // Get the next char, break if there is none
        let Some(&c) = sum.get(i) else {
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

        // Continue to the next index
        i += 1;
    }

    // Add a number, if we miss one
    if numbers.len() == operators.len() {
        numbers.push(self::number(sum, &mut i)?);
    }

    // Return the operators and numbers
    Ok((operators, numbers))
}
