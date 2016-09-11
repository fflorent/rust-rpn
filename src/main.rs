fn main(expr: &str) -> i32 {
  let result = 0;
  for c in expr.chars() {
    match c {
      '0' ... '9' => println!("number"),
      ' ' => println!("space"),
      '+' => println!("plus"),
      _ => panic!("unsupported character")
    }
  }
  return result;
}

#[test]
fn it_adds() {
  let result = main("+1 2");
  assert_eq!(result, 3);
}
