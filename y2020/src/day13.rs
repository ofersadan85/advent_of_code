#[derive(Debug, Default)]
struct NextBus {
    id: usize,
    depart_time: usize,
}

fn first_bus_value(input: &str) -> usize {
    let mut lines = input.lines();
    let min_time: usize = lines.next().unwrap().parse().unwrap();
    let first_bus = lines
        .next()
        .unwrap_or_default()
        .split(',')
        .flat_map(str::parse)
        .map(|n| NextBus {
            id: n,
            depart_time: ((min_time / n) + 1) * n,
        })
        .min_by_key(|bus| bus.depart_time)
        .unwrap_or_default();
    first_bus.id * (first_bus.depart_time - min_time)
}

#[advent_of_code_macros::aoc_tests]
mod tests {
    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn example_1() {
        assert_eq!(first_bus_value(EXAMPLE), 295);
    }

    #[test]
    fn part_1() {
        assert_eq!(first_bus_value(&read_input()), 259);
    }
}
