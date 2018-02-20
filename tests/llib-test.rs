extern crate dalongdemo;
#[test]
fn result_test() {
    let result = dalongdemo::appdemo::userlogin();
    assert_eq!(result, "dalong")
}
#[test]
fn addtest(){
    let result =dalongdemo::appdemo::add(1,3);
    assert_eq!(result,4);
}

