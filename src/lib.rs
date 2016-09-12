/// Evaluates an RPL expression.
///
/// # Examples
/// ````
/// extern crate rpn;
///
/// let result:f32 = rpn::evaluate("5 2 +").unwrap();
/// ````
///
/// # Errors
/// This function will return an error in case of bad expression:
///
/// - if it includes an unrecognized operator (recognized ones are +, -, * and /
/// - if it misses an operand (i.e. value)
/// - if it misses an operator
pub fn evaluate(expr: &str) -> Result<f32, &str> {
  let mut stack:Vec<f32> = Vec::new();
  for token in expr.split_whitespace() {
    let wrapped_operand = token.parse::<f32>();
    let is_operator = wrapped_operand.is_err();
    if is_operator {
      if stack.len() < 2 {
        return Err("Unsufficient operands before operator");
      }
      // xxxFlorent: Any way to destructure like this? Not sure of what I am
      // doing below
      // let [ operand1, operand2 ] = stack.drain((token.len() - 3)..).collect();
      let operand2 = stack.pop().expect("expected f32 values in stack");
      let operand1 = stack.pop().expect("expected f32 values in stack");
      let result = match token {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" => operand1 * operand2,
        "/" => operand1 / operand2,
        _ => return Err("Unsupported operator")
      };
      stack.push(result);
    } else {
      stack.push(wrapped_operand.unwrap());
    }
  }

  if stack.len() != 1 {
    return Err("Remaining untreated operands. Probably missing operator.");
  }
  return Ok(stack.pop().expect("expected a f32 value remaining in stack"));
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
fn it_evaluates_complex_expressions() {
  // ((1+2) * 8 / (5-1) - 1) / 2
  let result = evaluate("1 2 + 8 * 5 1 - / 1 - 2 /");
  assert_eq!(result.unwrap(), 2.5);
}

#[test]
fn it_allows_multiple_shitespaces() {
  let result = evaluate("1  2 +\t3 -");
  assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn it_panics_for_unsupported_characters() {
  let result = evaluate("1 2 t");
  assert_eq!(result.unwrap_err(), "Unsupported operator");
}

#[test]
fn it_panics_for_unsufficient_operands() {
  let result = evaluate("1 +");
  assert_eq!(result.unwrap_err(), "Unsufficient operands before operator");
}

#[test]
fn it_panics_for_unsufficient_operators() {
  let result = evaluate("1 2 3 +");
  assert_eq!(result.unwrap_err(),
    "Remaining untreated operands. Probably missing operator.");
}
