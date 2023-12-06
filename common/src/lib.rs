use std::fmt::Debug;

pub mod algorithms;
pub mod file;
pub mod math;
pub mod range;
pub mod v2;

/// Quick shortcut to "pretty-print"
pub fn pprint<T: Debug>(item: &T) {
    println!("{item:#?}");
}
