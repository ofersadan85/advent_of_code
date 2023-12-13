use std::fmt::Debug;

pub mod algorithms;
pub mod cards;
pub mod file;
pub mod math;
pub mod range;
pub mod v2;

mod trace;
pub use trace::*;

/// Quick shortcut to "pretty-print"
pub fn pprint<T: Debug>(item: &T) {
    println!("{item:#?}");
}
