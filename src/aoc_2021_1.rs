pub fn count_increments(data: Vec<i32>) -> usize {
    (1..data.len())
        .filter(|i| data[*i] - data[i - 1] > 0)
        .count()
}

pub fn count_increments_windows(data: Vec<i32>) -> usize {
    let windows: Vec<i32> = data.windows(3).map(|slice| slice.iter().sum()).collect();
    (1..windows.len())
        .filter(|i| windows[*i] - windows[i - 1] > 0)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_1_1() {
        let example = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263";
        let example = example
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap_or_default())
            .collect();
        let result = super::count_increments(example);
        assert_eq!(result, 7);
    }
}
