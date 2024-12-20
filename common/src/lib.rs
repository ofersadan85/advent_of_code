use std::fmt::Debug;

pub mod algorithms;
pub mod cards;
pub mod file;
pub mod grid;
pub mod math;
pub mod range;
pub mod v2;
pub mod iterators;

/// Quick shortcut to "pretty-print"
pub fn pprint<T: Debug>(item: &T) {
    println!("{item:#?}");
}
