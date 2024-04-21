use crate::operator::Operator;

pub fn evaluate(numbers: &mut Vec<f64>, operators: &mut Vec<Operator>) -> f64 {
    // Make sure there are more numbers than operators or no numbers or operators at all
    assert!(
        (numbers.is_empty() && operators.is_empty()) || numbers.len() > operators.len(),
        "There should be more numbers than operators"
    );

    // Evaluate the multiplicative operations first
    let mut i = 0;
    while let Some(op) = operators.get(i) {
        match *op {
            Operator::Mul => {
                operators.remove(i);
                numbers[i] *= numbers.remove(i + 1);
            }
            Operator::Div => {
                operators.remove(i);
                numbers[i] /= numbers.remove(i + 1);
            }
            Operator::Add | Operator::Sub => i += 1,
        }
    }

    // Then the additive operations
    i = 0;
    while let Some(op) = operators.get(i) {
        match *op {
            Operator::Add => {
                numbers[i] += numbers.remove(i + 1);
                operators.remove(i);
            }
            Operator::Sub => {
                numbers[i] -= numbers.remove(i + 1);
                operators.remove(i);
            }
            Operator::Mul | Operator::Div => i += 1,
        }
    }

    // Make sure only the result is left
    assert!(
        numbers.len() <= 1 && operators.is_empty(),
        "Something went wrong while evaluating the sum"
    );

    // Return the result or 0
    numbers.first().copied().unwrap_or_default()
}
