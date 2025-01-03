use advent_of_code_macros::aoc_tests;

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

impl TryFrom<&str> for SupplyStacks {
    type Error = &'static str;

    fn try_from(data: &str) -> Result<Self, Self::Error> {
        let mut columns_data = vec![];
        let mut rows = data.split('\n').map(String::from);
        loop {
            let row = rows.next().ok_or("Invalid row")?;
            if row.trim().is_empty() {
                break;
            }
            columns_data.push(row);
        }
        columns_data.reverse();
        let mut columns = vec![];
        let n_columns: usize = columns_data[0]
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .max()
            .ok_or("Invalid row")?;
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

        Ok(Self {
            columns,
            move_orders,
        })
    }
}

fn part_1(stack: &mut SupplyStacks) -> String {
    for move_order in &stack.move_orders {
        let additional_cargo: Vec<_> = (0..move_order.amount)
            .filter_map(|_| stack.columns[move_order.src].pop())
            .collect();
        stack.columns[move_order.dst].extend(additional_cargo);
    }
    stack.columns.iter().filter_map(|col| col.last()).collect()
}

fn part_2(stack: &mut SupplyStacks) -> String {
    for move_order in &stack.move_orders {
        let mut temp_storage: Vec<_> = (0..move_order.amount)
            .filter_map(|_| stack.columns[move_order.src].pop())
            .collect();
        temp_storage.reverse();
        stack.columns[move_order.dst].append(&mut temp_storage);
    }
    stack.columns.iter().filter_map(|col| col.last()).collect()
}

#[aoc_tests]
mod tests {
    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn example_1() {
        let mut input = SupplyStacks::try_from(EXAMPLE).unwrap();
        assert_eq!(part_1(&mut input), "CMZ");
    }

    #[test]
    fn solution_1() {
        let mut input = SupplyStacks::try_from(read_input().as_str()).unwrap();
        assert_eq!(part_1(&mut input), "TBVFVDZPN");
    }

    #[test]
    fn example_2() {
        let mut input = SupplyStacks::try_from(EXAMPLE).unwrap();
        assert_eq!(part_2(&mut input), "MCD");
    }

    #[test]
    fn solution_2() {
        let mut input = SupplyStacks::try_from(read_input().as_str()).unwrap();
        assert_eq!(part_2(&mut input), "VLCWHTDSZ");
    }
}
