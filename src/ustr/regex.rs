use super::{ UString, ToUString };
use super::ffi::*;

trait Regex {
  fn matches(&self, pattern: &Self) -> Result<bool, UErrorCode>;
  fn matches_str(&self, pattern: &str) -> Result<bool, UErrorCode>;
  fn find(&self, pattern: &Self) -> Result<Match, UErrorCode>;
}

#[allow(unused_variable)]
impl<'a> Regex for &'a str {
  fn matches(&self, pattern: & &str) -> Result<bool, UErrorCode> {
    self.to_u().matches(&pattern.to_u())
  }

  fn matches_str(&self, pattern: &str) -> Result<bool, UErrorCode> {
    unimplemented!();
  }

  fn find(&self, pattern: & &str) -> Result<Match, UErrorCode> {
    unimplemented!();
  }
}

impl Regex for UString {
  fn matches(&self, pattern: &UString) -> Result<bool, UErrorCode> {
    let mut error_code = ZERO_ERROR;
    let m = match Match::new(pattern.to_owned()) {
      Err(e)  => return Err(e),
      Ok(m)       => m
    };

    regex_setText(m.regex, self.as_ptr(), self.len() as i32, &mut error_code);

    if success(error_code) {
      Ok(regex_matches(m.regex, 0, &mut error_code))
    } else {
      Err(error_code)
    } 
  }

  fn matches_str(&self, pattern: &str) -> Result<bool, UErrorCode> {
    self.matches(&pattern.to_u())
  }

  fn find(&self, pattern: &UString) -> Result<Match, UErrorCode> {
    let mut error_code = ZERO_ERROR;
    let m = match Match::new(pattern.to_owned()) {
      Err(e)  => return Err(e),
      Ok(m)       => m
    };

    regex_setText(m.regex, self.as_ptr(), self.len() as i32, &mut error_code);

    if success(error_code) {
      let _found = regex_find(m.regex, 0, &mut error_code);
      Ok(m)
    } else {
      Err(error_code)
    } 
  }
}

struct Match {
  regex: *mut URegularExpression
}

impl Match {
  fn new(pattern: UString) -> Result<Match, UErrorCode> {
    let mut parse_error = UParseError::empty();
    let mut error_code = ZERO_ERROR;
    let regex = regex_open(pattern.as_ptr(), pattern.len() as i32, UREGEX_NONE, &mut parse_error, &mut error_code);
    
    if success(error_code) {
      Ok(Match { regex: regex })
    } else {
      Err(error_code)
    }
  }

  fn text(&self) -> UString {
    let mut error_code = ZERO_ERROR;
    let mut text_length = 0;
    let u = regex_getText(self.regex, &mut text_length, &mut error_code);

    UString::from_bytes(u)
  }
}
