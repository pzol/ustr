#[feature(globs)];

extern crate ustr;

use ustr::*;
use std::vec::{ Items };

struct Match {
  pos: uint,
  u: UString,
  rest: UString,
  count: uint
}

pub struct SplitsChar<'a> {
  iter: std::vec::Items<'a, u16>,
  break_pred: 'a |UChar| -> bool,
  finished: bool
}

type UCharVec<'a> = &'a [UChar];

pub struct Splits<'a> {
  slice: &'a [UChar],
  break_pred: 'a |&UCharVec| -> bool,
  finished: bool,
  count: uint,
  max: uint,
  pos: uint
}

trait Each<'a> {
  fn each_char(&'a self, break_pred: 'a |UChar| -> bool)   -> SplitsChar<'a>;
  fn each(&'a self, max: uint, break_pred: 'a |&UCharVec| -> bool) -> Splits<'a>;
  fn find(&'a self, other: &UString) -> Option<uint>; 
}

impl<'a> Each<'a> for UString {
  fn each_char(&'a self, break_pred: 'a |UChar| -> bool) -> SplitsChar<'a> {
    SplitsChar {
      iter: self.buf.iter(),
      break_pred: break_pred,
      finished: false
    }
  }

  fn each(&'a self, max: uint, break_pred: 'a |&UCharVec| -> bool) -> Splits<'a> {
    Splits {
      slice: self.buf.init(),
      break_pred: break_pred,
      finished: false,
      count: 1,
      max: max,
      pos: 0
    }
  }

  fn find(&'a self, other: &UString) -> Option<uint> {
    let pred = |v: &UCharVec| { let u = UString { buf: v.to_owned() }; u.starts_with(other) };

    let r = self.each(0, pred).next();
    println!("{} {} {}", r.get_ref().pos.clone(), r.get_ref().u, *other);
    
    match  r {
      Some(ref m) if m.rest.starts_with(other) => Some(m.pos),
      _                             => None
    }
  }
}

impl<'a> Iterator<UString> for SplitsChar<'a> {
  fn next(&mut self) -> Option<UString> {
    if self.finished { 
      return None }; 

    let mut word = ~[];

    loop {
      let next = self.iter.next();

      if next.is_none() {
        self.finished = true; 
        break;
      } else {
        let c = *next.unwrap();
        if (self.break_pred)(c) {
          if word.is_empty() {    // skip repeating breakers
            continue;
          } else {
            break;
          }
        }
        word.push(c);
      }
    }

    Some(UString { buf: word })
  }
}

impl<'a> Iterator<Match> for Splits<'a> {
  fn next(&mut self) -> Option<Match> {
    if self.finished {
      return None;
    }

    if self.max > 0 && self.count == self.max {
      self.finished = true;
      return Some(Match { count: self.count, pos: self.pos, u: UString { buf: self.slice.to_owned() }, rest: UString { buf: self.slice.to_owned() } });
    }

    let mut word = ~[];

    loop {
      if self.slice.is_empty() {
        self.finished = true; 
        break;
      } else {
        if (self.break_pred)(&self.slice) {
          if word.is_empty() {
            self.pos   = self.pos + 1;
            self.slice = self.slice.slice_from(1);
            continue;
          } else {
            break;
          }
        }
        word.push(self.slice[0]);
      }

      self.pos   = self.pos + 1;
      self.slice = self.slice.slice_from(1);
    }
    self.count = self.count + 1;
    Some(Match { count: self.count, pos: self.pos, u: UString { buf: word }, rest: UString { buf: self.slice.to_owned() } })
  }
}

#[test]
fn test_find(){
  let u = "foobar".to_u();
  let r = u.find(&"ob".to_u());
  assert_eq!(r, Some(2));

  let r = u.find(&"xxx".to_u());
  assert_eq!(r, None);
  
}

#[test]
fn test(){
  let u = "In   a   hole, there lives a: hobbit".to_u();
  println!("test |{}|", u);

  // let breaker = |token| {
  //   let breaker = " ,:\n".to_u();
  //   breaker.buf.contains(&token)
  // };

  // for word in u.each(breaker) {
  //   println!("word {}", word.inspect());
  // }

  let breaker = |v: &UCharVec| {
    let u = UString { buf: v.to_owned() };
    u.starts_with(&" ".to_u()) || u.buf[0] == 10u16
  };

  let mut words = ~[];

  for word in u.each(4, breaker) {
    words.push(word.u.clone());
    println!("word {}", word.u.to_str());
  }

  let expected = ~["In".to_u(), "a".to_u(), "hole,".to_u(), " there lives a: hobbi".to_u()];
  assert_eq!(words, expected);

  // fail!("bb");
}
