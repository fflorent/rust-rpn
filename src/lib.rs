enum Operator {
  Addition,
  Substraction,
  Multiplication,
  Division,
  Modulo
}

enum OperationElt {
  Operator(Operator),
  Operand(f32)
}

fn tokenizer(expr: &str) -> Result<Vec<OperationElt>, String> {
  expr.split_whitespace()
    .map(|el| {
      match el {
        "+" => Ok(OperationElt::Operator(Operator::Addition)),
        "-" => Ok(OperationElt::Operator(Operator::Substraction)),
        "*" => Ok(OperationElt::Operator(Operator::Multiplication)),
        "/" => Ok(OperationElt::Operator(Operator::Division)),
        "%" => Ok(OperationElt::Operator(Operator::Modulo)),
        operand => match operand.parse::<f32>() {
          Ok(val) => Ok(OperationElt::Operand(val)),
          Err(_) => Err(format!("Cannot parse operand \"{}\"", operand))
        }
      }
    })
    .into_iter()
    .collect()
}

/// Evaluates an RPL expression.
///
/// # Examples
/// ````
/// extern crate rpn;
///
/// let result:f32 = rpn::evaluate("5 2 +").unwrap();
/// assert_eq!(result, 7.0);
/// ````
///
/// # Errors
/// This function will return an error in case of bad expression:
///
/// - if it includes an unrecognized operator (recognized ones are +, -, * and /
/// - if it misses an operand (i.e. value)
/// - if it misses an operator
pub fn evaluate(expr: &str) -> Result<f32, String> {
  return match tokenizer(expr) {
    Ok(tokens) => {
      let mut stack:Vec<f32> = Vec::new();
      for token in tokens {
        match token {
          OperationElt::Operator(operator) => {
            if stack.len() < 2 {
              return Err("Unsufficient operands before operator".to_string());
            }
            let operand2 = stack.pop().expect("expected f32 values in stack");
            let operand1 = stack.pop().expect("expected f32 values in stack");
            let result = match operator {
              Operator::Addition        => operand1 + operand2,
              Operator::Substraction    => operand1 - operand2,
              Operator::Multiplication  => operand1 * operand2,
              Operator::Division        => operand1 / operand2,
              Operator::Modulo          => operand1 % operand2
            };
            stack.push(result);
          },
          OperationElt::Operand(val) => stack.push(val)
        }
      }
      if stack.len() != 1 {
        return Err("Remaining untreated operands. Probably missing operator.".to_string());
      }
      return Ok(stack.pop().expect("expected a f32 value remaining in stack"));
    },
    Err(err) => Err(err)
  }
}

#[test]
fn it_adds() {
  let result = evaluate("1 2 +");
  assert_eq!(result.unwrap(), 3.0);
}

#[test]
fn it_substracts() {
  let result = evaluate("1 2 -");
  assert_eq!(result.unwrap(), -1.0);
}

#[test]
fn it_multiplies() {
  let result = evaluate("6 7 *");
  assert_eq!(result.unwrap(), 42.0);
}

#[test]
fn it_divides() {
  let result = evaluate("1 2 /");
  assert_eq!(result.unwrap(), 0.5);
}

#[test]
fn it_modulos() {
  let result = evaluate("4 2 %");
  assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn it_evaluates_complex_expressions() {
  // ((1+2) * 8 / (5-1) - 4 % 3) / 2
  let result = evaluate("1 2 + 8 * 5 1 - / 4 3 % - 2 /");
  assert_eq!(result.unwrap(), 2.5);
}

#[test]
fn it_allows_multiple_shitespaces() {
  let result = evaluate("1  2 +\t3 -");
  assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn it_fails_for_unsupported_characters() {
  let result = evaluate("1 2 t");
  assert_eq!(result.unwrap_err(), "Cannot parse operand \"t\"");
}

#[test]
fn it_fails_for_unsufficient_operands() {
  let result = evaluate("1 +");
  assert_eq!(result.unwrap_err(), "Unsufficient operands before operator");
}

#[test]
fn it_fails_for_unsufficient_operators() {
  let result = evaluate("1 2 3 +");
  assert_eq!(result.unwrap_err(),
    "Remaining untreated operands. Probably missing operator.");
}
