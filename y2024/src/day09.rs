use std::cell::Cell;

#[derive(Debug, Clone)]
enum Blocks {
    File {
        length: usize,
        index: usize,
        used: Cell<usize>,
    },
    Empty(usize),
}

fn parse(input: &str) -> Vec<Blocks> {
    input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let digit = usize::try_from(c.to_digit(10)?).ok()?;
            if i % 2 == 0 {
                Some(Blocks::File {
                    length: digit,
                    index: i / 2,
                    used: Cell::new(0),
                })
            } else {
                Some(Blocks::Empty(digit))
            }
        })
        .collect()
}

fn checksum(blocks: &mut [Blocks]) -> usize {
    let mut iter = blocks.iter_mut();
    let mut block_index = 0;
    let mut sum: usize = 0;
    let mut unused_from_back: Option<&mut Blocks> = None;
    loop {
        let front_file = iter.next();
        match front_file {
            None => {
                if let Some(Blocks::File {
                    length,
                    index,
                    used,
                }) = unused_from_back
                {
                    let unused_length = *length - used.get();
                    sum += (0..unused_length)
                        .map(|i| (i + block_index) * *index)
                        .sum::<usize>();
                    used.replace(*length);
                }
                break;
            }
            Some(Blocks::File {
                length,
                index,
                used,
            }) => {
                sum += (0..*length)
                    .map(|i| (i + block_index) * *index)
                    .sum::<usize>();
                block_index += *length;
                used.replace(*length);
            }
            Some(Blocks::Empty(length)) => {
                let mut empty_length = *length;
                while empty_length > 0 {
                    let back_file = unused_from_back.take().or_else(|| iter.next_back());
                    match back_file {
                        None => break,
                        Some(Blocks::Empty(_)) => continue,
                        Some(Blocks::File {
                            length,
                            index,
                            used,
                        }) => {
                            let previous_used = used.get();
                            let unused_length = *length - previous_used;
                            let fill_length = empty_length.min(unused_length);
                            sum += (0..fill_length)
                                .map(|i| (i + block_index) * *index)
                                .sum::<usize>();
                            block_index += fill_length;
                            used.replace(previous_used + fill_length);
                            empty_length -= fill_length;
                            if unused_length > fill_length {
                                unused_from_back = back_file;
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}

fn defrag(blocks: &mut Vec<Blocks>) -> usize {
    let mut sum = 0;
    let blocks_clone = blocks.clone();
    let mut blocks_iter = blocks.iter().enumerate();
    let mut block_index = 0;
    loop {
        let front_file = blocks_iter.next();
        match front_file {
            None => break,
            Some((index_used, Blocks::File { length, index, .. })) if matches!(blocks_clone[index_used], Blocks::File { ref used, .. } if used.get() == 0) =>
            {
                print!(
                    "{}",
                    (0..*length).map(|_| index.to_string()).collect::<String>()
                );
                sum += (0..*length)
                    .map(|i| (i + block_index) * *index)
                    .sum::<usize>();
                block_index += *length;
            }
            Some((_, Blocks::File { length, .. })) => {
                print!("{}", (0..*length).map(|_| 'X').collect::<String>());
                block_index += length;
            }
            Some((_, Blocks::Empty(length))) => {
                let mut empty_length = *length;
                while empty_length > 0 {
                    let fill_file: Option<&Blocks> = blocks_clone.iter().rev().find(|b| {
                        if let Blocks::File { length, used, .. } = b {
                            *length <= empty_length && used.get() == 0
                        } else {
                            false
                        }
                    });
                    if let Some(Blocks::File {
                        length,
                        index,
                        used,
                    }) = fill_file
                    {
                        print!(
                            "{}",
                            (0..*length).map(|_| index.to_string()).collect::<String>()
                        );
                        sum += (0..*length)
                            .map(|i| (i + block_index) * index)
                            .sum::<usize>();
                        block_index += *length;
                        used.replace(*length);
                        // println!("Filled {empty_length} with {length}");
                        empty_length -= length;
                    } else {
                        print!("{}", (0..empty_length).map(|_| '.').collect::<String>());
                        block_index += empty_length;
                        empty_length = 0;
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_checksum() {
        let mut example = parse(EXAMPLE);
        assert_eq!(checksum(&mut example), 1928);
    }

    #[test]
    fn part_1() {
        let mut input = parse(&read_to_string("../inputs/2024/day09.txt").unwrap());
        assert_eq!(checksum(&mut input), 6259790630969);
    }

    #[test]
    fn test_defrag() {
        let mut example = parse(EXAMPLE);
        assert_eq!(defrag(&mut example), 2858);
    }

    #[test]
    #[ignore = "Wrong answer (too high)"]
    fn part_2() {
        let mut input = parse(&read_to_string("../inputs/2024/day09.txt").unwrap());
        assert_ne!(defrag(&mut input), 10662597557716);
    }
}
