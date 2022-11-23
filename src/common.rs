// use itertools::Itertools;
// use std::collections::{HashMap, HashSet};

pub fn split_lines(s: &str) -> Vec<String> {
    s.trim().split('\n').map(|x| x.trim().to_string()).collect()
}

pub fn get_data(path: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(split_lines(&std::fs::read_to_string(path)?))
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return v;
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
