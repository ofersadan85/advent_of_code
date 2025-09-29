use std::{cmp::Reverse, str::FromStr};

#[derive(Debug)]
struct RacePlan {
    id: char,
    plan: Vec<isize>,
    len: usize,
    power: isize,
}

impl FromStr for RacePlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = "Invalid input";
        let (id, plan) = s.split_once(':').ok_or_else(|| err)?;
        let id = id.chars().next().ok_or_else(|| err)?;
        let plan = plan
            .split(',')
            .map(|s| match s {
                "+" => Ok(1),
                "-" => Ok(-1),
                "=" => Ok(0),
                _ => Err(err),
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            id,
            plan,
            len: 0,
            power: 10,
        })
    }
}

impl RacePlan {
    fn total(&self) -> isize {
        let segments = (self.len / self.plan.len()) as isize;
        let rem = self.len % self.plan.len();
        self.plan.iter().sum::<isize>() * segments + self.plan[..rem].iter().sum::<isize>()
    }
}

fn parse_input(input: &str, segment_len: usize) -> Vec<RacePlan> {
    input
        .lines()
        .map(|line| {
            let mut plan: RacePlan = line.parse().expect("Invalid input");
            plan.len = segment_len;
            plan
        })
        .collect()
}

fn rank_plans(plans: &mut [RacePlan]) -> String {
    plans.sort_unstable_by_key(|p| Reverse(p.total()));
    plans.iter().map(|p| p.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse_input() {
        let input = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";
        let mut plans = parse_input(input, 10);
        assert_eq!(rank_plans(&mut plans), "BDCA");
        let input = read_to_string("../inputs/everybody/quest06part1.txt").expect("input file");
        let mut plans = parse_input(&input, 10);
        // for plan in &plans {
        //     let segments = (plan.len / plan.plan.len()) as isize;
        //     let rem = plan.len % plan.plan.len();
        //     let total = plan.plan.iter().sum::<isize>() * segments + plan.plan[..rem].iter().sum::<isize>();

        //     println!("{} {:?} {} ({segments} segments + {rem}) sum {}", plan.id, plan.plan, total, plan.plan.iter().sum::<isize>());
        // }
        assert_eq!(rank_plans(&mut plans), "HJGK");
    }
}
