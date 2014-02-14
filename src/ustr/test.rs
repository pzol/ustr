#[feature(globs)];

extern mod encoding;
extern mod ustr;

use ustr::*;
use encoding::*;

// #[test]
// fn test_to_u(){
//   let u = "foobar\n".to_u().upcase();
//   u.printf();
// }

// #[test]
// fn test_printf(){
//   let s = "föobär";
//   let u = s.to_u();

//   u.printf();

//   // u.upcase().printf();
// }

// #[test]
// fn test_convert(){
//   let s = "föobär";
//   let u = s.to_u();
//   assert_eq!(s.to_owned(), u.to_str());
// }

#[test]
fn test_strlen(){
  let u = "föobär".to_u();
  assert_eq!(u.strlen(), 6);
}

// #[test]
// fn ints(){
//   let i1: uint = 10;
//   let i2: i32  = 10;
//   assert_eq!(i1, i2 as uint);
// }

#[test]
fn test_from_utf8(){
  let s = "föobär";
  let u: *UChar = UString::from_utf8(s);
  
  // unsafe {
  //   u_printf_u_52(u);
  // }
}
