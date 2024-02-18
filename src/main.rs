#![warn(clippy::pedantic, clippy::nursery)]

mod parse;

use std::io;

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
        eprint!(">>> ");
        sum.clear();
        io::stdin().read_line(&mut sum).unwrap();
        sum = sum.chars().filter(|c| !c.is_whitespace()).collect();

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
