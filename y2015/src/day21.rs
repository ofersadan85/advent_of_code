use itertools::iproduct;
use std::iter::once;
use strum::{EnumIter, IntoEnumIterator};

pub trait GearStats {
    fn damage(&self) -> u32;
    fn armor(&self) -> u32;
    fn cost(&self) -> u32;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum RingPower {
    One,
    Two,
    Three,
}

impl From<&RingPower> for u32 {
    fn from(value: &RingPower) -> Self {
        match value {
            RingPower::One => 1,
            RingPower::Two => 2,
            RingPower::Three => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ring {
    DamageRing(RingPower),
    ArmorRing(RingPower),
}

impl GearStats for Ring {
    fn damage(&self) -> u32 {
        match self {
            Self::DamageRing(ring_power) => ring_power.into(),
            Self::ArmorRing(_) => 0,
        }
    }

    fn armor(&self) -> u32 {
        match self {
            Self::ArmorRing(ring_power) => ring_power.into(),
            Self::DamageRing(_) => 0,
        }
    }

    fn cost(&self) -> u32 {
        match self {
            Self::DamageRing(ring_power) => match ring_power {
                RingPower::One => 25,
                RingPower::Two => 50,
                RingPower::Three => 100,
            },
            Self::ArmorRing(ring_power) => match ring_power {
                RingPower::One => 20,
                RingPower::Two => 40,
                RingPower::Three => 80,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Weapon {
    Dagger,
    ShortSword,
    WarHammer,
    LongSword,
    GreatAxe,
}

impl GearStats for Weapon {
    fn damage(&self) -> u32 {
        match self {
            Self::Dagger => 4,
            Self::ShortSword => 5,
            Self::WarHammer => 6,
            Self::LongSword => 7,
            Self::GreatAxe => 8,
        }
    }

    fn armor(&self) -> u32 {
        0
    }

    fn cost(&self) -> u32 {
        match self {
            Self::Dagger => 8,
            Self::ShortSword => 10,
            Self::WarHammer => 25,
            Self::LongSword => 40,
            Self::GreatAxe => 74,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Armor {
    Leather,
    ChainMail,
    SplintMail,
    BandedMail,
    PlateMail,
}

impl GearStats for Armor {
    fn damage(&self) -> u32 {
        0
    }

    fn armor(&self) -> u32 {
        match self {
            Self::Leather => 1,
            Self::ChainMail => 2,
            Self::SplintMail => 3,
            Self::BandedMail => 4,
            Self::PlateMail => 5,
        }
    }

    fn cost(&self) -> u32 {
        match self {
            Self::Leather => 13,
            Self::ChainMail => 31,
            Self::SplintMail => 53,
            Self::BandedMail => 75,
            Self::PlateMail => 102,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Gear {
    pub weapon: Weapon,
    pub armor: Option<Armor>,
    pub ring1: Option<Ring>,
    pub ring2: Option<Ring>,
}

impl GearStats for Gear {
    fn damage(&self) -> u32 {
        self.weapon.damage()
            + self.armor.map_or(0, |r| r.damage())
            + self.ring1.map_or(0, |r| r.damage())
            + self.ring2.map_or(0, |r| r.damage())
    }

    fn armor(&self) -> u32 {
        self.weapon.armor()
            + self.armor.map_or(0, |a| a.armor())
            + self.ring1.map_or(0, |r| r.armor())
            + self.ring2.map_or(0, |r| r.armor())
    }

    fn cost(&self) -> u32 {
        self.weapon.cost()
            + self.armor.map_or(0, |a| a.cost())
            + self.ring1.map_or(0, |r| r.cost())
            + self.ring2.map_or(0, |r| r.cost())
    }
}

impl Gear {
    pub fn combinations() -> Vec<Self> {
        iproduct!(
            Weapon::iter(),
            Armor::iter().map(Some).chain(once(None)),
            RingPower::iter()
                .map(|r| Some(Ring::DamageRing(r)))
                .chain(RingPower::iter().map(|r| Some(Ring::ArmorRing(r))))
                .chain(once(None)),
            RingPower::iter()
                .map(|r| Some(Ring::DamageRing(r)))
                .chain(RingPower::iter().map(|r| Some(Ring::ArmorRing(r))))
                .chain(once(None)),
        )
        .filter_map(|(w, a, r1, r2)| {
            if r1 == r2 && r1.is_some() {
                None
            } else {
                Some(Self {
                    weapon: w,
                    armor: a,
                    ring1: r1,
                    ring2: r2,
                })
            }
        })
        .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Player {
    hp: u32,
    gear: Gear,
}

impl GearStats for Player {
    fn damage(&self) -> u32 {
        self.gear.damage()
    }

    fn armor(&self) -> u32 {
        self.gear.armor()
    }

    fn cost(&self) -> u32 {
        self.gear.cost()
    }
}

#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl GearStats for Boss {
    fn damage(&self) -> u32 {
        self.damage
    }

    fn armor(&self) -> u32 {
        self.armor
    }

    fn cost(&self) -> u32 {
        0
    }
}

impl std::str::FromStr for Boss {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use anyhow::anyhow;
        let mut lines = s.lines();
        Ok(Self {
            hp: lines
                .next()
                .ok_or_else(|| anyhow!("hp line"))?
                .split_whitespace()
                .last()
                .ok_or_else(|| anyhow!("hp split"))?
                .parse()?,
            damage: lines
                .next()
                .ok_or_else(|| anyhow!("damage line"))?
                .split_whitespace()
                .last()
                .ok_or_else(|| anyhow!("damage split"))?
                .parse()?,
            armor: lines
                .next()
                .ok_or_else(|| anyhow!("armor line"))?
                .split_whitespace()
                .last()
                .ok_or_else(|| anyhow!("armor split"))?
                .parse()?,
        })
    }
}

#[derive(Debug)]
struct Game {
    user: Player,
    boss: Boss,
}

impl Game {
    const fn new(boss: Boss, user_gear: Gear) -> Self {
        Self {
            user: Player {
                hp: 100,
                gear: user_gear,
            },
            boss,
        }
    }

    fn step(&mut self) -> bool {
        let attack = self.user.damage().saturating_sub(self.boss.armor()).max(1);
        self.boss.hp = self.boss.hp.saturating_sub(attack);
        if self.boss.hp == 0 {
            return true;
        }
        let attack = self.boss.damage().saturating_sub(self.user.armor()).max(1);
        self.user.hp = self.user.hp.saturating_sub(attack);
        if self.user.hp == 0 {
            return true;
        }
        false
    }

    fn play(&mut self) {
        loop {
            if self.step() {
                break;
            }
        }
    }
}

fn cheapest_win(boss: &Boss) -> Option<u32> {
    Gear::combinations()
        .iter()
        .filter_map(|g| {
            let mut game = Game::new(*boss, *g);
            game.play();
            if game.boss.hp == 0 {
                Some(g.cost())
            } else {
                None
            }
        })
        .min()
}

fn expensive_loss(boss: &Boss) -> Option<u32> {
    Gear::combinations()
        .iter()
        .filter_map(|g| {
            let mut game = Game::new(*boss, *g);
            game.play();
            if game.user.hp == 0 {
                Some(g.cost())
            } else {
                None
            }
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE_USER: Player = Player {
        hp: 8,
        gear: Gear {
            weapon: Weapon::ShortSword,
            armor: Some(Armor::PlateMail),
            ring1: None,
            ring2: None,
        },
    };
    const EXAMPLE_BOSS: Boss = Boss {
        hp: 12,
        damage: 7,
        armor: 2,
    };

    #[test]
    fn test_game_steps() {
        let mut game = Game::new(EXAMPLE_BOSS, EXAMPLE_USER.gear);
        game.user.hp = EXAMPLE_USER.hp;
        eprintln!("{:?}", game.user);
        eprintln!("{:?}", game.user.gear.armor());
        eprintln!("{:?}", game.user.gear.armor.unwrap().armor());
        game.step();
        assert_eq!(game.user.hp, 6, "User after step 1");
        assert_eq!(game.boss.hp, 9, "Boss after step 1");
        game.step();
        assert_eq!(game.user.hp, 4, "User after step 2");
        assert_eq!(game.boss.hp, 6, "Boss after step 2");
        game.step();
        assert_eq!(game.user.hp, 2, "User after step 3");
        assert_eq!(game.boss.hp, 3, "Boss after step 3");
        let result = game.step();
        assert_eq!(game.user.hp, 2, "User final");
        assert_eq!(game.boss.hp, 0, "Boss final");
        assert!(result);
    }

    #[test]
    fn test_game_play() {
        let mut game = Game::new(EXAMPLE_BOSS.clone(), EXAMPLE_USER.gear.clone());
        game.user.hp = EXAMPLE_USER.hp;
        game.play();
        assert_eq!(game.user.hp, 2);
        assert_eq!(game.boss.hp, 0);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day21.txt").unwrap();
        let boss: Boss = input.parse().unwrap();
        assert_eq!(cheapest_win(&boss), Some(111));
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day21.txt").unwrap();
        let boss: Boss = input.parse().unwrap();
        assert_eq!(expensive_loss(&boss), Some(188));
    }
}
