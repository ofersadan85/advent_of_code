use advent_of_code_macros::aoc_solver;

fn fft_step(input: &[i32], output: &mut [i32]) {
    let n = input.len();

    // prefix[i] = sum(input[..i])
    let mut prefix = vec![0i32; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + input[i];
    }

    for (row, item) in output.iter_mut().enumerate().take(n) {
        let block = row + 1;
        let mut sum = 0;

        // first +1 block starts at `row`
        let mut pos = row;

        while pos < n {
            // +1 block
            let end = (pos + block).min(n);
            sum += prefix[end] - prefix[pos];

            // skip zero block
            pos += block * 2;
            if pos >= n {
                break;
            }

            // -1 block
            let end = (pos + block).min(n);
            sum -= prefix[end] - prefix[pos];

            // skip zero block
            pos += block * 2;
        }

        *item = sum.abs() % 10;
    }
}

#[aoc_solver(
    suffix = "example_1",
    input = "80871224585914546619083218645595",
    expected = 24176176
)]
#[aoc_solver(
    suffix = "example_2",
    input = "19617804207202209144916044189917",
    expected = 73745418
)]
#[aoc_solver(
    suffix = "example_3",
    input = "69317163492948606335995924319873",
    expected = 52432133
)]
#[aoc_solver(file = "inputs/2019/day16.txt", expected = 63794407)]
fn part_1(input: &str) -> usize {
    let mut signal: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| {
            let d = c.to_digit(10).expect("digits");
            i32::try_from(d).expect("i32 digit")
        })
        .collect();
    let mut buffer = vec![0; signal.len()];
    for _ in 0..100 {
        fft_step(&signal, &mut buffer);
        std::mem::swap(&mut signal, &mut buffer);
    }
    let result = signal.iter().take(8).fold(0, |acc, &x| acc * 10 + x);
    usize::try_from(result).expect("usize value")
}

#[aoc_solver(
    suffix = "example_1",
    input = "03036732577212944063491565474664",
    expected = 84462026
)]
#[aoc_solver(
    suffix = "example_2",
    input = "02935109699940807407585447034323",
    expected = 78725270
)]
#[aoc_solver(
    suffix = "example_3",
    input = "03081770884921959731165446850517",
    expected = 53553731
)]
#[aoc_solver(file = "inputs/2019/day16.txt", expected = 77247538)]
fn part_2(input: &str) -> usize {
    let signal: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| {
            let d = c.to_digit(10).expect("digits");
            i32::try_from(d).expect("i32 digit")
        })
        .collect();
    let offset = input
        .chars()
        .take(7)
        .fold(0, |n, c| n * 10 + c.to_digit(10).expect("digits"));
    let offset = usize::try_from(offset).expect("usize offset");
    let total = signal.len() * 10_000;

    assert!(
        offset >= total / 2,
        "suffix trick only works when offset is in second half"
    );

    // Only build the part we actually need.
    let mut signal: Vec<i32> = (offset..total).map(|i| signal[i % signal.len()]).collect();

    for _ in 0..100 {
        let mut suffix = 0;
        for x in signal.iter_mut().rev() {
            suffix = (suffix + *x) % 10;
            *x = suffix;
        }
    }
    let result = signal.iter().take(8).fold(0, |acc, &x| acc * 10 + x);
    usize::try_from(result).expect("usize value")
}
