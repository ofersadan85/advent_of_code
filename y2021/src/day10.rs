const PATH: &str = "../inputs/2021/day10.txt";
const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

fn calc_score(s: &str) -> usize {
    let s = s.replace(['(', '[', '{', '<'], "");
    match s.chars().next().unwrap_or('X') {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn clean_line(s: &str) -> String {
    let mut last = s.len() + 1;
    let mut result = s.to_string();
    while result.len() < last {
        last = result.len();
        result = result
            .replace("<>", "")
            .replace("[]", "")
            .replace("()", "")
            .replace("{}", "");
    }
    result
}

fn clean_corrupted(data: &[String]) -> usize {
    data.iter().map(|row| calc_score(&clean_line(row))).sum()
}

fn fix_incomplete(data: &[String]) -> usize {
    let mut scores: Vec<usize> = data
        .iter()
        .filter(|&row| calc_score(&clean_line(row)) == 0)
        .map(|row| {
            let row = clean_line(row);
            let mut result = 0;
            for c in row.chars().rev() {
                let value = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
                result = result * 5 + value;
            }
            result
        })
        .collect();
    scores.sort_unstable();
    scores.get(scores.len() / 2).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::file::split_lines;

    #[test]
    fn example_1() {
        let data = split_lines(EXAMPLE);
        assert_eq!(clean_corrupted(&data), 26397);
    }

    #[test]
    fn example_2() {
        let data = split_lines(EXAMPLE);
        assert_eq!(fix_incomplete(&data), 288_957);
    }

    #[test]
    fn task_1() {
        let data = split_lines(&std::fs::read_to_string(PATH).unwrap());
        assert_eq!(clean_corrupted(&data), 290_691);
    }

    #[test]
    fn task_2() {
        let data = split_lines(&std::fs::read_to_string(PATH).unwrap());
        assert_eq!(fix_incomplete(&data), 2_768_166_558);
    }
}
