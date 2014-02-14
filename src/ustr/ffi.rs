#[allow(ctypes, dead_code)];
#[feature(globs)];

use std::vec;

pub type UBool     = u8;
pub type UProperty = int;
pub type UChar     = u8;
pub type UChar32   = i32;
// pub type UErrorCode = int;

#[deriving(Eq)]
pub enum UErrorCode {
  ZERO_ERROR = 0
}

pub static TRUE: u8  = 1u8;
pub static FALSE: u8 = 0u8;

pub static SENTINEL: UChar32 = -1; 

pub mod icuio {
  use super::*;

  #[link(name = "icuio")]
  extern "C" {
    pub fn u_printf_u_52(patternSpecification: *UChar) -> i32;
  }
}

pub mod icuuc {
  use super::*;
  use std::libc::{c_char };

  #[link(name = "icuuc")]
  extern "C" {
      pub fn u_strToUpper_52(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32;
      pub fn u_strlen_52(s: *UChar) -> i32;
      pub fn u_errorName_52(code: UErrorCode) -> *char;
      pub fn u_strFromUTF8WithSub_52(dest: *mut UChar, destCapacity: i32, pDestLength: *mut i32, src: *c_char, subChar: UChar32, pNumSubstitutions: *i32, pErrorCode: *mut UErrorCode) -> *UChar;
  }
}

mod icucore {
  use super::*;
  use std::libc::{c_char };

  #[link(name = "icucore")]
  extern "C" {
      pub fn u_strFromUTF8WithSub(dest: *mut UChar, destCapacity: i32, pDestLength: *mut i32, src: *c_char, subChar: UChar32, pNumSubstitutions: *i32, pErrorCode: *mut UErrorCode) -> *UChar;
  }
}

#[test]
fn test(){

    let s = "foobar";
    unsafe {
      let cap = s.char_len() * 2;
      let mut buf: ~[UChar] = std::vec::from_elem(cap, 0u8);// std::vec::with_capacity(cap);
      let pDestLength: *mut i32 = &mut 0;
      let pNumSubstitutions: *mut i32 = &mut 0;
      let pErrorCode: *mut UErrorCode = &mut ZERO_ERROR;

      let mut r = std::ptr::null();
      println!("{:?}", cap);
      
      s.with_c_str(|c_str| {
        r = icucore::u_strFromUTF8WithSub(buf.as_mut_ptr(), buf.capacity() as i32, pDestLength, c_str, 2i32, &SENTINEL, pErrorCode);   
        println!("{} {} {:?}", *pDestLength, *pNumSubstitutions, *pErrorCode);
      });

      println!("{:?}", buf);
    }

}

// u_strToUpper(UChar *dest, int32_t destCapacity, const UChar *src, int32_t srcLength, const char *locale, UErrorCode *pErrorCode)
// U_CAPI int32_t U_EXPORT2
// u_strToUpper(UChar *dest, int32_t destCapacity,
//              const UChar *src, int32_t srcLength,
//              const char *locale,
//              UErrorCode *pErrorCode) {
//     UCaseMap csm=UCASEMAP_INITIALIZER;
//     setTempCaseMap(&csm, locale);
//     return ustrcase_map(
//         &csm,
//         dest, destCapacity,
//         src, srcLength,
//         ustrcase_internalToUpper, pErrorCode);
// }
