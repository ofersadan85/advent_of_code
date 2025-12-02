use advent_of_code_macros::aoc_tests;
use anyhow::{anyhow, Context, Result};
use std::{cmp::Ordering, collections::HashMap, iter::once, str::FromStr};

pub const EXAMPLE_RULE_LINE: &str = "ex{x>10:one,m<20:two,a>30:R,A}";
pub const EXAMPLE_PART_LINE: &str = "{x=787,m=2655,a=1222,s=2876}";
pub const EXAMPLE_FULL: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

type AllRules<'a> = HashMap<&'a str, RuleSet>;
type RangeMap = HashMap<(usize, usize), String>;
type ConditionRanges = (Option<(usize, usize)>, Option<(usize, usize)>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
    state: String,
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = s
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .filter_map(|part| {
                let (_, value) = part.split_once('=')?;
                value.parse().ok()
            })
            .collect();
        if numbers.len() != 4 {
            // The input is always ordered as x,m,a,s so we can just check the length
            return Err("Invalid number of fields");
        }
        Ok(Self {
            x: numbers[0],
            m: numbers[1],
            a: numbers[2],
            s: numbers[3],
            state: "in".to_string(), // Initial state
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangedPart {
    x: RangeMap,
    m: RangeMap,
    a: RangeMap,
    s: RangeMap,
}

impl Default for RangedPart {
    fn default() -> Self {
        let initial_map: RangeMap = once(((1, 4000), "in".to_string())).collect();
        Self {
            x: initial_map.clone(),
            m: initial_map.clone(),
            a: initial_map.clone(),
            s: initial_map,
        }
    }
}

impl RangedPart {
    fn is_finished(&self) -> bool {
        self.x.values().all(|target| target == "R" || target == "A")
            && self.m.values().all(|target| target == "R" || target == "A")
            && self.a.values().all(|target| target == "R" || target == "A")
            && self.s.values().all(|target| target == "R" || target == "A")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Field {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Field {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' => Ok(Self::X),
            'm' => Ok(Self::M),
            'a' => Ok(Self::A),
            's' => Ok(Self::S),
            _ => Err("Unknown field"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    field: Field,
    op: Ordering,
    value: usize,
    target: String,
}

impl Rule {
    fn eval(&self, p: &Part) -> bool {
        match self.field {
            Field::X => self.op == p.x.cmp(&self.value),
            Field::M => self.op == p.m.cmp(&self.value),
            Field::A => self.op == p.a.cmp(&self.value),
            Field::S => self.op == p.s.cmp(&self.value),
        }
    }

    fn eval_range(&self, start: usize, end: usize) -> ConditionRanges {
        use Ordering::{Equal, Greater, Less};
        match (self.op, start.cmp(&self.value), end.cmp(&self.value)) {
            (Equal, _, _) => unreachable!("Equal is not a valid operator"),
            (Greater, Greater, _) | (Less, _, Less) => (Some((start, end)), None),
            (Greater, _, Equal | Less) | (Less, Equal | Greater, _) => (None, Some((start, end))),
            (Less, Less, Equal | Greater) => {
                (Some((start, self.value - 1)), Some((self.value + 1, end)))
            }
            (Greater, Equal | Less, Greater) => {
                (Some((self.value + 1, end)), Some((start, self.value - 1)))
            }
        }
    }

    fn eval_range_for_field(&self, part: &mut RangedPart) -> Option<(usize, usize)> {
        let current_ranges = match self.field {
            Field::X => part.x.iter(),
            Field::M => part.m.iter(),
            Field::A => part.a.iter(),
            Field::S => part.s.iter(),
        };
        let mut new_ranges = HashMap::new();
        let mut result = None;
        for ((start, end), _) in current_ranges {
            let (condition_true, condition_false) = self.eval_range(*start, *end);
            if let Some((start_true, end_true)) = condition_true {
                new_ranges.insert((start_true, end_true), self.target.clone());
            }
            if condition_false.is_some() {
                result = condition_false;
            }
        }
        match self.field {
            Field::X => part.x = new_ranges,
            Field::M => part.m = new_ranges,
            Field::A => part.a = new_ranges,
            Field::S => part.s = new_ranges,
        }
        result
    }
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Input example: x>10:one
        let (condition, target) = s.split_once(':').ok_or("No target")?;
        let mut chars = condition.chars();
        let field = chars
            .next()
            .and_then(|c| Field::try_from(c).ok())
            .ok_or("Unknown field")?;
        let op = chars
            .next()
            .and_then(|c| match c {
                '<' => Some(Ordering::Less),
                '>' => Some(Ordering::Greater),
                _ => None,
            })
            .ok_or("Unknown operator")?;
        let value = chars.as_str().parse().map_err(|_| "Not a number")?;
        Ok(Self {
            field,
            op,
            value,
            target: target.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct RuleSet {
    rules: Vec<Rule>,
    final_target: String,
}

impl RuleSet {
    fn eval(&self, p: &Part) -> String {
        for rule in &self.rules {
            if rule.eval(p) {
                return rule.target.clone();
            }
        }
        self.final_target.clone()
    }

    fn eval_range(&self, part: &mut RangedPart) {
        let mut last_ranges = None;
        for rule in &self.rules {
            last_ranges = rule.eval_range_for_field(part);
        }
        if let Some((start, end)) = last_ranges {
            if let Some(last_rule) = self.rules.last() {
                match last_rule.field {
                    Field::X => part.x.insert((start, end), last_rule.target.clone()),
                    Field::M => part.m.insert((start, end), last_rule.target.clone()),
                    Field::A => part.a.insert((start, end), last_rule.target.clone()),
                    Field::S => part.s.insert((start, end), last_rule.target.clone()),
                };
            }
        }
    }
}

fn parse_rule_set(s: &str) -> Result<(&str, RuleSet)> {
    let (name, rules) = s.split_once('{').context("No rules")?;
    let mut parts: Vec<&str> = rules.split(',').collect();
    let final_target = parts
        .pop()
        .context("No final target")?
        .trim_end_matches('}');
    let rules = parts
        .iter()
        .filter_map(|rule| Rule::from_str(rule).ok())
        .collect();
    Ok((
        name,
        RuleSet {
            rules,
            final_target: final_target.to_string(),
        },
    ))
}

fn parse_input(input: &str) -> Result<(AllRules<'_>, Vec<Part>)> {
    let double_line_end = if input.contains('\r') {
        "\r\n\r\n"
    } else {
        "\n\n"
    };
    let (rules_str, parts_str) = input
        .split_once(double_line_end)
        .context("No double line break")?;
    let machine: AllRules = rules_str
        .lines()
        .filter_map(|line| parse_rule_set(line).ok())
        .collect();
    assert_eq!(rules_str.lines().count(), machine.len()); // Sanity check: we parsed all lines correctly
    let parts: Vec<Part> = parts_str
        .lines()
        .filter_map(|line| Part::from_str(line).ok())
        .collect();
    assert_eq!(parts_str.lines().count(), parts.len()); // Sanity check: we parsed all lines correctly
    Ok((machine, parts))
}

fn machine_process<H>(machine: &HashMap<&str, RuleSet, H>, parts: Vec<Part>) -> Result<usize>
where
    H: std::hash::BuildHasher,
{
    let mut parts = parts;
    let mut total_accepted = 0;
    while let Some(mut part) = parts.pop() {
        let rule_set = machine
            .get(part.state.as_str())
            .ok_or_else(|| anyhow!("Unknown rule"))?;
        let new_target = rule_set.eval(&part);
        if new_target == "A" {
            total_accepted += part.x + part.m + part.a + part.s;
        } else if new_target != "R" {
            part.state = new_target;
            parts.push(part);
        }
    }
    Ok(total_accepted)
}

fn machine_process_ranges<H>(machine: &HashMap<&str, RuleSet, H>) -> usize
where
    H: std::hash::BuildHasher,
{
    let mut part = RangedPart::default();
    while !part.is_finished() {
        for rule_set in machine.values() {
            rule_set.eval_range(&mut part);
        }
    }
    [part.x, part.m, part.a, part.s]
        .iter()
        .map(|field_map| {
            field_map
                .iter()
                .map(|((start, end), _)| end - start + 1)
                .sum::<usize>()
        })
        .product()
}

#[aoc_tests]
mod tests {
    #[test]
    #[ignore = "Too slow, need to debug"]
    fn part2_example() {
        let (machine, _) = parse_input(EXAMPLE_FULL).unwrap();
        let total_accepted = machine_process_ranges(&machine);
        assert_eq!(total_accepted, 167409079868000);
    }

    #[test]
    fn part1_example() {
        let (machine, parts) = parse_input(EXAMPLE_FULL).unwrap();
        let total_accepted = machine_process(&machine, parts).unwrap();
        assert_eq!(total_accepted, 19114);
    }

    #[test]
    fn part1() {
        let (machine, parts) = parse_input(include_str!("../../inputs/2023/day19.txt")).unwrap();
        let total_accepted = machine_process(&machine, parts).unwrap();
        assert_eq!(total_accepted, 362930);
    }

    #[test]
    fn rule_set_eval() {
        let (machine, mut parts) = parse_input(EXAMPLE_FULL).unwrap();
        let mut output = vec![];
        while parts[0].state != "R" && parts[0].state != "A" {
            let rule_set = machine
                .get(parts[0].state.as_str())
                .ok_or_else(|| anyhow!("Unknown rule"))
                .unwrap();
            parts[0].state = rule_set.eval(&parts[0]);
            output.push(parts[0].state.clone());
        }
        assert_eq!(output, vec!["qqz", "qs", "lnx", "A"]);
    }

    #[test]
    fn parse_full() {
        let (rules, parts) = parse_input(EXAMPLE_FULL).unwrap();
        assert_eq!(rules.len(), 11);
        assert_eq!(parts.len(), 5);
    }

    #[test]
    fn parse_part() {
        let part = Part::from_str(EXAMPLE_PART_LINE).unwrap();
        let expected = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
            state: "in".to_string(), // Initial state
        };
        assert_eq!(part, expected);
    }

    #[test]
    fn parse_rule() {
        let rule = Rule::from_str("x>10:one").unwrap();
        let expected = Rule {
            field: Field::X,
            op: Ordering::Greater,
            value: 10,
            target: "one".to_string(),
        };
        assert_eq!(rule, expected);
    }

    #[test]
    fn rule_eval_range() {
        let rule = Rule::from_str("x>10:one").unwrap();
        let range_result = rule.eval_range(1, 4000);
        assert_eq!(range_result, (Some((11, 4000)), Some((1, 9))));
    }
}
