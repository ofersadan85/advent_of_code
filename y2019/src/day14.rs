use std::collections::HashMap;

use advent_of_code_macros::aoc_solver;

#[derive(Debug)]
struct RecipeItem<'a> {
    name: &'a str,
    quantity: usize,
}

impl<'a> RecipeItem<'a> {
    fn from_str(s: &'a str) -> Result<Self, String> {
        let (quantity_str, name) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| format!("Invalid recipe item format: {s}"))?;
        Ok(Self {
            name,
            quantity: quantity_str
                .parse()
                .map_err(|e| format!("Invalid quantity: {e}"))?,
        })
    }
}

#[derive(Debug)]
struct Recipe<'a> {
    output: RecipeItem<'a>,
    inputs: Vec<RecipeItem<'a>>,
}

impl<'a> Recipe<'a> {
    fn from_str(s: &'a str) -> Result<Self, String> {
        let (inputs_str, output_str) = s
            .split_once(" => ")
            .ok_or_else(|| format!("Invalid recipe format: {s}"))?;
        let inputs: Result<Vec<RecipeItem<'a>>, String> =
            inputs_str.split(", ").map(RecipeItem::from_str).collect();
        Ok(Self {
            output: RecipeItem::from_str(output_str)?,
            inputs: inputs?,
        })
    }
}

fn parse_input(input: &str) -> HashMap<&str, Recipe<'_>> {
    input
        .lines()
        .map(Recipe::from_str)
        .collect::<Result<Vec<Recipe>, _>>()
        .expect("Invalid recipe format")
        .into_iter()
        .map(|recipe| (recipe.output.name, recipe))
        .collect()
}

fn ore_from_fuel(recipes: &HashMap<&str, Recipe>, fuel_quantity: usize) -> usize {
    let mut target: HashMap<&str, usize> = HashMap::new();
    target.insert("FUEL", fuel_quantity);
    let mut ore_needed = 0;
    let mut surplus: HashMap<&str, usize> = HashMap::new();
    while !target.is_empty() {
        let (name, quantity) = target.iter().next().map(|(n, q)| (*n, *q)).unwrap();
        target.remove(name);
        let recipe = recipes.get(name).expect("Recipe not found");
        let surplus_quantity = surplus.get(name).copied().unwrap_or(0);
        let quantity_needed = if quantity > surplus_quantity {
            quantity - surplus_quantity
        } else {
            surplus.insert(name, surplus_quantity - quantity);
            continue;
        };
        let times = quantity_needed.div_ceil(recipe.output.quantity);
        for input in &recipe.inputs {
            let input_quantity = input.quantity * times;
            if input.name == "ORE" {
                ore_needed += input_quantity;
            } else {
                *target.entry(input.name).or_insert(0) += input_quantity;
            }
        }
        let produced_quantity = recipe.output.quantity * times;
        let new_surplus = produced_quantity - quantity_needed;
        surplus.insert(name, new_surplus);
    }
    ore_needed
}

/// Binary search to find the maximum amount of fuel that can be produced with the given amount of ore.
fn fuel_from_ore(recipes: &HashMap<&str, Recipe>, ore_quantity: usize) -> usize {
    let mut low = 0;
    let mut high = ore_quantity;
    while low < high {
        let mid = (low + high).div_ceil(2);
        let ore_needed = ore_from_fuel(recipes, mid);
        if ore_needed <= ore_quantity {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

const EXAMPLE1: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

const EXAMPLE2: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

const EXAMPLE3: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

const EXAMPLE4: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

#[aoc_solver(suffix="example1", input = EXAMPLE1, expected = 165)]
#[aoc_solver(suffix="example2", input = EXAMPLE2, expected = 13312)]
#[aoc_solver(suffix="example3", input = EXAMPLE3, expected = 180697)]
#[aoc_solver(suffix="example4", input = EXAMPLE4, expected = 2210736)]
#[aoc_solver(file = "inputs/2019/day14.txt", expected = 532506)]
fn part_1(input: &str) -> usize {
    let recipes = parse_input(input);
    ore_from_fuel(&recipes, 1)
}

#[aoc_solver(suffix="example2", input = EXAMPLE2, expected = 82892753)]
#[aoc_solver(suffix="example3", input = EXAMPLE3, expected = 5586022)]
#[aoc_solver(suffix="example4", input = EXAMPLE4, expected = 460664)]
#[aoc_solver(file = "inputs/2019/day14.txt", expected = 2595245)]
fn part_2(input: &str) -> usize {
    let recipes = parse_input(input);
    fuel_from_ore(&recipes, 1_000_000_000_000)
}
