use advent_of_code_common::Solver;

fn max_volt(input: &str, max_len: usize) -> Result<usize, ()> {
    if max_len == 1 {
        let c = input.as_bytes().iter().max().ok_or(())?;
        let c = c - b'0'; // Convert ASCII byte to digit
        return Ok(c.into());
    }
    let searchable_len = input.len().saturating_sub(max_len - 1);
    let searchable_input = &input.as_bytes()[..searchable_len];
    let (first_max_index, first_max) = searchable_input
        .iter()
        .enumerate()
        .max_by(|(ia, ca), (ib, cb)| ca.cmp(cb).then(ia.cmp(ib).reverse()))
        .ok_or(())?;
    let first_max = first_max - b'0'; // Convert ASCII byte to digit
    let next_input = &input[first_max_index + 1..];
    let exp = u32::try_from(max_len - 1).expect("exponent fits in u32");
    Ok(usize::from(first_max) * 10_usize.pow(exp) + max_volt(next_input, max_len - 1)?)
}

struct Parts1And2(usize);
impl Solver<'_> for Parts1And2 {
    type Output = usize;

    fn solve(&self, input: &str) -> Self::Output {
        assert!(input.is_ascii());
        input.lines().filter_map(|s| max_volt(s, self.0).ok()).sum()
    }

    fn file_path(&self) -> std::path::PathBuf {
        crate::default_input_path!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::expect_solution;

    #[test]
    fn part_1() {
        expect_solution!(Parts1And2(2), 0, 357);
        expect_solution!(Parts1And2(2), 1, 17613);
    }

    #[test]
    fn part_2() {
        expect_solution!(Parts1And2(12), 0, 3121910778619);
        expect_solution!(Parts1And2(12), 1, 175304218462560);
    }
}
