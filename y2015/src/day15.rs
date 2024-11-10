use std::collections::HashSet;

use anyhow::{anyhow, Context};
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct Ingredient {
    name: String,
    capacity: f64,
    durability: f64,
    flavor: f64,
    texture: f64,
    calories: f64,
    quantity: u32,
}

impl std::str::FromStr for Ingredient {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let name = split
            .next()
            .ok_or_else(|| anyhow!("No name"))?
            .trim_end_matches(':')
            .to_string();
        let capacity = split
            .nth(1)
            .ok_or_else(|| anyhow!("No capacity"))?
            .trim_end_matches(',')
            .parse()
            .context("capacity")?;
        let durability = split
            .nth(1)
            .ok_or_else(|| anyhow!("No durability"))?
            .trim_end_matches(',')
            .parse()
            .context("durability")?;
        let flavor = split
            .nth(1)
            .ok_or_else(|| anyhow!("No flavor"))?
            .trim_end_matches(',')
            .parse()
            .context("flavor")?;
        let texture = split
            .nth(1)
            .ok_or_else(|| anyhow!("No texture"))?
            .trim_end_matches(',')
            .parse()
            .context("texture")?;
        let calories = split
            .nth(1)
            .ok_or_else(|| anyhow!("No calories"))?
            .trim_end_matches(',')
            .parse()
            .context("calories")?;
        Ok(Self {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
            quantity: 1,
        })
    }
}

impl Ingredient {
    fn set_quantity(&mut self, quantity: u32) {
        let new_quantity = f64::from(quantity);
        let old_quantity = f64::from(self.quantity);
        let ratio = new_quantity / old_quantity;
        self.capacity *= ratio;
        self.durability *= ratio;
        self.flavor *= ratio;
        self.texture *= ratio;
        self.calories *= ratio;
        self.quantity = quantity;
    }
}

impl std::ops::Add for &Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl std::ops::Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            name: format!("{} + {}", self.name, rhs.name),
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
            quantity: self.quantity + rhs.quantity,
        }
    }
}

impl std::ops::Add<&Self> for Ingredient {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            name: format!("{} + {}", self.name, rhs.name),
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
            quantity: self.quantity + rhs.quantity,
        }
    }
}

fn parse_input(s: &str) -> Vec<Ingredient> {
    s.lines().map(|s| s.parse().unwrap()).collect()
}

fn cookie_value(cookie: &[Ingredient]) -> f64 {
    let mut final_cookie = cookie.first().unwrap().clone();
    for i in 1..cookie.len() {
        final_cookie = final_cookie + cookie.get(i).unwrap();
    }

    final_cookie.capacity.max(0.0)
        * final_cookie.durability.max(0.0)
        * final_cookie.flavor.max(0.0)
        * final_cookie.texture.max(0.0)
}

#[allow(clippy::cast_possible_truncation)]
fn get_optimal(cookie: &mut [Ingredient], max: usize) -> f64 {
    let mut perm = HashSet::new();
    (1..=max)
        .combinations_with_replacement(cookie.len())
        .filter(|v| v.iter().sum::<usize>() == max)
        .for_each(|c| {
            let local_perm = c.iter().copied().permutations(cookie.len());
            perm.extend(local_perm);
        });
    let mut max = 0.0;
    for p in &perm {
        for (i, _) in p.iter().enumerate() {
            // We can unwrap here because we know it's within the bounds
            cookie.get_mut(i).unwrap().set_quantity(p[i] as u32);
        }
        let value: f64 = cookie_value(cookie);
        if value > max {
            max = value;
        }
    }
    max
}

#[allow(clippy::float_cmp, clippy::cast_possible_truncation)]
fn get_optimal_calories(cookie: &mut [Ingredient], max: usize, calories: f64) -> f64 {
    let mut perm = HashSet::new();
    (1..=max)
        .combinations_with_replacement(cookie.len())
        .filter(|v| v.iter().sum::<usize>() == max)
        .for_each(|c| {
            let local_perm = c.iter().copied().permutations(cookie.len());
            perm.extend(local_perm);
        });
    let mut max = 0.0;
    for p in &perm {
        for (i, _) in p.iter().enumerate() {
            // We can unwrap here because we know it's within the bounds
            cookie.get_mut(i).unwrap().set_quantity(p[i] as u32);
        }
        let cookie_calories: f64 = cookie.iter().map(|c| c.calories).sum();
        if cookie_calories.round() == calories {
            let value: f64 = cookie_value(cookie);
            if value > max {
                max = value;
            }
        }
    }
    max
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    const EXAMPLE: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_parse() {
        let examples = parse_input(EXAMPLE);
        assert_eq!(examples.len(), EXAMPLE.lines().count(), "lines");
        assert_eq!(
            examples,
            vec![
                Ingredient {
                    name: "Butterscotch".to_string(),
                    capacity: -1.0,
                    durability: -2.0,
                    flavor: 6.0,
                    texture: 3.0,
                    calories: 8.0,
                    quantity: 1,
                },
                Ingredient {
                    name: "Cinnamon".to_string(),
                    capacity: 2.0,
                    durability: 3.0,
                    flavor: -2.0,
                    texture: -1.0,
                    calories: 3.0,
                    quantity: 1,
                }
            ]
        );
    }

    #[test]
    fn test_ops() {
        let mut examples = parse_input(EXAMPLE);
        examples.get_mut(0).unwrap().set_quantity(44);
        examples.get_mut(1).unwrap().set_quantity(56);
        let cookie = examples.first().unwrap() + examples.get(1).unwrap();
        assert_eq!(
            cookie,
            Ingredient {
                name: "Butterscotch + Cinnamon".to_string(),
                capacity: 68.0,
                durability: 80.0,
                flavor: 152.0,
                texture: 76.0,
                calories: 520.0,
                quantity: 100
            },
            "{cookie:?}"
        );
        let examples2 = parse_input(EXAMPLE);
        examples.get_mut(0).unwrap().set_quantity(1);
        examples.get_mut(1).unwrap().set_quantity(1);
        assert_eq!(examples, examples2);
    }

    #[test]
    fn test_optimal() {
        // let mut examples = parse_input(&read_to_string("../inputs/2015/day15.txt").unwrap());
        let mut examples = parse_input(EXAMPLE);
        let optimal = get_optimal(&mut examples, 100);
        assert_eq!(optimal.round(), 62842880.0);
    }

    #[test]
    fn test_part_1() {
        let mut input = parse_input(&read_to_string("../inputs/2015/day15.txt").unwrap());
        let optimal = get_optimal(&mut input, 100);
        assert_eq!(optimal.round(), 18965440.0);
    }

    #[test]
    fn test_optimal_calories() {
        // let mut examples = parse_input(&read_to_string("../inputs/2015/day15.txt").unwrap());
        let mut examples = parse_input(EXAMPLE);
        let optimal = get_optimal_calories(&mut examples, 100, 500.0);
        assert_eq!(optimal.round(), 57600000.0);
    }

    #[test]
    fn test_part_2() {
        let mut input = parse_input(&read_to_string("../inputs/2015/day15.txt").unwrap());
        let optimal = get_optimal_calories(&mut input, 100, 500.0);
        assert_eq!(optimal.round(), 15862900.0);
    }
}
