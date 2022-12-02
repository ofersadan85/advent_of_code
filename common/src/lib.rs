use std::fmt::Debug;

pub mod algorithms;
pub mod file;
pub mod math;
pub mod v2;

/// Quick shortcut to "pretty-print"
pub fn pprint<T: Debug>(item: &T) {
    println!("{:#?}", item);
}

/// Split on lines breaks and trim whitespace from lines
pub fn split_lines(s: &str) -> Vec<String> {
    s.trim().split('\n').map(|x| x.trim().to_string()).collect()
}
