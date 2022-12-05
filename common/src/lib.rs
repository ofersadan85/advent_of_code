use std::fmt::Debug;

pub mod algorithms;
pub mod file;
pub mod math;
pub mod v2;

/// Quick shortcut to "pretty-print"
pub fn pprint<T: Debug>(item: &T) {
    println!("{:#?}", item);
}

/// General building block for advent of code
pub struct AdventOfCode {
    pub year: u16,
    pub day: u8,
    pub example: String,
}
