use crate::day11_part1::{last_number, EXAMPLE, PATH};
use advent_of_code_common::file::lines_as_blocks;
use std::{cell::RefCell, rc::Rc};

type MonkeyRef = Rc<RefCell<EfficientMonkey>>;
type ItemRef = Rc<RefCell<EfficientItem>>;

struct EfficientMonkey {
    items: Vec<ItemRef>,
    operation: String,
    divisor: u128,
    target_true: Option<MonkeyRef>,
    target_false: Option<MonkeyRef>,
    inspect_count: u128,
}

impl EfficientMonkey {
    fn from_block(block: &[String]) -> MonkeyRef {
        let operation = block[2].split(" = ").last().unwrap().to_string();
        let divisor = last_number(&block[3]);
        let raw_monkey = Self {
            items: Vec::new(),
            operation,
            divisor,
            target_true: None,
            target_false: None,
            inspect_count: 0,
        };
        Rc::new(RefCell::new(raw_monkey))
    }
}

struct EfficientItem {
    value: u128,
    owner: MonkeyRef,
}

impl EfficientItem {
    fn new(value: u128, owner: MonkeyRef) -> ItemRef {
        let raw_item = Self { value, owner };
        Rc::new(RefCell::new(raw_item))
    }
}

struct KeepAway {
    monkeys: Vec<MonkeyRef>,
    items: Vec<ItemRef>,
}

fn input(example: bool) -> KeepAway {
    let path = if example { EXAMPLE } else { PATH };
    let blocks = lines_as_blocks(&std::fs::read_to_string(path).unwrap());
    let monkeys: Vec<MonkeyRef> = blocks
        .iter()
        .map(|b| EfficientMonkey::from_block(b))
        .collect();
    let mut items: Vec<ItemRef> = Vec::new();
    for (i, b) in blocks.iter().enumerate() {
        monkeys[i].borrow_mut().items = b[1]
            .replace(',', "")
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .map(|v| EfficientItem::new(v, monkeys[i].clone()))
            .collect();
        items.extend(monkeys[i].borrow().items.iter().cloned());
        let target_true_index: usize = last_number(&b[4]).try_into().unwrap();
        monkeys[i].borrow_mut().target_true = Some(monkeys[target_true_index].clone());
        let target_false_index: usize = last_number(&b[5]).try_into().unwrap();
        monkeys[i].borrow_mut().target_false = Some(monkeys[target_false_index].clone());
    }
    KeepAway { monkeys, items }
}

// #[test]
// fn example_2() {
//     let mut monkeys = input(true);
//     assert_eq!(part_2(&mut monkeys), 2713310158);
// }

// #[test]
// fn task_2() {
//     let mut monkeys = input(false);
//     assert_eq!(part_2(&mut monkeys), 0);
// }
