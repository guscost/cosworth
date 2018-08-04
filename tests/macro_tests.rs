#[macro_use]
extern crate cosworth;

#[test]
fn macro_test() {
    let expected = format!("Hello, cosworth v{}!", cosworth::VERSION);
    assert_eq!(hello!(), expected);
}
