#[crate_id = "ustr#0.1"];
#[crate_type = "lib"];
#[license = "MIT"];
#[feature(globs)];
#[allow(dead_code)];

extern crate extra;

pub use ffi::*;
use std::{ fmt, ptr, vec, str };
mod ffi;
mod regex;

#[deriving(Eq, Clone)]
pub struct UString {
  buf:  ~[UChar]      // UChar *ptr;
}

//////////////////////////////////// Traits ////////////////////////////////////

pub trait ToUString {
  fn to_u(&self) -> UString;
}

pub trait UJoin {
  fn join(&self, delim: &UString) -> UString;
}

impl UJoin for ~[UString] {
  fn join(&self, delim: &UString) -> UString {
    let mut it = self.iter();

    match it.next() {
      None    => UString::new(),
      Some(u) => {
        let mut result = u.to_owned();
        for word in it {
          result = result.concat(delim).concat(word);
        }
        result
      }

    }
  }
}

/// Convert a `~str` to a UString
///     ~"foobar".to_u()
impl<'a> ToUString for &'a str {
  fn to_u(&self) -> UString {
    if self.len() == 0 {
      return UString::new();
    }

    let cap = self.len() * 2;
    let mut buf: ~[UChar] = vec::from_elem(cap, 0u16);

    let mut pDestLength = 0;
    let mut pNumSubstitutions: i32 = 0;
    let mut error_code = ZERO_ERROR;

    ffi::strFromUTF8WithSub(buf.as_mut_ptr(), 
                            buf.capacity() as i32, 
                            &mut pDestLength, 
                            self.as_bytes().as_ptr() as *i8,
                            self.len() as i32,
                            SENTINEL, 
                            &mut pNumSubstitutions, 
                            &mut error_code);

    assert!(success(error_code), ffi::error_name(error_code));
    
    unsafe { buf.set_len(pDestLength as uint); }
    
    UString { buf: buf }
  }
}

impl fmt::Show for UString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "{}", self.to_str())
  }
}

impl Add<UString, UString> for UString {
    fn add(&self, other: &UString) -> UString {
      self.concat(other)
    }
}

impl ToStr for UString {
  /// Convert a UString to a `~str`
  /// 
  ///     "foobar".to_u().to_str()
  fn to_str(&self) -> ~str {
    let mut buf: ~[u8] = vec::from_elem(self.strlen() * 2, 0u8);
    let mut pDestLength = 0;
    let mut pNumSubstitutions: i32 = 0;
    let mut error_code = ZERO_ERROR;

    ffi::strToUTF8WithSub(buf.as_mut_ptr() as *mut i8,
                          buf.capacity() as i32, 
                          &mut pDestLength,
                          self.buf.as_ptr(),
                          self.buf.len() as i32, // length, requires 0 termination of src
                          SENTINEL, 
                          &mut pNumSubstitutions, 
                          &mut error_code);
      unsafe { buf.set_len(pDestLength as uint); }
      str::from_utf8_owned(buf).unwrap_or(~"")
  }
}

impl UString {
  pub fn new() -> UString {
    UString { buf: ~[]}
  }

  #[inline]
  pub fn from_bytes(src: *UChar) -> UString {
    let len = ffi::strlen(src);
    let buf = unsafe { vec::raw::from_buf_raw(src, len as uint) };
    
    UString { buf: buf }
  }

  #[inline]
  pub fn as_ptr(&self) -> *UChar {
    self.buf.as_ptr()
  } 

  //////////////////////////////////// PUBLIC API ////////////////////////////////////
  /// Return a string representation of an UString for debugging
  pub fn inspect(&self) -> ~str {
    format!("UString \\{\"{:s}\", buf: {:?}\\}", self.to_str(), self.buf)
  }

  pub fn to_owned(&self) -> UString {
    UString { buf: self.buf.to_owned() }
  }

  /// Return a new string with both of them concatenated
  pub fn concat(&self, other: &UString) -> UString {
    UString { buf: self.buf + other.buf }
  }

  pub fn concat_str(&self, other: &str) -> UString {
    self.concat(&other.to_u())
  }

  // pub fn contains(&self, other: &UString) -> bool {
  //   true
  // }

  /// Returns true if the string is empty
  pub fn is_empty(&self) -> bool {
    self.buf.is_empty()
  }

