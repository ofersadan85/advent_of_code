mod common;
use crate::common::get_data;
mod aoc_2021_1;
use aoc_2021_1::*;

fn main() -> Result<(), std::io::Error>{
    let path = "inputs/aoc_2021_1.txt";
    let result = count_increments(get_data(path)?);
    println!("{result:?}");
    let result = count_increments_windows(get_data(path)?);
    println!("{result:?}");
    Ok(())
}
