use advent_of_code_macros::{aoc_tests, char_enum};

trait NextMoves {
    fn next_moves(&self, destination: &Self) -> Vec<Directional>;
}

#[char_enum(display)]
enum Numeric {
    Zero = '0',
    One = '1',
    Two = '2',
    Three = '3',
    Four = '4',
    Five = '5',
    Six = '6',
    Seven = '7',
    Eight = '8',
    Nine = '9',
    Push = 'A',
}

impl NextMoves for Numeric {
    fn next_moves(&self, destination: &Self) -> Vec<Directional> {
        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        use Directional::{Down, Left, Right, Up};
        use Numeric::{Eight, Five, Four, Nine, One, Push, Seven, Six, Three, Two, Zero};
        let moves_up = match (self, destination) {
            (a, b) if a == b => return vec![Directional::Push],
            (Zero | Push, One | Two | Three)
            | (One | Two | Three, Four | Five | Six)
            | (Four | Five | Six, Seven | Eight | Nine) => vec![Up],
            (Zero | Push, Four | Five | Six) | (One | Two | Three, Seven | Eight | Nine) => {
                vec![Up, Up]
            }
            (Zero | Push, Seven | Eight | Nine) => vec![Up, Up, Up],
            _ => vec![],
        };
        let moves_right = match (self, destination) {
            (One | Four | Seven, Zero | Two | Five | Eight)
            | (Zero | Two | Five | Eight, Push | Three | Six | Nine) => vec![Right],
            (One | Four | Seven, Push | Three | Six | Nine) => vec![Right, Right],
            _ => vec![],
        };
        let moves_down = match (self, destination) {
            (Seven | Eight | Nine, Four | Five | Six)
            | (Four | Five | Six, One | Two | Three)
            | (One | Two | Three, Zero | Push) => vec![Down],
            (Seven | Eight | Nine, One | Two | Three) | (Four | Five | Six, Zero | Push) => {
                vec![Down, Down]
            }
            (Seven | Eight | Nine, Zero | Push) => vec![Down, Down, Down],
            _ => vec![],
        };
        let moves_left = match (self, destination) {
            (Zero | Two | Five | Eight, One | Four | Seven)
            | (Push | Three | Six | Nine, Zero | Two | Five | Eight) => vec![Left],
            (Push | Three | Six | Nine, One | Four | Seven) => vec![Left, Left],
            _ => vec![],
        };
        let mut moves = moves_up;
        moves.extend(moves_right);
        moves.extend(moves_down);
        moves.extend(moves_left);
        moves.push(Directional::Push);
        moves
    }
}

#[char_enum(display)]
enum Directional {
    Up = '^',
    Down = 'v',
    Left = '<',
    Right = '>',
    Push = 'A',
}

impl NextMoves for Directional {
    fn next_moves(&self, destination: &Self) -> Vec<Self> {
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        use Directional::{Down, Left, Push, Right, Up};
        // let moves_right = match (self, destination) {
        //     (a, b) if a == b => return vec![],
        //     (Left, Right|Push) => vec![Right, Right],
        //     (Left, Down) | (Up, Push) | (Down, Right) => vec![Right],
        //     _ => vec![],
        // };
        // let moves_down = match (self, destination) {
        //     (Push | Up, Left | Down | Right) => vec![Down],
        //     _ => vec![],
        // };
        // let moves_up = match (self, destination) {
        //     (Left | Down | Right, Up | Push) => vec![Up],
        //     _ => vec![],
        // };
        // let moves_left = match (self, destination) {
        //     (Right | Push, Left) => vec![Left, Left],
        //     (Right | Push, Up | Down) | (Up | Down, Left) => vec![Left],
        //     _ => vec![],
        // };
        // let mut moves = moves_right;
        // moves.extend(moves_down);
        // moves.extend(moves_up);
        // moves.extend(moves_left);
        // moves
        let mut moves = match (self, destination) {
            (Push, Push) | (Right, Right) | (Left, Left) | (Down, Down) | (Up, Up) => vec![],
            (Up, Down) | (Push, Right) => vec![Down],
            (Up, Left) | (Push, Down) => vec![Down, Left],
            (Push, Up) | (Down, Left) | (Right, Down) => vec![Left],
            (Up, Right) => vec![Right, Down],
            (Up, Push) | (Down, Right) | (Left, Down) => vec![Right],
            (Down, Push) | (Left, Up) => vec![Right, Up],
            (Down, Up) | (Right, Push) => vec![Up],
            (Left, Right) => vec![Right, Right],
            (Left, Push) => vec![Right, Right, Up],
            (Right, Up) => vec![Up, Left],
            (Right, Left) => vec![Left, Left],
            (Push, Left) => vec![Down, Left, Left],
        };
        moves.push(Push);
        moves
    }
}