  pub fn chars<'a>(&'a self) -> vec::Items<'a, UChar> {
    self.buf.iter()
  }

  /// Returns a new copy of UString with the first letters upper case, all the rest lower case.
  pub fn to_capital(&self) -> UString {
    let mut buf = self.to_lower().buf;
    buf[0] = ffi::to_upper(buf[0] as i32) as UChar;
    UString { buf: buf }
  }

  /// Returns a new copy of UString with all upper case letters replaced with their uppercase counterparts.
  pub fn to_upper(&self) -> UString {
    let buf = self.buf.map(|c| ffi::to_upper(*c as UChar32) as UChar);
    UString { buf: buf }
  }

  /// Returns a new copy of UString with all lower case letters replaced with their uppercase counterparts.
  pub fn to_lower(&self) -> UString {
    let buf = self.buf.map(|c| ffi::to_lower(*c as UChar32) as UChar);
    UString { buf: buf }
  }

  /// Returns a new copy of UString with all first letters of a word in upper case, all others lower case.
  pub fn to_title(&self) -> UString {
    let mut buf: ~[UChar] = vec::from_elem(self.buf.len() + 1, 0u16);
    let mut error_code = ZERO_ERROR;

    ffi::strToTitle(buf.as_mut_ptr(),
                    buf.capacity() as i32, 
                    self.buf.as_ptr(), 
                    self.buf.len() as i32,
                    ptr::null(), // break iterator
                    UString::null_locale(), // locale
                    &mut error_code);

    assert!(success(error_code), ffi::error_name(error_code));
    unsafe { buf.set_len(self.buf.len()) };
    UString { buf: buf }
  }

  pub fn split(&self, delim: UString) -> ~[UString] {
    let mut words = ~[];
    let mut word  = ~[];

    for c in self.buf.iter() {
      if delim.buf.contains(c) {
        if word.len() > 0 {
          words.push(UString { buf: word.clone() });
          word = ~[];
        }
      } else {
        word.push(*c);
      }
    }

    if word.len() > 0 {
      words.push(UString { buf: word });
    }

    words
  }

  /// Returns true if the UString starts with the given prefix
  pub fn starts_with(&self, prefix: &UString) -> bool {
    self.buf.starts_with(prefix.buf)
  }

  /// Returns true if the UString ends with the given suffix
  pub fn ends_with(&self, suffix: &UString) -> bool {
    self.buf.ends_with(suffix.buf)
  }

  /// Returns a new UString from position start with the given length.
  pub fn slice_len(&self, start: uint, length: uint) -> UString {
    let it  = self.buf.iter().skip(start).take(length);
    let buf = it.map(|e| e.clone()).to_owned_vec();
    UString { buf: buf }
  }

  fn slice_pos(&self, pos: int, len: uint) -> uint {
    let ilen = len as int;
    let result = match pos {
          p if p < 0 && ilen + p < 0 => 0,
          p if p < 0                => ilen + p,
          p                         => p
    } as uint;
    result
  }

  /// Return a new UString containing the chars from start to end. The first char is 0.
  /// Negative start or end mean the position from the end of the string, -1 is the last character
  /// examples:
  ///
  ///     "foobar".to_u().slice(0, 2)   // => "foo"  
  ///     "foobar".to_u().slice(-3, -1) // => "bar"
  ///
  /// Out of bounds positions will use the start or end respectively and yield no error.
  pub fn slice(&self, start: int, end: int) -> UString {
    let rstart = self.slice_pos(start, self.len());
    let rend   = self.slice_pos(end, self.len());

    let mut pos = rstart;
    let it  = self.buf.iter().skip(rstart).take_while(|_| { pos = pos + 1; pos -1 <= rend } );
    let buf = it.map(|e| e.clone()).to_owned_vec();
    UString { buf: buf }
  }

  /// Returns the length (of UTF16 chars)
  pub fn len(&self) -> uint {
    self.buf.len()
  }

  /// Convert to int
  pub fn to_i(&self) -> int {
    from_str(self.to_str()).unwrap_or(0)
  }

  /// Convert to f32
  pub fn to_f(&self) -> f32 {
    from_str(self.to_str()).unwrap_or(0f32)
  }

  fn null_locale() -> *i8 { 
    "".as_bytes().as_ptr() as *i8
  }

  fn strlen(&self) -> uint {
    ffi::strlen(self.as_ptr()) as uint
  }
}

