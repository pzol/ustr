#[allow(ctypes, dead_code)];
#[feature(globs)];

use std::str;
use std::libc::{ c_char };


pub type UChar     = u16;
pub type UChar32   = i32;

#[deriving(Eq)]
pub enum UErrorCode {
  ZERO_ERROR = 0,
  BUFFER_OVERFLOW_ERROR = 15
}

pub struct UBreakIterator;

pub enum UBreakIteratorType {
  UBRK_CHARACTER,
  UBRK_WORD,
  UBRK_LINE,
  UBRK_SENTENCE,
  UBRK_TITLE
}

pub static TRUE: u8  = 1u8;
pub static FALSE: u8 = 0u8;

pub static SENTINEL: UChar32 = -1; 

mod icuio {
  use super::*;

  #[link(name = "icuio")]
  extern "C" {
    pub fn u_printf_u_52(patternSpecification: *UChar, ...) -> i32;
  }
}

mod icui18n {
  use super::*;

  struct URegularExpression;

  #[link(name = "icui18n")]
  extern "C" {
    pub fn uregex_split_52(regexp: *URegularExpression, destBuf: *mut UChar, destCapacity: i32, requiredCapacity: *i32, destFields: &[*UChar], destFieldsCapacity: i32, status: *mut UErrorCode) -> i32;
  }
}

mod icuuc {
  use super::*;
  use std::libc::{ c_char };

  #[link(name = "icuuc")]
  extern "C" {
      // http://icu-project.org/apiref/icu4c/ustring_8h.html
      pub fn u_strToUpper_52(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32;
      pub fn u_strToLower_52(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32;
      pub fn u_strlen_52(s: *UChar) -> i32;
      pub fn u_errorName_52(code: i32) -> *char;
      pub fn u_strFromUTF8WithSub_52(dest: *mut UChar, destCapacity: i32, pDestLength: *mut i32, src: *c_char, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar;
      pub fn u_strToUTF8WithSub_52(dest: *mut c_char, destCapacity: i32, pDestLength: *mut i32, src: *UChar, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar;
      pub fn u_strToTitle_52(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, titleIter: *UBreakIterator, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32;
      pub fn u_strcat_52(dest: *mut UChar, src: *UChar) -> *mut UChar;

      pub fn u_strtok_r_52(src: *mut UChar, delim: *UChar, saveState: *mut *mut UChar) -> *UChar;
  }
}

// Adapters to the ICUUC versions, in case their signature changes. Also to hide the u_.*_52 and unsafe calls
#[inline]
pub fn strtok_r(src: *mut UChar, delim: *UChar, saveState: *mut *mut UChar) -> *UChar {
  unsafe {
    icuuc::u_strtok_r_52(src, delim, saveState)
  }
}

#[inline]
pub fn strFromUTF8WithSub(dest: *mut UChar, destCapacity: i32, pDestLength: *mut i32, src: *c_char, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar {
  unsafe {
    icuuc::u_strFromUTF8WithSub_52(dest, destCapacity, pDestLength, src, srcLength, subChar, pNumSubstitutions, pErrorCode)
  }
}

#[inline]
pub fn strToUTF8WithSub(dest: *mut c_char, destCapacity: i32, pDestLength: *mut i32, src: *UChar, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar {
  unsafe {
    icuuc::u_strToUTF8WithSub_52(dest, destCapacity, pDestLength, src, srcLength, subChar, pNumSubstitutions, pErrorCode)
  }
}

#[inline]
pub fn strToLower(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32 {
  unsafe {
    icuuc::u_strToLower_52(dest, destCapacity, src, srcLength, locale, pErrorCode)
  }
}

#[inline]
pub fn strToUpper(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32 {
  unsafe {
    icuuc::u_strToUpper_52(dest, destCapacity, src, srcLength, locale, pErrorCode)
  }
}

#[inline]
pub fn strcat(dest: *mut UChar, src: *UChar) -> *mut UChar {
  unsafe {
    icuuc::u_strcat_52(dest, src)
  }
}

#[inline]
pub fn strToTitle(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, titleIter: *UBreakIterator, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32 {
  unsafe {
    icuuc::u_strToTitle_52(dest, destCapacity, src, srcLength, titleIter, locale, pErrorCode)
  }
}

#[inline]
pub fn strlen(buf: *UChar) -> i32 {
  unsafe {
    icuuc::u_strlen_52(buf)
  }
}

#[inline]
pub fn error_name(code: UErrorCode) -> ~str {
  unsafe {
    str::raw::from_c_str(icuuc::u_errorName_52(code as i32) as *i8)
  }
}

#[inline]
pub fn success(code: UErrorCode) -> bool {
  code == ZERO_ERROR
}

#[inline]
pub fn failure(code: UErrorCode) -> bool {
  code != ZERO_ERROR
}

#[inline]
pub fn printf(buf: *UChar) {
  unsafe {
    icuio::u_printf_u_52(buf);
  }
}
