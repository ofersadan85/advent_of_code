// use itertools::Itertools;
// use std::collections::{HashMap, HashSet};

pub fn split_lines(s: &str) -> Vec<String> {
    s.trim().split('\n').map(|x| x.trim().to_string()).collect()
}

pub fn get_data(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(split_lines(&std::fs::read_to_string(path)?))
}
