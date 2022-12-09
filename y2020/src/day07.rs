use std::{cell::RefCell, collections::HashMap, rc::Rc};

const PATH: &str = "inputs/day07.txt";
const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
const DELIMITER: char = '@';

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    can_contain: Vec<(usize, BagRef)>,
}

impl Bag {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            can_contain: Vec::new(),
        }
    }
}

type BagRef = Rc<RefCell<Bag>>;
type BagMap = HashMap<String, BagRef>;

fn input(example: bool) -> BagMap {
    let first_splitter = format!(" {} contain ", DELIMITER);
    let second_splitter = format!(" {} ", DELIMITER);
    let data = if example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    }
    .replace("bags", &DELIMITER.to_string())
    .replace("bag", &DELIMITER.to_string())
    .replace(',', "")
    .replace(&format!(" {DELIMITER}."), "")
    .trim_end_matches(&[DELIMITER, '.', ' '])
    .to_string();
    let data: Vec<(&str, &str)> = data
        .split('\n')
        .map(|row| row.split_once(&first_splitter).unwrap())
        .collect();
    let bag_map: BagMap = data
        .iter()
        .map(|(bag_name, _)| {
            (
                bag_name.to_string(),
                Rc::new(RefCell::new(Bag::new(bag_name))),
            )
        })
        .collect();
    for (bag_name, content) in data {
        bag_map
            .get(bag_name)
            .unwrap()
            .borrow_mut()
            .can_contain
            .extend(content.split(&second_splitter).flat_map(|part| {
                let (number, inner_name) = part.split_once(' ').unwrap();
                Some((number.parse().ok()?, bag_map.get(inner_name)?.clone()))
            }))
    }
    bag_map
}

#[test]
fn test_input() {
    for bag in input(true) {
        println!("{:?}", bag);
        println!("******************");
    }
    todo!("Just testing the output")
}
