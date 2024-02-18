#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

mod chars;
mod parse;

use std::io::{stdin, Read};

use crate::chars::Chars;

fn read_sum(sum: &mut String) {
    eprint!(">>> ");
    sum.clear();
    let iter = Chars::new(
        stdin()
            .lock()
            .bytes()
            .take_while(Result::is_ok)
            .map(Result::unwrap),
    )
    .take_while(|c| *c != '\n')
    .filter(|c| !c.is_whitespace());
    for c in iter {
        sum.push(c);
    }
}

pub fn evaluate(operators: &mut Vec<char>, numbers: &mut Vec<f64>) -> f64 {
    // Iterate through the operators, evaluating the multiplicative ones
    let mut i = 0;
    while let Some(operator) = operators.get(i) {
        match operator {
            '*' => {
                // If the current operator is '*'
                // Multiply the current number with the next
                numbers[i] *= numbers[i + 1];

                // Remove the operator and the next number
                operators.remove(i);
                numbers.remove(i + 1);
            }
            '/' => {
                // If the current number is '/'
                // Divide the current number by the next
                numbers[i] /= numbers[i + 1];

                // Remove the current operator and the next number
                operators.remove(i);
                numbers.remove(i + 1);
            }
            // Otherwise, continue to the next operator
            _ => i += 1,
        }
    }

    // Iterate through the additive operators
    i = 0;
    while let Some(operator) = operators.get(i) {
        match operator {
            // If it was an addition operator
            '+' => {
                // Add the next number to the current
                numbers[i] += numbers[i + 1];

                // Remove the current operator and the next number
                operators.remove(i);
                numbers.remove(i + 1);
            }
            '-' => {
                // Subtract the next number from the current
                numbers[i] -= numbers[i + 1];

                // Remove the next operator and the next number
                operators.remove(i);
                numbers.remove(i + 1);
            }
            // Otherwise, continue to the next operator
            _ => i += 1,
        }
    }
    numbers[0]
}

fn main() {
    let mut sum = String::new();
    loop {
        // read sum
        read_sum(&mut sum);

        // parse sum, and evaluate sums between parentheses
        let (mut operators, mut numbers) = match parse::sum(&sum) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Failed to parse sum: {error:?}");
                continue;
            }
        };

        // evaluate the sum, and print the sum
        let result = evaluate(&mut operators, &mut numbers);
        eprintln!("{result}");
    }
}
