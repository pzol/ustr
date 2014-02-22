#[crate_id = "ustr#0.1"];
#[crate_type = "lib"];
#[license = "MIT"];
#[feature(globs)];
#[allow(dead_code)];

extern crate extra;
pub use ustring::*;
pub use regex::*;
mod ffi;
mod ustring;
mod regex;
