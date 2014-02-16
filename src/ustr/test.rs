#[feature(globs)];

extern mod ustr;

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
fn test_length(){
  let u = "föobär".to_u();
  assert_eq!(u.length(), 6);

  let u = "".to_u();
  assert_eq!(u.length(), 0);
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
}

#[test]
fn test_fmt(){
  let u = "föobär".to_u();
  let s = format!("{}", u);
  assert_eq!(s, ~"föobär");
}

#[test]
fn test_upcase(){
  let u = "Föobär".to_u();
  assert_eq!(u.upcase().to_str(), ~"FÖOBÄR")
}

#[test]
fn test_downcase(){
  let u = "FÖOBÄR".to_u();
  assert_eq!(u.downcase().to_str(), ~"föobär")
}

#[test]
fn test_titleize(){
  let u = "In a hill there lives a hobbit".to_u();
  assert_eq!(u.titleize().to_str(), ~"In A Hill There Lives A Hobbit");
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
