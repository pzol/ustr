#[feature(globs)];

extern crate ustr;
use ustr::{ UString, UChar, ToUString };

// use super::{ UString, ToUString };

type UCharVec<'a> = &'a [UChar];

#[deriving(Eq)]
enum TokenType {
  ValueToken,
  SplitToken,
  FinishedToken
}

pub struct TokenIter<'a> {
  buf: &'a [UChar],
  pos: uint,
  last_pos: uint,
  end: uint,
  pred: 'a |&UCharVec| -> Option<uint>,
  state: TokenType
}

trait Find<'a> {
  fn token_iter(&'a self, predicate: 'a |&UCharVec| -> Option<uint>) -> TokenIter<'a>;

  // naive approach unicode wise
  fn word_breaker() -> 'a |&UCharVec| -> Option<uint> {
    |buf: &UCharVec| -> Option<uint> {
      let mut count = 0;
      while [' ' as u16, ',' as u16].contains(&buf[count]) {
        count += 1;
      }

      if count > 0 {
        Some(count)
      } else {
        None
      }
    }
  }

  fn split_iter(&'a self, predicate: 'a |&UCharVec| -> Option<uint>) -> std::iter::FilterMap<'a, TokenMatch, ustr::UString, TokenIter<'a>> {
    self.token_iter(predicate).
      filter_map(|m| {
        if m.token == ValueToken && m.str.len() > 0 {
          Some(m.str)
        } else {
          None
        }
      })
  }

  fn index(&'a self, predicate: 'a |&UCharVec| -> Option<uint>) -> Option<uint> {
    match self.token_iter(predicate).next() {
      None    => None,
      Some(m) => Some(m.start)
    }
  }
}

impl<'a> Find<'a> for UString {
  fn token_iter(&'a self, predicate: 'a |&UCharVec| -> Option<uint>) -> TokenIter<'a> {
    TokenIter {
      buf: self.buf,
      pred: predicate,
      pos: 0,
      last_pos: 0,
      end: self.buf.len(),
      state: ValueToken
    }
  }
}

#[deriving(Eq)]
struct TokenMatch {
  start: uint,
  str:   UString,
  token: TokenType
}

impl<'a> Iterator<TokenMatch> for TokenIter<'a> {
  fn next(&mut self) -> Option<TokenMatch> {
    match self.state {
      ValueToken => {
        while self.pos < self.end {
          let ptr = self.buf.slice_from(self.pos);

          match (self.pred)(&ptr) {
            None    => (),
            Some(l) => {
              let r = Some(TokenMatch { start: self.last_pos, token: self.state, str: UString { buf: self.buf.slice(self.last_pos, self.pos).to_owned() } });
              self.state = SplitToken;
              self.last_pos = self.pos;
              self.pos      = self.last_pos + l;
              return r;
            }
          }

          self.pos = self.pos + 1;
        }

        // reached the end of the buffer
        let r = Some(TokenMatch { start: self.last_pos, token: self.state, str: UString { buf: self.buf.slice(self.last_pos, self.pos).to_owned() } });        
        self.state = FinishedToken;
        return r;
      },

      SplitToken => {
        let r = Some(TokenMatch { start: self.last_pos, token: self.state, str: UString { buf: self.buf.slice(self.last_pos, self.pos).to_owned() } });
        self.state      = ValueToken;
        self.last_pos   = self.pos;
        return r;
      },

      FinishedToken => { return None; }
    }
  }
}

#[test]
fn test_token_iter(){
  let u = "foo,  bar, baz,  bar, aaa".to_u();
  let breaker = |buf: &UCharVec| {
    let sep = ", ".to_u().buf;
    for (idx, &c) in sep.iter().enumerate() {
      if buf[idx] != c {
        return None
      }
    }
    Some(2)
  };

  let mut it = u.token_iter(breaker);
  assert_eq!(it.next(), Some(TokenMatch { start: 0,  token: ValueToken, str: "foo".to_u()  }));
  assert_eq!(it.next(), Some(TokenMatch { start: 3,  token: SplitToken, str: ", ".to_u()   }));
  assert_eq!(it.next(), Some(TokenMatch { start: 5,  token: ValueToken, str: " bar".to_u() }));
  assert_eq!(it.next(), Some(TokenMatch { start: 9,  token: SplitToken, str: ", ".to_u()   }));
  assert_eq!(it.next(), Some(TokenMatch { start: 11, token: ValueToken, str: "baz".to_u()  }));
  assert_eq!(it.next(), Some(TokenMatch { start: 14, token: SplitToken, str: ", ".to_u()   }));
  assert_eq!(it.next(), Some(TokenMatch { start: 16, token: ValueToken, str: " bar".to_u() }));
  assert_eq!(it.next(), Some(TokenMatch { start: 20, token: SplitToken, str: ", ".to_u()   }));
  assert_eq!(it.next(), Some(TokenMatch { start: 22, token: ValueToken, str: "aaa".to_u()  }));
  assert_eq!(it.next(), None);
}

#[test]
fn test_split(){
  let u = "foo    bar    baz".to_u();
  let breaker = |buf: &UCharVec| {
    if buf[0] == 32u16 {
      Some(1)
    } else {
      None
    }
  };

  let breaker: |&UCharVec| -> Option<uint> = Find::word_breaker();
  let words: ~[UString] = u.split_iter(breaker).collect();
  assert_eq!(words, ~["foo".to_u(), "bar".to_u(), "baz".to_u()]);
}
