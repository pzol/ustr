#[feature(globs)];
#[allow(ctypes, non_camel_case_types)];

use std::str;
use std::libc::{ c_char };


pub type UChar     = u16;
pub type UChar32   = i32;

#[deriving(Eq)]
pub enum UErrorCode {
  ZERO_ERROR = 0,
  BUFFER_OVERFLOW_ERROR = 15,

  U_REGEX_INTERNAL_ERROR = 0x10300, U_REGEX_RULE_SYNTAX, 
  U_REGEX_INVALID_STATE, U_REGEX_BAD_ESCAPE_SEQUENCE, U_REGEX_PROPERTY_SYNTAX, U_REGEX_UNIMPLEMENTED, 
  U_REGEX_MISMATCHED_PAREN, U_REGEX_NUMBER_TOO_BIG, U_REGEX_BAD_INTERVAL, U_REGEX_MAX_LT_MIN, 
  U_REGEX_INVALID_BACK_REF, U_REGEX_INVALID_FLAG, U_REGEX_LOOK_BEHIND_LIMIT, U_REGEX_SET_CONTAINS_STRING, 
  U_REGEX_OCTAL_TOO_BIG, U_REGEX_MISSING_CLOSE_BRACKET, U_REGEX_INVALID_RANGE, U_REGEX_STACK_OVERFLOW, 
  U_REGEX_TIME_OUT, U_REGEX_STOPPED_BY_CALLER, U_REGEX_ERROR_LIMIT
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

pub enum URegularExpression {}

mod icuio {
  use super::*;

  #[link(name = "icuio")]
  extern "C" {
    pub fn u_printf_u_52(patternSpecification: *UChar, ...) -> i32;
  }
}

static U_PARSE_CONTEXT_LEN: i32 = 16;

/// http://icu-project.org/apiref/icu4c/structUParseError.html
pub struct UParseError {
  line: i32,
  offset: i32,
  preContext:  [UChar, ..U_PARSE_CONTEXT_LEN],
  postContext: [UChar, ..U_PARSE_CONTEXT_LEN]
}

impl UParseError {
  pub fn empty() -> UParseError {
    UParseError { line: 0, offset: 0, 
                  preContext:  [0, ..U_PARSE_CONTEXT_LEN],
                  postContext: [0, ..U_PARSE_CONTEXT_LEN] 
                }
  }
}

pub enum URegexpFlag {
  UREGEX_NONE = 0,
  UREGEX_CANON_EQ = 128, UREGEX_CASE_INSENSITIVE = 2, UREGEX_COMMENTS = 4, UREGEX_DOTALL = 32, 
  UREGEX_LITERAL = 16, UREGEX_MULTILINE = 8, UREGEX_UNIX_LINES = 1, UREGEX_UWORD = 256, 
  UREGEX_ERROR_ON_UNKNOWN_ESCAPES = 512 
}

mod icui18n {
  use super::*;

  #[link(name = "icui18n")]
  extern "C" {
    /// Open (compile) an ICU regular expression.
    pub fn uregex_open_52(pattern: *UChar, patternLength: i32, flags: URegexpFlag, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    /// Attempts to match the input string against the pattern.
    pub fn uregex_matches_52(regexp: *mut URegularExpression, startIndex: i32, status: *mut UErrorCode) -> bool;
    /// Set the subject text string upon which the regular expression will look for matches. 
    pub fn uregex_setText_52(regexp: *mut URegularExpression, text: *UChar, textLength: i32, status: *mut UErrorCode);
    /// Get the subject text that is currently associated with this regular expression object. 
    pub fn uregex_getText_52(regexp: *mut URegularExpression, textLength: *mut i32, status: *mut UErrorCode) -> *UChar;
    /// Find the first matching substring of the input string that matches the pattern. 
    pub fn uregex_find_52(regexp: *mut URegularExpression, startIndex: i32, status: *mut UErrorCode) -> bool;
    /// Split a string into fields.
    pub fn uregex_split_52(regexp: *mut URegularExpression, destBuf: *mut UChar, destCapacity: i32, requiredCapacity: *i32, destFields: &[*UChar], destFieldsCapacity: i32, status: *mut UErrorCode) -> i32;
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
      // #[link_name="u_strlen_52"]
      pub fn u_strlen_52(s: *UChar) -> i32;
      pub fn u_errorName_52(code: i32) -> *char;
      pub fn u_strFromUTF8WithSub_52(dest: *mut UChar, destCapacity: i32, pDestLength: *mut i32, src: *c_char, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar;
      pub fn u_strToUTF8WithSub_52(dest: *mut c_char, destCapacity: i32, pDestLength: *mut i32, src: *UChar, srcLength: i32, subChar: UChar32, pNumSubstitutions: *mut i32, pErrorCode: *mut UErrorCode) -> *mut UChar;
      pub fn u_strToTitle_52(dest: *mut UChar, destCapacity: i32, src: *UChar, srcLength: i32, titleIter: *UBreakIterator, locale: *c_char, pErrorCode: *mut UErrorCode) -> i32;
      pub fn u_strcat_52(dest: *mut UChar, src: *UChar) -> *mut UChar;

      pub fn u_strtok_r_52(src: *mut UChar, delim: *UChar, saveState: *mut *mut UChar) -> *UChar;

      pub fn u_tolower_52(c: UChar32) -> UChar32;
      pub fn u_toupper_52(c: UChar32) -> UChar32;
  }
}

// pub fn uregex_split_52(regexp: *mut URegularExpression, destBuf: *mut UChar, destCapacity: i32, requiredCapacity: *i32, destFields: &[*UChar], destFieldsCapacity: i32, status: *mut UErrorCode) -> i32;
pub fn uregex_split(regexp: *mut URegularExpression, destBuf: *mut UChar, destCapacity: i32, requiredCapacity: *i32, destFields: &[*UChar], destFieldsCapacity: i32, status: *mut UErrorCode) -> i32 {
  unsafe { icui18n::uregex_split_52(regexp, destBuf, destCapacity, requiredCapacity, destFields, destFieldsCapacity, status) }
}

pub fn regex_setText(regexp: *mut URegularExpression, text: *UChar, textLength: i32, status: *mut UErrorCode) {
  unsafe { icui18n::uregex_setText_52(regexp, text, textLength, status) }
}

pub fn regex_getText(regexp: *mut URegularExpression, textLength: *mut i32, status: *mut UErrorCode) -> *UChar {
  unsafe { icui18n::uregex_getText_52(regexp, textLength, status) }
}

pub fn regex_open(pattern: *UChar, patternLength: i32, flags: URegexpFlag, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression {
  unsafe { icui18n::uregex_open_52(pattern, patternLength, flags, pe, status) }
}

pub fn regex_matches(regexp: *mut URegularExpression, startIndex: i32, status: *mut UErrorCode) -> bool {
  unsafe { icui18n::uregex_matches_52(regexp, startIndex, status) }
}

pub fn regex_find(regexp: *mut URegularExpression, startIndex: i32, status: *mut UErrorCode) -> bool {
  unsafe { icui18n::uregex_find_52(regexp, startIndex, status) }
}

#[inline]
pub fn to_upper(c: UChar32) -> UChar32 {
  unsafe { icuuc::u_toupper_52(c) }
}

#[inline]
pub fn to_lower(c: UChar32) -> UChar32 {
  unsafe { icuuc::u_tolower_52(c) }
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
