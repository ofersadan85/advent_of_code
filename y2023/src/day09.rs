use tracing::debug;

fn parse_input(s: &str) -> Vec<Vec<isize>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(str::parse::<isize>)
                .collect()
        })
        .collect()
}

fn polynomial_degree(data: &[isize], level: u8) -> u8 {
    let mut final_line = true;
    debug!("{data:?}");
    let mut next_line = vec![];
    for w in data.windows(2) {
        let diff = w[1] - w[0];
        if diff != next_line.first().copied().unwrap_or(diff) {
            final_line = false;
        }
        next_line.push(diff);
    }
    if final_line {
        debug!("{next_line:?}");
        level + 1
    } else {
        polynomial_degree(&next_line, level + 1)
    }
}

#[advent_of_code_macros::aoc_tests]
mod tests {
    pub const EXAMPLE: &str = "0 3 6 9 12 15
                            1 3 6 10 15 21
                            10 13 16 21 30 45";

    #[test]
    fn degrees() {
        // let input = read_input();
        let data = parse_input(EXAMPLE);
        for (row, i) in data.iter().zip([1, 2, 3]) {
            let d = polynomial_degree(row, 0);
            assert_eq!(d, i, "Row {row:?}");
        }
    }
}
