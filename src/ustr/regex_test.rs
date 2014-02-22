#[feature(globs)];

use ustring::*;
use regex::*;
// use regex::Regex;
mod lib;
mod ustring;
mod ffi;
mod regex;

#[test]
fn test_regex() {
  let b = "foobar".to_u().matches_str("^foo.*$");
  println!("{:?}", b);

  // assert!(("foobar").matches_str("^foo.*$").unwrap());

}
