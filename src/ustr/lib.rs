#[crate_id = "ustr#0.1"];
#[crate_type = "lib"];
#[license = "MIT"];
#[feature(globs)];
#[allow(dead_code)];

extern mod extra;

pub use ffi::*;
use std::{ fmt, ptr, vec, str };
mod ffi;

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

  pub fn as_ptr(&self) -> *UChar {
    self.buf.as_ptr()
  } 

  //////////////////////////////////// PUBLIC API ////////////////////////////////////
  pub fn inspect(&self) -> ~str {
    format!("UString \\{\"{:s}\", buf: {:?}\\}", self.to_str(), self.buf)
  }

  pub fn to_owned(&self) -> UString {
    UString { buf: self.buf.to_owned() }
  }

  pub fn concat(&self, other: &UString) -> UString {
    UString { buf: self.buf + other.buf }
  }

  pub fn concat_str(&self, other: &str) -> UString {
    self.concat(&other.to_u())
  }

  pub fn chars<'a>(&'a self) -> vec::Items<'a, UChar> {
    self.buf.iter()
  }

  // Returns a new copy of UString with all uppercase letters replaced with their uppercase counterparts.
  pub fn upcase(&self) -> UString {
    let buf = self.buf.map(|c| ffi::to_upper(*c as UChar32) as UChar);
    UString { buf: buf }
  }

  // Returns a new copy of UString with all lowercase letters replaced with their uppercase counterparts.
  pub fn downcase(&self) -> UString {
    let buf = self.buf.map(|c| ffi::to_lower(*c as UChar32) as UChar);
    UString { buf: buf }
  }

  pub fn titleize(&self) -> UString {
    let mut buf: ~[UChar] = vec::from_elem(self.buf.len() + 1, 0u16);
    let dummy = 0;
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

  pub fn len(&self) -> uint {
    self.buf.len()
  }

  fn null_locale() -> *i8 { 
    "".as_bytes().as_ptr() as *i8
  }

  fn strlen(&self) -> uint {
    ffi::strlen(self.as_ptr()) as uint
  }
}

