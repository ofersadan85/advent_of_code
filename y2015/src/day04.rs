pub fn normal(input: &str, expect: &str) -> usize {
    (0usize..usize::MAX)
        .find(|i| {
            let test_str = format!("{input}{i}");
            let hash = md5::compute(test_str);
            format!("{hash:x}").starts_with(expect)
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod normal_tests {
    use super::*;

    #[test]
    fn example_1() {
        let start = std::time::Instant::now();
        assert_eq!(normal("abcdef", "00000"), 609043);
        println!("Normal example 1 took {:?}", start.elapsed());
    }

    #[test]
    fn example_2() {
        let start = std::time::Instant::now();
        assert_eq!(normal("pqrstuv", "00000"), 1048970); // cspell: disable-line
        println!("Normal example 2 took {:?}", start.elapsed());
    }

    #[test]
    fn part_1() {
        let start = std::time::Instant::now();
        assert_eq!(normal("yzbqklnj", "00000"), 282749); // cspell: disable-line
        println!("Normal part 1 took {:?}", start.elapsed());
    }

    #[test]
    fn part_2() {
        let start = std::time::Instant::now();
        assert_eq!(normal("yzbqklnj", "000000"), 9962624); // cspell: disable-line
        println!("Normal part 2 took {:?}", start.elapsed());
    }
}

#[cfg(test)]
mod parallel_tests {
    //! This is a parallel version of the normal solution.
    //! It is not faster than the normal solution, because the overhead of
    //! parallelization is too high for this problem.
    //! It is only here to show how to use rayon.
    use rayon::prelude::*;

    pub fn parallel(input: &str, expect: &str) -> usize {
        (0usize..usize::MAX)
            .into_par_iter()
            .find_first(|i| {
                let test_str = format!("{input}{i}");
                let hash = md5::compute(test_str);
                format!("{hash:x}").starts_with(expect)
            })
            .unwrap_or(0)
    }
    #[test]
    fn example_1() {
        let start = std::time::Instant::now();
        assert_eq!(parallel("abcdef", "00000"), 609043);
        println!("Parallel example 1 took {:?}", start.elapsed());
    }

    #[test]
    fn example_2() {
        let start = std::time::Instant::now();
        assert_eq!(parallel("pqrstuv", "00000"), 1048970); // cspell: disable-line
        println!("Parallel example 2 took {:?}", start.elapsed());
    }

    #[test]
    fn part_1() {
        let start = std::time::Instant::now();
        assert_eq!(parallel("yzbqklnj", "00000"), 282749); // cspell: disable-line
        println!("Parallel part 1 took {:?}", start.elapsed());
    }

    #[test]
    fn part_2() {
        let start = std::time::Instant::now();
        assert_eq!(parallel("yzbqklnj", "000000"), 9962624); // cspell: disable-line
        println!("Parallel part 2 took {:?}", start.elapsed());
    }
}
