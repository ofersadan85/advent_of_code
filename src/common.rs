// use itertools::Itertools;
// use std::collections::{HashMap, HashSet};
use std::{str::FromStr};

pub fn get_data<T: FromStr + Default>(path: &str) -> Result<Vec<T>, std::io::Error> {
    let data: Vec<String> = std::fs::read_to_string(path)?
        .split_ascii_whitespace()
        .map(|word| word.to_string())
        .collect();
    let new_data = data.iter().map(|x| x.parse().unwrap_or_default()).collect();
    Ok(new_data)
}
