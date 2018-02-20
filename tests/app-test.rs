#[cfg(test)]
extern crate dalongdemo;
mod tests{
  use dalongdemo::appdemo;
  #[test]
  fn first_test(){
    let result = appdemo::userlogin();
    assert_eq!(result, "dalong")
  }
}