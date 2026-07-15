use crate::intcode::{IntcodeComputer, State};
use advent_of_code_common::coords::{Direction, Point};
use advent_of_code_macros::aoc_solver;
use std::collections::HashMap;

struct PaintingRobot {
    direction: Direction,
    position: Point,
    colors: HashMap<Point, isize>,
}

impl Default for PaintingRobot {
    fn default() -> Self {
        Self {
            direction: Direction::North,
            position: Point::default(),
            colors: HashMap::new(),
        }
    }
}

impl PaintingRobot {
    fn run(&mut self, computer: &mut IntcodeComputer, starting_color: isize) {
        self.colors.insert(self.position, starting_color);
        while computer.state != State::Halted {
            let input_color = *self.colors.get(&self.position).unwrap_or(&0);
            computer.queue_input(input_color);
            computer.run();
            let color_output = computer.output.pop_front().expect("color output");
            let turn_output = computer.output.pop_front().expect("turn output");
            self.colors.insert(self.position, color_output);
            self.direction = match turn_output {
                0 => self.direction.turn_cw_270(),
                1 => self.direction.turn_cw_90(),
                _ => unreachable!("invalid turn output"),
            };
            self.position += self.direction;
        }
    }
}

impl std::fmt::Display for PaintingRobot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.colors.keys().map(|p| p.x).min().unwrap_or(isize::MIN);
        let max_x = self.colors.keys().map(|p| p.x).max().unwrap_or(isize::MAX);
        let min_y = self.colors.keys().map(|p| p.y).min().unwrap_or(isize::MIN);
        let max_y = self.colors.keys().map(|p| p.y).max().unwrap_or(isize::MAX);
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let color = *self.colors.get(&Point { x, y }).unwrap_or(&0);
                let c = match color {
                    0 => ' ',
                    1 => '#',
                    _ => unreachable!("invalid color"),
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_solver(file = "inputs/2019/day11.txt", expected = 1951)]
fn part_1(input: &str) -> usize {
    let mut computer: IntcodeComputer = input.parse().expect("valid program");
    let mut robot = PaintingRobot::default();
    robot.run(&mut computer, 0);
    robot.colors.len()
}

const PART_2_EXPECTED: &str = "
 #  # #  #   ## ###   ##  #  #  ##  ###    
 #  # # #     # #  # #  # #  # #  # #  #   
 #### ##      # ###  #  # #### #    #  #   
 #  # # #     # #  # #### #  # #    ###    
 #  # # #  #  # #  # #  # #  # #  # # #    
 #  # #  #  ##  ###  #  # #  #  ##  #  #   "; // HKJBAHCR

#[aoc_solver(file = "inputs/2019/day11.txt", expected = PART_2_EXPECTED)]
fn part_2(input: &str) -> String {
    let mut computer: IntcodeComputer = input.parse().expect("valid program");
    let mut robot = PaintingRobot::default();
    robot.run(&mut computer, 1);
    let s = robot.to_string();
    let mut s: Vec<&str> = s.lines().rev().collect(); // output is upside down, so reverse the lines
    s.insert(0, ""); // add a blank line at the top to match the expected output
    // println!("{}", s.join("\n"));
    s.join("\n")
}
