use itertools::iproduct;
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    iter::{self, once},
    mem,
    str::FromStr,
};
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

impl<T: GearStats> GearStats for Option<T> {
    fn damage(&self) -> u32 {
        self.as_ref().map_or(0, GearStats::damage)
    }

    fn armor(&self) -> u32 {
        self.as_ref().map_or(0, GearStats::armor)
    }

    fn cost(&self) -> u32 {
        self.as_ref().map_or(0, GearStats::cost)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Gear {
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    pub ring1: Option<Ring>,
    pub ring2: Option<Ring>,
}

impl GearStats for Gear {
    fn damage(&self) -> u32 {
        self.weapon.damage() + self.armor.damage() + self.ring1.damage() + self.ring2.damage()
    }

    fn armor(&self) -> u32 {
        self.weapon.armor() + self.armor.armor() + self.ring1.armor() + self.ring2.armor()
    }

    fn cost(&self) -> u32 {
        self.weapon.cost() + self.armor.cost() + self.ring1.cost() + self.ring2.cost()
    }
}

impl Gear {
    pub fn weapon_combinations() -> Vec<Self> {
        iproduct!(
            Weapon::iter().map(Some),
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

/// Spell variants that the player can use
///
/// Spells without an inner type are single-use (no lasting effect), and spells with an inner (number) type
/// are lasting effects, with the number representing the remaining number of turns.
#[derive(Debug, Clone, Copy, EnumIter, Eq)]
pub enum Spell {
    /// Costs 53 mana. It instantly does 4 damage
    MagicMissile,
    /// Costs 73 mana. It instantly does 2 damage and heals you for 2 hit points
    Drain,
    /// Costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7
    Shield(u32),
    /// Costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage
    Poison(u32),
    /// Costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana
    Recharge(u32),
}

/// An implementation of [`PartialEq`] that only checks the enum variant without the timer
///
/// This will ensure that in a [`HashSet`] there can only be one of each,
/// but we still need to avoid inserting a new one with a different timer without removing the old one first
impl PartialEq for Spell {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

/// An implementation of [`Hash`] that only checks the enum variant without the timer
///
/// This will ensure that in a [`HashSet`] there can only be one of each,
/// but we still need to avoid inserting a new one with a different timer without removing the old one first
impl Hash for Spell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
    }
}

pub trait SpellEffect
where
    Self: Sized,
{
    fn heal(&self) -> u32;
    fn mana_cost(&self) -> u32;
    fn mana_gain(&self) -> u32;
    fn default_timer(&self) -> u32;
    fn timer(&self) -> u32;
    fn update_timer(&self) -> Option<Self>;
    fn is_immediate(&self) -> bool {
        false
    }
    /// Dummy default implementation that can be overridden
    #[must_use]
    fn with_timer(&self, _timer: u32) -> Option<Self> {
        unimplemented!()
    }
    /// Dummy default implementation that can be overridden
    #[must_use]
    fn with_default_timer(&self) -> Self {
        unimplemented!()
    }
}

impl SpellEffect for Spell {
    fn heal(&self) -> u32 {
        match self {
            Self::Drain => 2,
            _ => 0,
        }
    }

    fn mana_cost(&self) -> u32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield(_) => 113,
            Self::Poison(_) => 173,
            Self::Recharge(_) => 229,
        }
    }

    fn mana_gain(&self) -> u32 {
        match self {
            Self::Recharge(_) => 101,
            _ => 0,
        }
    }

    fn default_timer(&self) -> u32 {
        match self {
            Self::MagicMissile | Self::Drain => 0,
            Self::Shield(_) | Self::Poison(_) => 6,
            Self::Recharge(_) => 5,
        }
    }

    fn timer(&self) -> u32 {
        match self {
            Self::MagicMissile | Self::Drain => 0,
            Self::Shield(t) | Self::Poison(t) | Self::Recharge(t) => *t,
        }
    }

    fn update_timer(&self) -> Option<Self> {
        match self {
            Self::MagicMissile
            | Self::Drain
            | Self::Shield(1)
            | Self::Poison(1)
            | Self::Recharge(1) => None,
            Self::Shield(t) => Some(Self::Shield(t - 1)),
            Self::Poison(t) => Some(Self::Poison(t - 1)),
            Self::Recharge(t) => Some(Self::Recharge(t - 1)),
        }
    }

    fn is_immediate(&self) -> bool {
        match self {
            Self::MagicMissile | Self::Drain => true,
            Self::Shield(_) | Self::Poison(_) | Self::Recharge(_) => false,
        }
    }

    fn with_timer(&self, timer: u32) -> Option<Self> {
        match self {
            Self::MagicMissile | Self::Drain => None,
            Self::Shield(_) => Some(Self::Shield(timer)),
            Self::Poison(_) => Some(Self::Poison(timer)),
            Self::Recharge(_) => Some(Self::Recharge(timer)),
        }
    }

    fn with_default_timer(&self) -> Self {
        self.with_timer(self.default_timer()).unwrap_or(*self)
    }
}

impl GearStats for Spell {
    fn damage(&self) -> u32 {
        match self {
            Self::MagicMissile => 4,
            Self::Drain => 2,
            Self::Poison(_) => 3,
            Self::Shield(_) | Self::Recharge(_) => 0,
        }
    }

    fn armor(&self) -> u32 {
        match self {
            Self::Shield(_) => 7,
            _ => 0,
        }
    }

    fn cost(&self) -> u32 {
        0
    }
}

impl<T: SpellEffect> SpellEffect for Option<T> {
    fn heal(&self) -> u32 {
        self.as_ref().map_or(0, SpellEffect::heal)
    }

    fn mana_cost(&self) -> u32 {
        self.as_ref().map_or(0, SpellEffect::mana_cost)
    }

    fn mana_gain(&self) -> u32 {
        self.as_ref().map_or(0, SpellEffect::mana_gain)
    }

    fn default_timer(&self) -> u32 {
        self.as_ref().map_or(0, SpellEffect::default_timer)
    }

    fn timer(&self) -> u32 {
        self.as_ref().map_or(0, SpellEffect::timer)
    }

    fn update_timer(&self) -> Option<Self> {
        self.as_ref().map(SpellEffect::update_timer)
    }

    fn is_immediate(&self) -> bool {
        self.as_ref().map_or(false, SpellEffect::is_immediate)
    }
}

pub type ActiveEffects = HashSet<Option<Spell>>;

impl SpellEffect for ActiveEffects {
    fn heal(&self) -> u32 {
        self.iter().map(SpellEffect::heal).sum()
    }

    fn mana_cost(&self) -> u32 {
        self.iter().map(SpellEffect::mana_cost).sum()
    }

    fn mana_gain(&self) -> u32 {
        self.iter().map(SpellEffect::mana_gain).sum()
    }

    fn default_timer(&self) -> u32 {
        0
    }

    fn timer(&self) -> u32 {
        0
    }

    /// This function always returns `Some` because we handle the `None` option on every spell
    fn update_timer(&self) -> Option<Self> {
        Some(self.iter().filter_map(SpellEffect::update_timer).collect())
    }
}

impl GearStats for ActiveEffects {
    fn damage(&self) -> u32 {
        self.iter().map(GearStats::damage).sum()
    }

    fn armor(&self) -> u32 {
        self.iter().map(GearStats::armor).sum()
    }

    fn cost(&self) -> u32 {
        self.iter().map(GearStats::cost).sum()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Player {
    pub hp: u32,
    pub gear: Gear,
    pub mana: u32,
    pub mana_spent: u32,
    pub active_effects: ActiveEffects,
}

impl Player {
    pub fn new(hp: u32) -> Self {
        Self {
            hp,
            ..Default::default()
        }
    }
}

impl GearStats for Player {
    fn damage(&self) -> u32 {
        self.gear.damage() + self.active_effects.damage()
    }

    fn armor(&self) -> u32 {
        self.gear.armor() + self.active_effects.armor()
    }

    fn cost(&self) -> u32 {
        self.gear.cost() + self.active_effects.cost()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boss {
    pub hp: u32,
    pub damage: u32,
    pub armor: u32,
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

impl FromStr for Boss {
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
                .unwrap_or("Armor: 0")
                .split_whitespace()
                .last()
                .ok_or_else(|| anyhow!("armor split"))?
                .parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Active,
    PlayerWon,
    BossWon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameDifficulty {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub player: Player,
    pub boss: Boss,
    pub state: GameState,
    pub damage_minimum: u32,
    pub difficulty: GameDifficulty,
}

impl Game {
    pub const fn new(
        boss: Boss,
        player: Player,
        damage_minimum: u32,
        difficulty: GameDifficulty,
    ) -> Self {
        Self {
            player,
            boss,
            state: GameState::Active,
            damage_minimum,
            difficulty,
        }
    }

    pub fn apply_spells(&mut self, cast_spell: Option<Spell>) {
        // eprintln!("Active spells: {:?}", self.player.active_effects);
        if cast_spell.is_some() && cast_spell.mana_cost() > self.player.mana {
            self.state = GameState::BossWon;
            return;
        }
        self.player.mana += self.player.active_effects.mana_gain();
        self.player.mana -= cast_spell.mana_cost();
        self.player.mana_spent += cast_spell.mana_cost();
        if cast_spell.is_immediate() {
            // eprintln!("Applying immediate spell {cast_spell:?}");
            self.boss.hp = self.boss.hp.saturating_sub(cast_spell.damage());
            self.player.hp += cast_spell.heal();
            if self.boss.hp == 0 {
                self.state = GameState::PlayerWon;
            }
        }
        if cast_spell.is_none() {
            // Only where `case_spell` is None, we're in the boss's turn
            // Otherwise, the player has already inflicted the damage in his attack
            // eprintln!(
            //     "Applying active effect damage: {}",
            //     self.player.active_effects.damage()
            // );
            self.boss.hp = self
                .boss
                .hp
                .saturating_sub(self.player.active_effects.damage());
        }
        let mut updated_effects = self
            .player
            .active_effects
            .update_timer()
            .expect("Active Spell Effects is always `Some`");
        mem::swap(&mut self.player.active_effects, &mut updated_effects);
        if let Some(spell) = cast_spell {
            if self.player.active_effects.contains(&Some(spell)) {
                self.state = GameState::BossWon;
                return;
            }
            if !cast_spell.is_immediate() {
                self.player.active_effects.insert(cast_spell);
            }
        }
    }

    pub fn player_turn(&mut self, cast_spell: Option<Spell>) {
        // eprintln!("Player casts: {:?}", cast_spell);
        // eprintln!("Boss before {:?}", self.boss);
        if self.difficulty == GameDifficulty::Hard {
            self.player.hp = self.player.hp.saturating_sub(1);
            if self.player.hp == 0 {
                self.state = GameState::BossWon;
                return;
            }
        }
        let attack = self
            .player
            .damage()
            .saturating_sub(self.boss.armor())
            .max(self.damage_minimum);
        self.boss.hp = self.boss.hp.saturating_sub(attack);
        self.apply_spells(cast_spell);
        if self.boss.hp == 0 {
            self.state = GameState::PlayerWon;
        }
        // eprintln!("Boss After {:?}", self.boss);
    }

    pub fn boss_turn(&mut self) {
        // eprintln!("BOSS TURN before {:?}", self.boss);
        // eprintln!("Player before {:?}", self.player);
        self.apply_spells(None);
        if self.boss.hp == 0 {
            // eprintln!("Boss killed by active effects");
            self.state = GameState::PlayerWon;
            return;
        }
        let attack = self
            .boss
            .damage()
            .saturating_sub(self.player.armor())
            .max(self.damage_minimum);
        self.player.hp = self.player.hp.saturating_sub(attack);
        if self.player.hp == 0 {
            self.state = GameState::BossWon;
        }
        // eprintln!("Player after {:?}", self.player);
    }

    pub fn step(&mut self, cast_spell: Option<Spell>) -> GameState {
        self.player_turn(cast_spell);
        if self.state == GameState::Active {
            self.boss_turn();
        }
        self.state
    }

    pub fn play<T>(&mut self, spells: T) -> GameState
    where
        T: IntoIterator<Item = Option<Spell>>,
    {
        // eprintln!("Game started");
        let mut spells = spells.into_iter();
        while self.state == GameState::Active {
            if let Some(cast_spell) = spells.next() {
                self.step(cast_spell);
            } else {
                break;
            }
        }
        self.state
    }
}

fn cheap_win(boss: &Boss, player: &Player) -> Option<u32> {
    Gear::weapon_combinations()
        .iter()
        .filter_map(|g| {
            let mut player = player.clone();
            player.gear = *g;
            let mut game = Game::new(*boss, player, 1, GameDifficulty::Easy);
            game.play(iter::once(None).cycle());
            if game.boss.hp == 0 {
                Some(g.cost())
            } else {
                None
            }
        })
        .min()
}

fn expensive_loss(boss: &Boss, player: &Player) -> Option<u32> {
    Gear::weapon_combinations()
        .iter()
        .filter_map(|g| {
            let mut player = player.clone();
            player.gear = *g;
            let mut game = Game::new(*boss, player, 1, GameDifficulty::Easy);
            game.play(iter::once(None).cycle());
            if game.player.hp == 0 {
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
    const EXAMPLE_BOSS: Boss = Boss {
        hp: 12,
        damage: 7,
        armor: 2,
    };

    #[test]
    fn test_game_step() {
        let mut player = Player::new(8);
        player.gear.weapon = Some(Weapon::ShortSword);
        player.gear.armor = Some(Armor::PlateMail);
        let mut game = Game::new(EXAMPLE_BOSS, player, 1, GameDifficulty::Easy);
        game.step(None);
        assert_eq!(game.player.hp, 6, "Player after step 1");
        assert_eq!(game.boss.hp, 9, "Boss after step 1");
        assert_eq!(game.state, GameState::Active);
        game.step(None);
        assert_eq!(game.player.hp, 4, "Player after step 2");
        assert_eq!(game.boss.hp, 6, "Boss after step 2");
        game.step(None);
        assert_eq!(game.player.hp, 2, "Player after step 3");
        assert_eq!(game.boss.hp, 3, "Boss after step 3");
        game.step(None);
        assert_eq!(game.player.hp, 2, "Player final");
        assert_eq!(game.boss.hp, 0, "Boss final");
        assert_eq!(game.state, GameState::PlayerWon);
    }

    #[test]
    fn test_game_play() {
        let mut player = Player::new(8);
        player.gear.weapon = Some(Weapon::ShortSword);
        player.gear.armor = Some(Armor::PlateMail);
        let mut game = Game::new(EXAMPLE_BOSS, player, 1, GameDifficulty::Easy);
        game.play(iter::once(None).cycle());
        assert_eq!(game.player.hp, 2, "Player final");
        assert_eq!(game.boss.hp, 0, "Boss final");
        assert_eq!(game.state, GameState::PlayerWon);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("../inputs/2015/day21.txt").unwrap();
        let boss: Boss = input.parse().unwrap();
        let player = Player::new(100);
        assert_eq!(cheap_win(&boss, &player), Some(111));
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("../inputs/2015/day21.txt").unwrap();
        let boss: Boss = input.parse().unwrap();
        let player = Player::new(100);
        assert_eq!(expensive_loss(&boss, &player), Some(188));
    }
}
