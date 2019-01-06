#[macro_use]
extern crate serde_json;
extern crate bytes;
extern crate cosworth;

use cosworth::prelude::*;


#[test]
fn response_test() {
  let expected = Response {
    status: 200,
    headers: HeaderMap::new(),
    body: bytes::Bytes::from("{\"foo\":\"bar\"}")
  };
  assert_eq!(Response::new(200, json!({"foo": "bar"})).expect("Invalid Response"), expected);
}
