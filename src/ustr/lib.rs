#[crate_id = "ustr#0.1"];
#[crate_type = "lib"];
#[license = "MIT"];
#[feature(globs)];
#[allow(dead_code)];

extern mod encoding;
extern mod extra;

pub use ffi::*;
use ffi::{ icuuc, icuio };
use encoding::*;

// use std::libc::{c_char, c_int, c_void};
use std::ptr;
use std::str::raw::{from_c_str};

mod ffi;

pub struct UString {
  buf:  ~[UChar],      // UChar *ptr;
  str:  ~str,
  busy: u8             // unsigned char busy;
}

pub trait ToUString {
  fn to_u(&self) -> UString;
}

impl<'a> ToUString for &'a str {
  fn to_u(&self) -> UString {
    let mut buf = all::UTF_16LE.encode(self.to_owned(), EncodeReplace).unwrap();

    UString { buf: buf, str: self.to_owned(), busy: 0 }
  }
}

impl UString {
  pub fn to_str(&self) -> ~str {
    all::UTF_16LE.decode(self.buf, DecodeReplace).unwrap()
  }

  pub fn from_utf8(s: &str) -> *UChar {
    unsafe {
      let cap = s.char_len() * 2;
      let mut buf: ~[UChar] = std::vec::from_elem(cap, 0u8);// std::vec::with_capacity(cap);
      let pDestLength: *mut i32 = &mut 0;
      let pNumSubstitutions: *mut i32 = &mut 0;
      let pErrorCode: *mut UErrorCode = &mut ZERO_ERROR;

      let mut r = ptr::null();
      println!("{:?}", cap);
      
      s.with_c_str(|c_str| {
        r = icuuc::u_strFromUTF8WithSub_52(buf.as_mut_ptr(), cap as i32, pDestLength, c_str, -1, &SENTINEL, pErrorCode);   
        println!("{} {} {:?}", *pDestLength, *pNumSubstitutions, *pErrorCode);
        
      });
      

      println!("{:?}", buf);
      
      r
    }
  }

  pub fn new() -> UString {
    UString { buf: ~[], str: ~"", busy: 0}
  }

  pub fn length(&self) -> uint {
    self.buf.len()
  }

  pub fn strlen(&self) -> i32 {
    unsafe {
      icuuc::u_strlen_52(self.buf.as_ptr())
    }
  }

  pub fn printf(&self) {
    unsafe {
      icuio::u_printf_u_52(self.buf.as_ptr());
    }
  }

  fn error_name(code: UErrorCode) -> ~str {
    unsafe {
      std::str::raw::from_c_str(icuuc::u_errorName_52(code) as *i8)
    }
  }

  fn success(code: UErrorCode) -> bool {
    code == ZERO_ERROR
  }

  fn failure(code: UErrorCode) -> bool {
    code != ZERO_ERROR
  }

  pub fn upcase(&self) -> UString {
    println!("upcase {}", self.length());

    unsafe {
      let mut buf: ~[UChar] = std::vec::from_elem(self.length() + 1, 0u8); //std::vec::with_capacity(self.length() + 1);
      let error_code : *mut UErrorCode = &mut ZERO_ERROR;
      let locale = std::ptr::null();
      let i = icuuc::u_strToUpper_52(buf.as_mut_ptr(), buf.capacity() as i32, self.buf.as_ptr(), self.length() as i32, locale, error_code);

      if UString::failure(*error_code) {
        fail!(UString::error_name(*error_code))
      }

      println!("{} |{:?}| {}", i, buf, buf.capacity());
      
      // UString { ptr: buf.as_ptr() as *UChar, capa: self.len, len: self.len, busy: 0 }
      UString::new()
    } 
  }

  // #[inline]
  // fn reserve_exact(&mut self, n: uint) {
    // unsafe {
      // raw::as_owned_vec(self).reserve_exact(n)
    // }
  // }
}

