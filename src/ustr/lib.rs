#[crate_id = "ustr#0.1"];
#[crate_type = "lib"];
#[license = "MIT"];
#[feature(globs)];
#[allow(dead_code)];

extern mod extra;

pub use ffi::*;
use ffi::{ icuuc };

use std::fmt;

mod ffi;

pub struct UString {
  buf:  ~[UChar],      // UChar *ptr;
  busy: u8             // unsigned char busy;
}

pub trait ToUString {
  fn to_u(&self) -> UString;
}

impl<'a> ToUString for &'a str {
  fn to_u(&self) -> UString {
    let cap = self.len() * 2;
    let mut buf: ~[UChar] = std::vec::from_elem(cap, 0u8);

    let mut pDestLength = 0;
    let mut pNumSubstitutions: i32 = 0;
    let mut pErrorCode = ZERO_ERROR;

    unsafe {
      icuuc::u_strFromUTF8WithSub_52(buf.as_mut_ptr(), 
                                            buf.capacity() as i32, 
                                            &mut pDestLength, 
                                            self.as_bytes().as_ptr() as *i8,
                                            self.len() as i32,
                                            SENTINEL, 
                                            &mut pNumSubstitutions, 
                                            &mut pErrorCode);
      buf.set_len(pDestLength as uint);
    }
    
    UString { buf: buf, busy: 0 }
  }
}

impl<'a> std::fmt::Show for &'a UString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "{}", self.to_str())
  }
}

impl std::fmt::Show for UString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "{}", self.to_str())
  }
}

impl std::fmt::Show for ~UString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "{}", self.to_str())
  }
}

impl UString {
  pub fn to_str(&self) -> ~str {
    let mut buf: ~[u8] = std::vec::from_elem(self.buf.len() * 2, 0u8);
    let mut pDestLength = 0;
    let mut pNumSubstitutions: i32 = 0;
    let mut pErrorCode = ZERO_ERROR;

    unsafe {    
      icuuc::u_strToUTF8WithSub_52(buf.as_mut_ptr() as *mut i8,
                                   buf.capacity() as i32, 
                                   &mut pDestLength,
                                   self.buf.as_ptr(),
                                   -1, // length, requires 0 termination of src
                                   SENTINEL, 
                                   &mut pNumSubstitutions, 
                                   &mut pErrorCode);
      buf.set_len(pDestLength as uint);
      std::str::from_utf8_owned(buf).unwrap()
    }
  }

  pub fn new() -> UString {
    UString { buf: ~[], busy: 0}
  }

  //////////////////////////////////// PUBLIC API ////////////////////////////////////

  // Returns a new copy of UString with all uppercase letters replaced with their uppercase counterparts.
  pub fn upcase(&self) -> UString {
    let mut buf: ~[UChar] = std::vec::from_elem(self.buf.capacity() + 1, 0u8);
    let mut pDestLength = 0;
    unsafe {
      let mut pErrorCode = ZERO_ERROR;
      let locale = std::ptr::null();

      icuuc::u_strToUpper_52(buf.as_mut_ptr(),
                             buf.capacity() as i32, 
                             self.buf.as_ptr(), 
                             self.buf.len() as i32,
                             locale,
                             &mut pErrorCode);

      UString { buf: buf, busy: 0 }
    }
  }

  // Returns a new copy of UString with all lowercase letters replaced with their uppercase counterparts.
  pub fn downcase(&self) -> UString {
    let mut buf: ~[UChar] = std::vec::from_elem(self.buf.capacity() + 1, 0u8);
    let mut pDestLength = 0;

    unsafe {
      
      let mut pErrorCode = ZERO_ERROR;
      let locale = std::ptr::null();

      icuuc::u_strToLower_52(buf.as_mut_ptr(),
                             buf.capacity() as i32, 
                             self.buf.as_ptr(), 
                             self.buf.len() as i32,
                             locale,
                             &mut pErrorCode);
    }

    UString { buf: buf, busy: 0 }
  }

  pub fn length(&self) -> uint {
    ffi::strlen(&self.buf) as uint
  }
}

