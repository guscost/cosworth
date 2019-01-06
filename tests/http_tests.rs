#[macro_use]
extern crate serde_json;
extern crate cosworth;
use cosworth::prelude::*;

#[test]
fn response_test() {
  let expected = Ok(Response {
    status: 200,
    headers: HeaderMap::new(),
    body: json!({"foo", "bar"})
  })
  assert_eq!(Response::new(200, json!({"foo": "bar"})), expected);
}
