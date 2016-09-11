fn main(expr: &str) -> i32 {
  return 0; // Let's return dummy values first.
}

#[test]
fn it_adds() {
  let result = main("+1 2");
  assert_eq!(result, 3);
}
