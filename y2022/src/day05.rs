#[derive(Debug, PartialEq, Eq)]
struct MoveOrder {
    amount: usize,
    src: usize,
    dst: usize,
}

#[derive(Debug)]
struct SupplyStacks {
    columns: Vec<Vec<char>>,
    move_orders: Vec<MoveOrder>,
}

impl SupplyStacks {
    fn new(data: &str) -> Self {
        let mut columns_data = vec![];
        let mut rows = data.split('\n').map(String::from);
        loop {
            let row = rows.next().unwrap();
            if row.trim().is_empty() {
                break;
            }
            columns_data.push(row);
        }
        columns_data.reverse();
        let mut columns = vec![];
        let n_columns: usize = columns_data[0]
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .max()
            .unwrap();
        for _ in 0..n_columns {
            columns.push(vec![]);
        }
        for row in &columns_data[1..] {
            for (i, c) in row[1..].chars().step_by(4).enumerate() {
                if c.is_alphabetic() {
                    columns[i].push(c);
                }
            }
        }
        let move_orders = rows
            .map(|row| {
                let values: Vec<usize> = row
                    .split_ascii_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                MoveOrder {
                    amount: values[0],
                    src: values[1] - 1,
                    dst: values[2] - 1,
                }
            })
            .collect();

        Self {
            columns,
            move_orders,
        }
    }
}

fn input(example: bool) -> SupplyStacks {
    const PATH: &str = "inputs/day05.txt";
    let data = if example {
        "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string()
    } else {
        std::fs::read_to_string(PATH).unwrap()
    };
    SupplyStacks::new(&data)
}

fn part_1(stack: &mut SupplyStacks) -> String {
    for move_order in &stack.move_orders {
        for _ in 0..move_order.amount {
            let moving_cargo = stack.columns[move_order.src].pop().unwrap();
            stack.columns[move_order.dst].push(moving_cargo);
        }
    }
    stack
        .columns
        .iter()
        .map(|col| col.last().unwrap())
        .collect()
}

fn part_2(stack: &mut SupplyStacks) -> String {
    for move_order in &stack.move_orders {
        let mut temp_storage = vec![];
        for _ in 0..move_order.amount {
            let moving_cargo = stack.columns[move_order.src].pop().unwrap();
            temp_storage.push(moving_cargo);
        }
        temp_storage.reverse();
        stack.columns[move_order.dst].append(&mut temp_storage);
    }
    stack
        .columns
        .iter()
        .map(|col| col.last().unwrap())
        .collect()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&mut input(true)), "CMZ");
}

#[test]
fn solution_1() {
    assert_eq!(part_1(&mut input(false)), "TBVFVDZPN");
}

#[test]
fn example_2() {
    assert_eq!(part_2(&mut input(true)), "MCD");
}

#[test]
fn solution_2() {
    assert_eq!(part_2(&mut input(false)), "VLCWHTDSZ");
}
