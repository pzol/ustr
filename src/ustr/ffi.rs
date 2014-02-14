#[allow(ctypes, dead_code)];
#[feature(globs)];

use std::str;

pub type UBool     = u8;
pub type UProperty = int;
pub type UChar     = u8;
pub type UChar32   = i32;

#[deriving(Eq)]
pub enum UErrorCode {
  ZERO_ERROR = 0,
  BUFFER_OVERFLOW_ERROR = 15
}

pub static TRUE: u8  = 1u8;
pub static FALSE: u8 = 0u8;

pub static SENTINEL: UChar32 = -1; 

pub mod icuio {
  use super::*;

  #[link(name = "icuio")]
  extern "C" {
    pub fn u_printf_u_52(patternSpecification: *UChar, ...) -> i32;
  }
}

pub mod icuuc {
  use super::*;
  use std::libc::{ c_char };

  #[link(name = "icuuc")]
  extern "C" {
      pub fn u_strToUpper_52(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32;
      pub fn u_strToLower_52(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32;
      pub fn u_strlen_52(s: *UChar) -> i32;
      pub fn u_errorName_52(code: i32) -> *char;
      // http://icu-project.org/apiref/icu4c/ustring_8h.html#a0e9b7cd493c351804322ad1805fbe775
      pub fn u_strFromUTF8WithSub_52(dest: *mut UChar, destCapacity: i32, pDestLength: *mut i32, src: *c_char, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar;
      pub fn u_strToUTF8WithSub_52(dest: *mut c_char, destCapacity: i32, pDestLength: *mut i32, src: *UChar, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar;
  }
}

pub fn strlen(buf: &~[UChar]) -> i32 {
  unsafe {
    icuuc::u_strlen_52(buf.as_ptr())
  }
}

pub fn error_name(code: UErrorCode) -> ~str {
  unsafe {
    str::raw::from_c_str(icuuc::u_errorName_52(code as i32) as *i8)
  }
}

pub fn success(code: UErrorCode) -> bool {
  code == ZERO_ERROR
}

pub fn failure(code: UErrorCode) -> bool {
  code != ZERO_ERROR
}

fn printf(buf: ~[UChar]) {
  unsafe {
    icuio::u_printf_u_52(buf.as_ptr());
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::vec;
  use std::str;

  #[test]
  fn test_u_strFromUTF8WithSub_52(){

      let src = "föobär";
      let cap = src.len() * 2;
      let mut buf: ~[UChar] = vec::from_elem(cap, 0u8);

      let mut pDestLength = 0;
      let mut pNumSubstitutions: i32 = 0;
      let mut pErrorCode = ZERO_ERROR;

      unsafe {
        icuuc::u_strFromUTF8WithSub_52(buf.as_mut_ptr(), 
                                       buf.capacity() as i32, 
                                       &mut pDestLength, 
                                       src.as_bytes().as_ptr() as *i8,
                                       src.len() as i32,
                                       SENTINEL, 
                                       &mut pNumSubstitutions, 
                                       &mut pErrorCode);   

        let expected = ~[102u8, 0u8, 246u8, 0u8, 111u8, 0u8, 98u8, 0u8, 228u8, 0u8, 114u8, 0u8, 0u8, 0u8, 0u8, 0u8];

        assert_eq!(pDestLength, 6);
        assert_eq!(pNumSubstitutions, 0);
        assert_eq!(pErrorCode, ZERO_ERROR);
        assert_eq!(buf, expected);
        
      }
  }

  #[test]
  fn test_u_strToUTF8WithSub_52(){
    let src: ~[UChar] = ~[102u8, 0u8, 246u8, 0u8, 111u8, 0u8, 98u8, 0u8, 228u8, 0u8, 114u8, 0u8, 0u8, 0u8];
    let mut buf: ~[u8] = vec::from_elem(src.len() * 2, 0u8);
    let mut pDestLength = 0;
    let mut pNumSubstitutions: i32 = 0;
    let mut pErrorCode = ZERO_ERROR;

    unsafe {    
      icuio::u_printf_u_52(src.as_ptr());

      icuuc::u_strToUTF8WithSub_52(buf.as_mut_ptr() as *mut i8,
                                    buf.capacity() as i32, 
                                    &mut pDestLength,
                                    src.as_ptr(),
                                    -1, // length, requires 0 termination of src
                                    SENTINEL, 
                                    &mut pNumSubstitutions, 
                                    &mut pErrorCode);  

      let expected = ~"föobär";
      buf.set_len(pDestLength as uint);
      let dest = str::from_utf8_owned(buf).unwrap();
      assert_eq!(dest.to_owned(), expected);
    }
  }
}
