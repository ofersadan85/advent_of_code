use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Replacement<'a> {
    src: &'a str,
    dst: &'a str,
}

impl<'a> Replacement<'a> {
    fn reverse(&mut self) {
        let tmp = self.src;
        self.src = self.dst;
        self.dst = tmp;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Data<'a> {
    replacements: Vec<Replacement<'a>>,
    molecule: String,
}

impl<'a> Data<'a> {
    fn reverse_all(&mut self) {
        self.replacements.iter_mut().for_each(|r| r.reverse());
    }
}

fn parse_input<'a>(s: &'a str) -> Data<'a> {
    let replacements: Vec<_> = s
        .lines()
        .take_while(|s| !s.trim().is_empty())
        .map(|s| {
            let (src, dst) = s.split_once(" => ").unwrap();
            Replacement {
                src: src.trim(),
                dst: dst.trim(),
            }
        })
        .collect();
    let molecule = s.lines().last().unwrap().trim().to_string();
    Data {
        replacements,
        molecule,
    }
}

fn find_indexes(src: &str, dst: &str) -> Vec<usize> {
    let mut indexes = vec![];
    let mut rest = src;
    let mut first = true;
    while let Some(index) = rest.find(dst) {
        let added = if first { 0 } else { dst.len() };
        let (_, post) = rest.split_at(index + dst.len());
        rest = post;
        indexes.push(index + indexes.last().unwrap_or(&0) + added);
        first = false;
    }
    indexes
}

fn replace_at_index(s: &str, r: &Replacement, index: usize) -> String {
    let (pre, post) = s.split_at(index + r.src.len());
    let (pre, mid) = pre.split_at(index);
    debug_assert_eq!(
        mid, r.src,
        "Expected {mid} == {} ({s}, {r:?}, {index})",
        r.src
    );
    format!("{}{}{}", pre, r.dst, post)
}

fn possible_mutations(molecule: &str, replacements: &[Replacement]) -> HashSet<String> {
    let mut set = HashSet::new();
    for r in replacements {
        let indexes = find_indexes(molecule, r.src);
        for i in indexes {
            set.insert(replace_at_index(molecule, r, i));
        }
    }
    set
}

fn shortest_mutation(data: &Data) -> usize {
    let mut data = data.clone();
    data.reverse_all();
    let mut set = HashSet::new();
    set.insert(data.molecule.clone());
    let mut seen = HashSet::new();
    seen.insert(data.molecule.clone());
    let mut count = 0;
    while !set.contains("e") {
        let old_set: HashSet<_> = set.drain().collect();
        eprintln!("{old_set:#?}");
        for item in old_set {
            let mut new_mutations = possible_mutations(&item, &data.replacements);
            new_mutations.retain(|m| m.len() <= data.molecule.len());
            set.extend(new_mutations.clone());
        }
        count += 1;
        set.retain(|m| !seen.contains(m) && (m == "e" || !m.contains("e")));
        seen.extend(set.clone());
        // if set.len() > 0 {
        //     // I'll make the assumption that the general direction should be for shorter length molecules
        //     // And so remove those that are below average length, with more weight (2:1) for being shorter
        //     let shortest = set.iter().min_by_key(|m| m.len()).expect("empty set").len();
        //     let longest = set.iter().max_by_key(|m| m.len()).expect("empty set").len();
        //     set.retain(|m| m.len() <= (shortest * 99 + longest) / 100);
        // } else {
        //     break;
        // }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE1: &str = "H => HO
        H => OH
        O => HH

        HOH";

    const EXAMPLE2: &str = "e => H
        e => O
        H => HO
        H => OH
        O => HH
        
        HOHOHO";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE1);
        assert_eq!(
            data,
            Data {
                replacements: vec![
                    Replacement {
                        src: "H",
                        dst: "HO"
                    },
                    Replacement {
                        src: "H",
                        dst: "OH"
                    },
                    Replacement {
                        src: "O",
                        dst: "HH"
                    }
                ],
                molecule: "HOH".to_string()
            }
        )
    }

    #[test]
    fn test_find_indexes() {
        assert_eq!(find_indexes("OHOH", "H"), vec![1, 3]);
        assert_eq!(find_indexes("OHOH", "O"), vec![0, 2]);
        assert_eq!(find_indexes("OnHOnH", "On"), vec![0, 3]);
        assert_eq!(find_indexes("OnHeOnH", "On"), vec![0, 4]);
        assert_eq!(find_indexes("OnHeOnH", "He"), vec![2]);
    }

    #[test]
    fn test_replace_at() {
        assert_eq!(
            replace_at_index(
                "OnHOnH",
                &Replacement {
                    src: "On",
                    dst: "Ca"
                },
                0
            ),
            "CaHOnH"
        );
        assert_eq!(
            replace_at_index(
                "OnHOnH",
                &Replacement {
                    src: "On",
                    dst: "Ca"
                },
                3
            ),
            "OnHCaH"
        );
        assert_eq!(
            replace_at_index("HOHOHOHO", &Replacement { src: "O", dst: "N" }, 1),
            "HNHOHOHO"
        );
        assert_eq!(
            replace_at_index("HOHOHOHO", &Replacement { src: "O", dst: "N" }, 3),
            "HOHNHOHO"
        );
        assert_eq!(
            replace_at_index("HOHOHOHO", &Replacement { src: "O", dst: "N" }, 5),
            "HOHOHNHO"
        );
        assert_eq!(
            replace_at_index("HOHOHOHO", &Replacement { src: "O", dst: "N" }, 7),
            "HOHOHOHN"
        );
    }

    #[test]
    fn test_reverse() {
        let mut r = Replacement { src: "A", dst: "B" };
        r.reverse();
        assert_eq!(r, Replacement { src: "B", dst: "A" });
    }

    #[test]
    fn test_example_1() {
        let data = parse_input(EXAMPLE1);
        assert_eq!(
            possible_mutations(&data.molecule, &data.replacements).len(),
            4
        );
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day19.txt").unwrap();
        let data = parse_input(&input);
        assert_eq!(
            possible_mutations(&data.molecule, &data.replacements).len(),
            509
        );
    }

    #[test]
    fn test_example_2() {
        let data = parse_input(EXAMPLE2);
        assert_eq!(shortest_mutation(&data), 6);
    }

    #[test]
    #[ignore = "Not working / Too long"]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day19.txt").unwrap();
        let data = parse_input(&input);
        assert_eq!(shortest_mutation(&data), 6);
    }
}