fn all_next_moves(seq: &[Directional]) -> Vec<Directional> {
    let mut previous = Directional::Push;
    let mut result = vec![];
    for direction in seq {
        result.extend(previous.next_moves(direction).iter().copied());
        previous = *direction;
    }
    result
}

fn all_numeric_moves(seq: &[Numeric]) -> Vec<Directional> {
    let mut previous = Numeric::Push;
    let mut result = vec![];
    for number in seq {
        result.extend(previous.next_moves(number).iter().copied());
        previous = *number;
    }
    result
}

fn dir_vec(seq: &str) -> Vec<Directional> {
    seq.chars()
        .filter_map(|c| Directional::try_from(c).ok())
        .collect()
}

fn num_vec(seq: &str) -> Vec<Numeric> {
    seq.chars()
        .filter_map(|c| Numeric::try_from(c).ok())
        .collect()
}

fn full_directions(seq: &str, robots: usize) -> Vec<Directional> {
    let mut next_step = all_numeric_moves(&num_vec(seq));
    for _ in 0..robots {
        next_step = all_next_moves(&next_step);
    }
    next_step
}

fn full_values(seq: &str, robots: usize) -> usize {
    let steps = full_directions(seq, robots);
    let value: usize = seq.replace('A', "").parse().expect("Number parse error");
    let steps_str = steps.iter().map(ToString::to_string).collect::<String>();
    dbg!(steps_str);
    // dbg!(&steps.len());
    // dbg!(&value);
    steps.len() * value
}

#[aoc_tests]
mod tests {
    #[test]
    fn dir_to_dir() {
        let step0 = "029A";
        let step1 = "<A^A>^^AvvvA";
        let step2 = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let step3 = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
        let result0 = all_numeric_moves(&num_vec(step0));
        assert_eq!(result0.len(), step1.len(), "Length mismatch 0-1");
        let result1 = all_next_moves(&dir_vec(step1));
        assert_eq!(result1.len(), step2.len(), "Length mismatch 1-2");
        let result2 = all_next_moves(&dir_vec(step2));
        assert_eq!(result2.len(), step3.len(), "Length mismatch 2-3");
        let result2from1 = all_next_moves(&result1);
        assert_eq!(result2.len(), result2from1.len(), "Length Mismatch 1-3");
    }

    #[test]
    #[ignore = "Failed"]
    fn example_1() {
        assert_eq!(full_values("029A", 2), 68 * 29, "Example 1");
        assert_eq!(full_values("980A", 2), 60 * 980, "Example 2");
        assert_eq!(full_values("179A", 2), 68 * 179, "Example 3");
        assert_eq!(full_values("456A", 2), 64 * 456, "Example 4");
        let steps_str = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        dbg!(steps_str);
        assert_eq!(full_values("379A", 2), 64 * 379, "Example 5");
    }

    #[test]
    #[ignore = "Failed"]
    fn part_1() {
        let result = read_input().lines().map(|l| full_values(l, 2)).sum::<usize>();
        assert_eq!(result, 0, "Part 1");
    }

    #[test]
    fn test_move_coverage() {
        use Numeric::*;
        let options = [
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Numeric::Push,
        ];
        itertools::iproduct!(options.iter(), options.iter())
            .map(|(a, b)| (a, b, a.next_moves(b)))
            .for_each(|(a, b, c)| {
                println!(
                    "{:?} -> {:?} = {:?}",
                    a,
                    b,
                    c.iter().map(|d| d.to_string()).collect::<String>()
                );
            });
        use Directional::*;
        let options = [Up, Down, Left, Right, Directional::Push];
        itertools::iproduct!(options.iter(), options.iter())
            .map(|(a, b)| (a, b, a.next_moves(b)))
            .for_each(|(a, b, c)| {
                println!(
                    "{:?} -> {:?} = {:?}",
                    a,
                    b,
                    c.iter().map(|d| d.to_string()).collect::<String>()
                );
            });
        // assert!(false);
    }
}
