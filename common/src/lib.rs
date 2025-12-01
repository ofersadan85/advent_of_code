pub mod algorithms;
pub mod cards;
pub mod coords;
pub mod file;
pub mod grid;
pub mod iterators;
pub mod math;
pub mod range;
pub mod v2;

pub fn trim_lines(input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
}
