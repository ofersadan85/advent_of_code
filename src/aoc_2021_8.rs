use itertools::Itertools;

fn is_match(bigger: &str, smaller: &str) -> bool {
    smaller.chars().all(|c| bigger.contains(c))
}

fn remove_value(v: Vec<String>, value: &str) -> Vec<String> {
    let index = v.iter().find_position(|s| *s == value).unwrap().0;
    let mut result = v;
    result.remove(index);
    result
}

fn row_map_digits(words: &[String], output: &[String]) -> usize {
    // let mut word_map: HashMap<u8, String> = HashMap::new();
    let mut words = words.to_owned();
    words.sort_unstable_by_key(|x| x.len());
    let mut word_map: [String; 10] = [
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];
    word_map[8] = words.pop().unwrap();
    word_map[1] = words.remove(0);
    word_map[7] = words.remove(0);
    word_map[4] = words.remove(0);

    word_map[3] = words
        .iter()
        .find(|s| s.len() == 5 && is_match(s, &word_map[1]))
        .unwrap()
        .to_string();
    words = remove_value(words, &word_map[3]);

    word_map[9] = words
        .iter()
        .find(|s| is_match(s, &word_map[4]))
        .unwrap()
        .to_string();
    words = remove_value(words, &word_map[9]);

    word_map[5] = words
        .iter()
        .find(|s| s.len() == 5 && is_match(&word_map[9], s))
        .unwrap()
        .to_string();
    words = remove_value(words, &word_map[5]);

    word_map[0] = words
        .iter()
        .find(|s| s.len() == 6 && is_match(s, &word_map[1]))
        .unwrap()
        .to_string();
    words = remove_value(words, &word_map[0]);

    word_map[6] = words
        .iter()
        .find(|s| s.len() == 6 && !word_map.contains(s))
        .unwrap()
        .to_string();
    words = remove_value(words, &word_map[6]);

    word_map[2] = words
        .iter()
        .find(|s| s.len() == 5 && !word_map.contains(s))
        .unwrap()
        .to_string();

    output
        .iter()
        .map(|w| {
            word_map
                .iter()
                .find_position(|s| s == &&w.chars().sorted().collect::<String>())
                .unwrap()
                .0
        })
        .join("")
        .parse()
        .unwrap()
}

fn row_count_unique(row: Vec<String>) -> usize {
    let unique_lengths: [usize; 4] = [2, 3, 4, 7];
    row.iter()
        .filter(|s| unique_lengths.contains(&s.len()))
        .count()
}

fn count_unique(data: Vec<String>) -> usize {
    data.iter()
        .map(|s| {
            row_count_unique(
                s.split('|')
                    .last()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|x| x.to_string())
                    .collect(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;
    const PATH: &str = "inputs/aoc_2021_8.txt";
    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    fn setup_data(data: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
        let mut result = vec![];
        for row in data {
            let mut split = row.split('|');
            let words: Vec<String> = split
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.chars().sorted().collect())
                .collect();
            let output: Vec<String> = split
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.chars().sorted().collect())
                .collect();
            result.push((words, output));
        }
        result
    }

    #[test]
    fn example_1() {
        let data = split_lines(EXAMPLE);
        let result: usize = count_unique(data);
        assert_eq!(result, 26);
    }

    #[test]
    fn example_2() {
        let data = setup_data(split_lines(EXAMPLE));
        let result: usize = data
            .iter()
            .map(|(words, output)| row_map_digits(words, output))
            .sum();
        assert_eq!(result, 61229);
    }

    #[test]
    fn task_1() {
        let data = get_data(PATH).unwrap();
        let result: usize = count_unique(data);
        assert_eq!(result, 554);
    }

    #[test]
    fn task_2() {
        let data = setup_data(get_data(PATH).unwrap());
        let result: usize = data
            .iter()
            .map(|(words, output)| row_map_digits(words, output))
            .sum();
        assert_eq!(result, 990964);
    }
}
