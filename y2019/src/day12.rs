use advent_of_code_macros::aoc_solver;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Default, Debug)]
struct V3 {
    x: isize,
    y: isize,
    z: isize,
}

trait Energy {
    fn energy(&self) -> isize;
}

impl Energy for V3 {
    fn energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug)]
struct Moon {
    position: V3,
    velocity: V3,
}

macro_rules! apply_axis_gravity {
    ($self:expr, $other:expr, $axis:ident) => {
        if $self.position.$axis < $other.position.$axis {
            $self.velocity.$axis += 1;
        } else if $self.position.$axis > $other.position.$axis {
            $self.velocity.$axis -= 1;
        }
    };
}

impl FromStr for Moon {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<isize> = s
            .trim_matches(&['<', '>'])
            .splitn(3, ", ")
            .filter_map(|p| p.split_once('=').map(|x| x.1))
            .filter_map(|p| p.parse().ok())
            .collect();
        let parts: [isize; 3] = parts.try_into().map_err(|_| "expected 3 parts")?;
        Ok(Self {
            position: V3 {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            },
            velocity: V3::default(),
        })
    }
}

impl Moon {
    fn apply_gravity(&mut self, other: &Self) {
        apply_axis_gravity!(self, other, x);
        apply_axis_gravity!(self, other, y);
        apply_axis_gravity!(self, other, z);
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

impl Energy for Moon {
    fn energy(&self) -> isize {
        self.position.energy() * self.velocity.energy()
    }
}

fn physics_step(moons: &mut [Moon]) {
    for (i, j) in (0..moons.len()).tuple_combinations() {
        let (slice_a, slice_b) = moons.split_at_mut(j);
        let moon_a = &mut slice_a[i];
        let moon_b = &mut slice_b[0];
        moon_a.apply_gravity(moon_b);
        moon_b.apply_gravity(moon_a);
    }
    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

#[aoc_solver(file = "inputs/2019/day12.txt", expected = 7013)]
fn part_1(input: &str) -> isize {
    let mut moons: Vec<Moon> = input
        .lines()
        .map(|l| l.parse().expect("valid moon"))
        .collect();
    for _ in 0..1000 {
        physics_step(&mut moons);
    }
    moons.iter().map(|m| m.energy()).sum()
}

const EXAMPLE1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

#[aoc_solver(input = EXAMPLE1, expected = 179)]
// #[aoc_solver(file = "inputs/2019/day12.txt", expected = 7013)]
fn part_2(input: &str) -> isize {
    let mut moons: Vec<Moon> = input
        .lines()
        .map(|l| l.parse().expect("valid moon"))
        .collect();
    for (j, moon) in moons.iter().enumerate() {
        println!("Step 0 Moon {j} {moon:?}");
    }
    for i in 0..2772 {
        physics_step(&mut moons);
        for (j, moon) in moons.iter().enumerate() {
            println!(
                "Step {i} Moon {j} {moon:?} Potential Energy: {} Kinetic Energy: {}",
                moon.position.energy(),
                moon.velocity.energy(),
                i = i + 1
            );
        }
        println!(
            "Total energy: {}",
            moons.iter().map(|m| m.energy()).sum::<isize>()
        );
    }
    // moons.iter().map(|m| m.energy()).sum()
    todo!()
}
