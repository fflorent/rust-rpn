fn main(expr: &str) -> i32 {
  let mut result = 0;
  let mut stack:Vec<i32> = Vec::new();
  for token in expr.split_whitespace() {
    let wrapped_operand = token.parse::<i32>();
    let is_operator = wrapped_operand.is_err();
    if is_operator {
      // xxxFlorent: Any way to destructure like this? Not sure of what I am doing below
      // let [ operand1, operand2 ] = stack.drain((token.len() - 3)..).collect();
      let operand1 = stack.pop().expect("expected i32 values only in stack");
      let operand2 = stack.pop().expect("expected i32 values only in stack");
      result += match token {
        "+" => operand1 + operand2,
        _ => panic!("unsupported operator")
      }
    } else {
      stack.push(wrapped_operand.unwrap());
    }
  }
  return result;
}

#[test]
fn it_adds() {
  let result = main("1 2 +");
  assert_eq!(result, 3);
}

#[test]
#[should_panic]
fn it_panics_for_unsupported_operators() {
  main("1 2 =");
  // xxxFlorent: How to test the panic message is right?
}
