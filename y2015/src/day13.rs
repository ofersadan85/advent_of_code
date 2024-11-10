use std::collections::HashMap;
use itertools::Itertools;

fn parse_line(line: &str) -> (&str, &str, isize) {
    let (source, rest) = line.split_once(" would ").expect("first split");
    let (measure, target) = rest
        .trim_matches('.')
        .split_once(" happiness units by sitting next to ")
        .expect("second split");
    let scale = if measure.contains("lose") { -1 } else { 1 };
    let (_, measure) = measure.split_once(' ').expect("third split");
    let measure = measure.parse::<isize>().expect("numbers") * scale;
    (source, target, measure)
}

type HappinessMeasure<'a> = HashMap<&'a str, isize>;
type HappinessMap<'a> = HashMap<&'a str, HappinessMeasure<'a>>;

fn parse_input(input: &str) -> HappinessMap<'_> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (source, target, measure) = parse_line(line);
        map.entry(source)
            .and_modify(|values: &mut HappinessMeasure| {
                values.insert(target, measure);
            })
            .or_insert_with(|| {
                let mut inner = HashMap::new();
                inner.insert(target, measure);
                inner
            });
    }
    map
}

fn part_1(map: &HappinessMap) -> Option<isize> {
    map.keys().permutations(map.len()).map(|table| {
        let mut happiness = 0;
        for i in 0..table.len() {
            let i_left = if i == 0 {table.len() - 1} else {i - 1};
            let i_right = if i == table.len() - 1 {0} else {i + 1};
            let p = table[i];
            let p_right = table[i_right];
            let p_left = table[i_left];
            happiness += map[p][p_left] + map[p][p_right];
        }
        happiness
    }).max()
}

fn part_2(map: &HappinessMap) -> Option<isize> {
    let me: HappinessMeasure = map.keys().map(|&key| (key, 0)).collect();
    let mut new_map = map.clone();
    for key in map.keys() {
        new_map.get_mut(key).expect("known key").insert("me", 0);
    }
    new_map.insert("me", me);
    part_1(&new_map)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_parse() {
        let input = read_to_string("../inputs/2015/day13_example.txt").unwrap();
        let parsed = parse_input(&input);
        let expected = serde_json::from_str(r#"{"Carol": {"Bob": 60, "David": 55, "Alice": -62}, "Alice": {"Bob": 54, "Carol": -79, "David": -2}, "David": {"Carol": 41, "Alice": 46, "Bob": -7}, "Bob": {"Carol": -7, "Alice": 83, "David": -63}}"#).unwrap();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_example_1() {
        let input = read_to_string("../inputs/2015/day13_example.txt").unwrap();
        let map = parse_input(&input);
        assert_eq!(part_1(&map).unwrap(), 330);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day13.txt").unwrap();
        let map = parse_input(&input);
        assert_eq!(part_1(&map).unwrap(), 618);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day13.txt").unwrap();
        let map = parse_input(&input);
        assert_eq!(part_2(&map).unwrap(), 601);
    }
}
