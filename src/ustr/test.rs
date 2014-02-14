#[feature(globs)];

extern mod ustr;

use ustr::*;

#[test]
fn test_length(){
  let u = "föobär".to_u();
  assert_eq!(u.length(), 6);
}

#[test]
fn test_pile_of_poo(){
  let s = ~"💩";
  let u = s.to_u();
  assert_eq!(s, u.to_str());
}

#[test]
fn test_from_utf8(){
  let s = ~"föobär";
  let u = s.to_u();
  assert_eq!(s, u.to_str());
}

#[test]
fn test_fmt(){
  let u = ~"föobär".to_u();
  let s = format!("{}", u);
  assert_eq!(s, ~"föobär");
}

#[test]
fn test_upcase(){
  let u = ~"föobär".to_u();
  assert_eq!(u.upcase().to_str(), ~"FÖOBÄR")
}

#[test]
fn test_downcase(){
  let u = ~"FÖOBÄR".to_u();
  assert_eq!(u.downcase().to_str(), ~"föobär")
}
