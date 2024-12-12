use tracing::instrument;

#[derive(Debug, Clone, Copy)]
enum Blocks {
    File { length: usize, index: usize },
    Empty(usize),
}

impl Blocks {
    const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    const fn length(&self) -> usize {
        match self {
            Self::File { length, .. } | Self::Empty(length) => *length,
        }
    }

    const fn index(&self) -> usize {
        match self {
            Self::File { index, .. } => *index,
            Self::Empty(_) => usize::MAX,
        }
    }
}

fn parse(input: &str) -> Vec<Blocks> {
    input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let digit = usize::try_from(c.to_digit(10)?).ok()?;
            if i % 2 == 0 {
                Some(vec![
                    Blocks::File {
                        length: digit,
                        index: i / 2,
                    };
                    digit
                ])
            } else {
                Some(vec![Blocks::Empty(digit); digit])
            }
        })
        .flatten()
        .collect()
}

fn checksum(blocks: &[Blocks]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| !block.is_empty())
        .map(|(i, block)| i * block.index())
        .sum()
}

#[instrument(skip_all, level = "info")]
fn checksum_fragmented(blocks: &mut [Blocks]) -> usize {
    let mut cursor_front = 0;
    let mut cursor_back = blocks.len() - 1;
    while cursor_front < cursor_back {
        let mut block_front = blocks.get(cursor_front);
        while let Some(Blocks::File { .. }) = block_front {
            if cursor_back <= cursor_front {
                break;
            }
            cursor_front += 1;
            block_front = blocks.get(cursor_front);
        }
        let mut block_back = blocks.get(cursor_back);
        while let Some(Blocks::Empty { .. }) = block_back {
            if cursor_back <= cursor_front {
                break;
            }
            cursor_back -= 1;
            block_back = blocks.get(cursor_back);
        }
        blocks.swap(cursor_front, cursor_back);
        cursor_back -= 1;
        cursor_front += 1;
    }
    checksum(blocks)
}

#[instrument(skip_all, level = "info")]
fn checksum_defragmented(blocks: &mut [Blocks]) -> usize {
    let mut cursor_back = blocks.len() - 1;
    while cursor_back > 0 {
        let mut block_back = blocks.get_mut(cursor_back);
        while let Some(Blocks::Empty(length)) = block_back {
            *length = 0;
            cursor_back -= 1;
            block_back = blocks.get_mut(cursor_back);
        }
        let (file_length, file_index) = block_back
            .map(|block| (block.length(), block.index()))
            .unwrap_or_default();
        let (empty_start, empty_length) = blocks
            .iter()
            .enumerate()
            .find(|(_, b)| b.is_empty() && b.length() >= file_length)
            .map(|(i, b)| (i, b.length()))
            .unwrap_or_default();
        if empty_length >= file_length && empty_start + file_length < cursor_back {
            for i in 0..file_length {
                blocks[i + empty_start] = Blocks::File {
                    length: file_length,
                    index: file_index,
                };
                blocks[cursor_back - i] = Blocks::Empty(0);
            }
            let left_over = empty_length - file_length;
            for i in 0..left_over {
                let index = empty_start + file_length + i;
                blocks[index] = Blocks::Empty(left_over);
            }
        }
        cursor_back = cursor_back.saturating_sub(file_length);
    }
    checksum(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use test_log::test;
    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_fragmented() {
        let mut example = parse(EXAMPLE);
        assert_eq!(checksum_fragmented(&mut example), 1928);
    }

    #[test]
    fn part_1() {
        let mut input = parse(&read_to_string("../inputs/2024/day09.txt").unwrap());
        assert_eq!(checksum_fragmented(&mut input), 6259790630969);
    }

    #[test]
    fn test_defragmented() {
        let mut example = parse(EXAMPLE);
        assert_eq!(checksum_defragmented(&mut example), 2858);
    }

    #[test]
    fn part_2() {
        let mut input = parse(&read_to_string("../inputs/2024/day09.txt").unwrap());
        assert_eq!(checksum_defragmented(&mut input), 6289564433984);
    }
}
