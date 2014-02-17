#[feature(globs)];

extern crate ustr;

use ustr::*;

static FOOBAR: &'static str = "föobär";

#[test]
fn test_to_str(){
  let u = FOOBAR.to_u();
  assert_eq!(u.to_str(), ~"föobär");
}

#[test]
fn test_to_str_empty(){
  let u = "".to_u();
  assert_eq!(u.to_str(), ~"");
}

#[test]
fn test_is_empty(){
  let u = "".to_u();
  assert!(u.is_empty());

  let u = "foo".to_u();
  assert!(!u.is_empty());
}

#[test]
fn test_len(){
  let u = "föobär".to_u();
  assert_eq!(u.len(), 6);

  let u = "".to_u();
  assert_eq!(u.len(), 0);
}

#[test]
fn test_pile_of_poo(){
  let s = "💩";
  let u = s.to_u();
  assert_eq!(u.to_str(), s.to_owned());
}

#[test]
fn test_from_utf8(){
  let s = "föobär";
  let u = s.to_u();
  assert_eq!(u.to_str(), s.to_owned());

  let e = "Hello, 世界!";
  assert_eq!(e.to_u().to_str(), e.to_owned());
}

#[test]
fn test_fmt(){
  let u = "föobär".to_u();
  let s = format!("{}", u);
  assert_eq!(s, ~"föobär");
}

#[test]
fn test_to_upper(){
  let u = "Föobär".to_u();
  assert_eq!(u.to_upper().to_str(), ~"FÖOBÄR");

  let e = "łódź".to_u();
  assert_eq!(e.to_upper().to_str(), ~"ŁÓDŹ");
}

#[test]
fn test_to_lower(){
  let u = "FÖOBÄR".to_u();
  assert_eq!(u.to_lower().to_str(), ~"föobär")
}

#[test]
fn test_to_title(){
  let u = "In a hill there lives a hobbit".to_u();
  assert_eq!(u.to_title().to_str(), ~"In A Hill There Lives A Hobbit");
}

#[test]
fn test_to_capital(){
  let u = "biLBo".to_u();
  assert_eq!(u.to_capital().to_str(), ~"Bilbo");
}

#[test]
fn test_inspect(){
  let u = "foo".to_u();
  assert_eq!(u.inspect(), ~"UString {\"foo\", buf: ~[102u16, 111u16, 111u16]}");
}

#[test]
fn test_from_bytes(){
  let bytes = ~[102u16, 111u16, 111u16, 0u16];
  let u = UString::from_bytes(bytes.as_ptr());
  assert_eq!(u.to_str(), ~"foo");
}

#[test]
fn test_add(){
  let u1 = "foo".to_u();
  let u2 = "bar".to_u();
  
  let combined = u1 + u2;
  assert_eq!(combined.to_str(), ~"foobar");
}


#[test]
fn test_concat(){
  let u1 = "foo".to_u();
  let u2 = "bar".to_u();
  
  let combined = u1.concat(&u2);
  assert_eq!(combined.to_str(), ~"foobar");
}

#[test]
fn test_split(){
  let u = "foo bar".to_u();
  let words = u.split(" ".to_u());
  assert_eq!(words, ~["foo".to_u(), "bar".to_u()]);

  let u = "In a hill, there lives:   a hobbit".to_u();
  let words = u.split(" ,:".to_u());
  assert_eq!(words, (~["In", "a", "hill", "there", "lives", "a", "hobbit"]).map(|w| w.to_u()));
}

#[test]
fn test_split_empty(){
  let u = "".to_u();
  let words = u.split(" ".to_u());
  assert_eq!(words, ~[]);
}

#[test]
fn test_join(){
  let words = ~["foo".to_u(), "bar".to_u()];
  let u = words.join(&" ".to_u());
  assert_eq!(u.to_str(), ~"foo bar");
}

#[test]
fn test_join_empty(){
  let words: ~[UString] = ~[];
  let u = words.join(&" ".to_u());
  assert_eq!(u.to_str(), ~"");
}

#[test]
fn test_start_with(){
  let u = "foobar".to_u();
  assert!(u.starts_with(&"foo".to_u()));
  assert!(!u.starts_with(&"bar".to_u()));
}

#[test]
fn test_ends_with(){
  let u = "foobar".to_u();
  assert!(u.ends_with(&"bar".to_u()));
  assert!(!u.ends_with(&"foo".to_u()));
}

#[test]
fn test_slice_len(){
  let u = "foobar".to_u();
  assert_eq!(u.slice_len(0, 3), "foo".to_u());
  assert_eq!(u.slice_len(3, 99), "bar".to_u());
}

#[test]
fn test_slice(){
  let u = "föobar".to_u();
  assert_eq!(u.slice(0, 2),    "föo".to_u());
  assert_eq!(u.slice(3, 2),    "".to_u());
  assert_eq!(u.slice(3, 99),   "bar".to_u());
  assert_eq!(u.slice(-3, -1),  "bar".to_u());
  assert_eq!(u.slice(-99, -1), "föobar".to_u());
}

#[test]
fn test_to_i(){
  assert_eq!("1".to_u().to_i(), 1);
  assert_eq!("x".to_u().to_i(), 0);
}

#[test]
fn test_to_f(){
  assert_eq!("1.2".to_u().to_f(), 1.2f32);
  assert_eq!("x".to_u().to_f(), 0f32);
}

#[test]
fn test_to_owned(){
  let u = &"föobär".to_u();
  assert_eq!("föobär".to_u(), u.to_owned());
}
