fn main(expr: &str) -> f32 {
  let mut stack:Vec<f32> = Vec::new();
  for token in expr.split_whitespace() {
    let wrapped_operand = token.parse::<f32>();
    let is_operator = wrapped_operand.is_err();
    if is_operator {
      if stack.len() < 2 {
        panic!("Unsufficient operands before operator");
      }
      // xxxFlorent: Any way to destructure like this? Not sure of what I am doing below
      // let [ operand1, operand2 ] = stack.drain((token.len() - 3)..).collect();
      let operand2 = stack.pop().expect("expected f32 values only in stack");
      let operand1 = stack.pop().expect("expected f32 values only in stack");
      let result = match token {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" => operand1 * operand2,
        "/" => operand1 / operand2,
        _ => panic!("Unsupported operator")
      };
      stack.push(result);
    } else {
      stack.push(wrapped_operand.unwrap());
    }
  }

  if stack.len() != 1 {
    panic!("Remaining untreated operands. Probably missing operator.");
  }
  return stack.pop().expect("expected a f32 value remaining in stack");
}

#[test]
fn it_adds() {
  let result = main("1 2 +");
  assert_eq!(result, 3.0);
}

#[test]
fn it_substracts() {
  let result = main("1 2 -");
  assert_eq!(result, -1.0);
}

#[test]
fn it_multiplies() {
  let result = main("6 7 *");
  assert_eq!(result, 42.0);
}

#[test]
fn it_divides() {
  let result = main("1 2 /");
  assert_eq!(result, 0.5);
}

#[test]
fn it_calculates_complex_expressions() {
  // ((1+2) * 8 / (5-1) - 1) / 2
  let result = main("1 2 + 8 * 5 1 - / 1 - 2 /");
  assert_eq!(result, 2.5);
}

#[test]
fn it_allows_multiple_shitespaces() {
  let result = main("1  2 +\t3 -");
  assert_eq!(result, 0.0);
}

#[test]
#[should_panic]
fn it_panics_for_unsupported_characters() {
  main("1 2 t");
  // xxxFlorent: How to test the panic message is right?
}

#[test]
#[should_panic]
fn it_panics_for_unsufficient_operands() {
  main("1 +");
  // xxxFlorent: How to test the panic message is right?
}

#[test]
#[should_panic]
fn it_panics_for_unsufficient_operators() {
  main("1 2 3 +");
  // xxxFlorent: How to test the panic message is right?
}
