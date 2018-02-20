extern crate dalongdemo;
#[test]
fn first_test(){
  let result = dalongdemo::appdemo::add(2, 4);
  assert_eq!(result,6);
}