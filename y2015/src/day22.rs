#[cfg(test)]
mod tests {
    use crate::day21::*;
    // use std::fs::read_to_string;
    use strum::IntoEnumIterator;
    const EXAMPLE_SPELL_BOSS: Boss = Boss {
        hp: 13,
        damage: 8,
        armor: 0,
    };

    #[test]
    fn test_spell_set() {
        let mut set = ActiveEffects::from_iter(Spell::iter().map(Some));
        for i in 0..100 {
            set.insert(None);
            set.insert(Some(Spell::Drain));
            set.insert(Some(Spell::MagicMissile));
            set.insert(Some(Spell::Poison(i)));
            set.insert(Some(Spell::Recharge(i)));
            set.insert(Some(Spell::Shield(i)));
        }
        assert_eq!(set.len(), Spell::iter().len() + 1);
    }

    #[test]
    fn test_cast_active_spell() {
        let mut player: Player = Player::new(10);
        player.mana = 1000;
        let mut boss = EXAMPLE_SPELL_BOSS;
        boss.hp = 100;
        for spell in Spell::iter() {
            let spell = Some(spell.with_default_timer());
            let mut game = Game::new(boss.clone(), player.clone(), 0);
            game.player_turn(spell);
            game.boss_turn();
            game.player_turn(spell);
            if spell.is_immediate() {
                assert_eq!(
                    game.state,
                    GameState::Active,
                    "{spell:?} Is immediate and should not end game"
                );
            } else {
                assert_eq!(
                    game.state,
                    GameState::BossWon,
                    "{spell:?} Cannot be cast while still active"
                );
            }
        }
    }

    fn find_active_spell<'a>(game: &'a Game, spell: &Spell) -> Option<Spell> {
        game.player
            .active_effects
            .iter()
            .flatten()
            .find(|s| s == &spell)
            .copied()
    }

    fn player_turn_test_helper(game: &mut Game, spell: Spell) {
        let before = game.clone();
        let before_spell = find_active_spell(&before, &spell);
        game.player_turn(Some(spell));
        if before.player.mana < spell.mana_cost() {
            assert_eq!(game.state, GameState::BossWon);
        } else {
            assert_ne!(game.state, GameState::BossWon);
        }
        assert_eq!(
            game.player.mana,
            before.player.mana + before.player.active_effects.mana_gain() - spell.mana_cost()
        );
        let after_spell = find_active_spell(&game, &spell);
        eprintln!("BEFORE SPELLS {:?}", before.player.active_effects);
        eprintln!("AFTER SPELLS {:?}", game.player.active_effects);
        match (before_spell, after_spell) {
            (None, None) => assert_eq!(
                game.player.active_effects.len(),
                before.player.active_effects.len()
            ),
            (None, Some(_)) => {} // TODO: Add test that all timers have decreased or dropped
            (Some(Spell::Drain), Some(Spell::Drain))
            | (Some(Spell::MagicMissile), Some(Spell::MagicMissile)) => {
                unreachable!("Immediate spell should not be added to active effects {spell:?}")
            }
            (Some(_before_spell), Some(_after_spell)) => {} // TODO: Add test that all timers have decreased or dropped
            (Some(_), None) => {
                unreachable!("There's always a spell *after* because we just inserted it")
            }
        }
    }

    #[test]
    fn test_spell_game_1() {
        let mut player = Player::new(10);
        player.mana = 250;
        let mut game = Game::new(EXAMPLE_SPELL_BOSS, player, 0);
        // First round
        let spell = Spell::Poison(6);
        player_turn_test_helper(&mut game, spell);
        assert_eq!(game.player.active_effects.len(), 1);
        assert_eq!(
            game.boss, EXAMPLE_SPELL_BOSS,
            "Poison should have no effect in cast turn"
        );
        assert_eq!(game.player.mana, 77);
        assert_eq!(game.state, GameState::Active);
        game.boss_turn();
        assert_eq!(game.player.hp, 2);
        assert_eq!(game.boss.hp, 10);
        assert_eq!(game.state, GameState::Active);
        // Second round
        let spell = Spell::MagicMissile;
        player_turn_test_helper(&mut game, spell);
        assert_eq!(game.player.active_effects.len(), 1); // Immediate effects do not get added
        assert_eq!(game.player.mana, 24);
        assert_eq!(game.boss.hp, 3);
        assert_eq!(game.state, GameState::Active);
        game.boss_turn();
        assert_eq!(game.state, GameState::PlayerWon);
    }

    #[test]
    fn test_spell_game_2() {
        let mut player: Player = Player::new(10);
        player.mana = 250;
        let mut boss = EXAMPLE_SPELL_BOSS;
        boss.hp = 14;
        let mut game = Game::new(boss, player, 0);
        // First round
        let spell = Spell::Recharge(5);
        player_turn_test_helper(&mut game, spell);
        assert_eq!(game.player.active_effects.len(), 1);
        assert_eq!(game.boss, boss, "Recharge should have no effect on boss");
        assert_eq!(game.player.mana, 21);
        assert_eq!(game.state, GameState::Active);
        game.boss_turn();
        assert_eq!(game.player.mana, 122);
        assert_eq!(game.player.hp, 2);
        assert_eq!(game.state, GameState::Active);
        assert_eq!(spell.timer(), find_active_spell(&game, &spell).timer() + 1);

        // Second round
        let spell = Spell::Shield(6);
        player_turn_test_helper(&mut game, spell);
        assert_eq!(game.player.active_effects.len(), 2);
        assert_eq!(
            game.player.mana,
            122 /* previous */ + 101 /* recharge */ - 113 /* shield */
        );
        assert_eq!(game.player.armor(), 7);
        assert_eq!(game.state, GameState::Active);
        game.boss_turn();
        assert_eq!(game.player.hp, 1);
        assert_eq!(game.player.mana, 211);
        assert_eq!(spell.timer(), find_active_spell(&game, &spell).timer() + 1);
        assert_eq!(game.state, GameState::Active);

        // Third round
        let spell = Spell::Drain;
        player_turn_test_helper(&mut game, spell);
        assert_eq!(
            game.player.active_effects.len(),
            2,
            "Immediate effects do not get added ({spell:?})"
        );
        assert_eq!(
            game.player.mana,
            211 /* previous */ + 101 /* recharge */ - 73 /* drain */
        );
        assert_eq!(game.boss.hp, 12);
        assert_eq!(game.player.hp, 3);
        assert!(
            find_active_spell(&game, &spell).is_none(),
            "Immediate effects do not get added ({spell:?})"
        );
        assert_eq!(
            find_active_spell(&game, &Spell::Recharge(1)).timer(),
            1,
            "Recharge still in effect, about to wear off"
        );
        assert_eq!(game.state, GameState::Active);
        game.boss_turn();
        assert_eq!(game.player.mana, 340);
        assert_eq!(game.player.hp, 2);
        assert!(
            find_active_spell(&game, &Spell::Recharge(0)).is_none(),
            "Recharge wore off"
        );
        assert_eq!(game.state, GameState::Active);

        // Fourth round
        let spell = Spell::Poison(6);
        player_turn_test_helper(&mut game, spell);
        assert_eq!(
            game.player.active_effects.len(),
            2,
            "One wore off, but one added"
        );
        assert_eq!(game.player.mana, 340 /* previous */ - 173 /* poison */);
        assert_eq!(game.boss.hp, 12);
        assert_eq!(game.player.hp, 2);
        assert_eq!(game.state, GameState::Active);
        game.boss_turn();
        assert_eq!(game.boss.hp, 9);
        assert_eq!(game.player.hp, 1);
        assert_eq!(spell.timer(), find_active_spell(&game, &spell).timer() + 1);
        assert_eq!(
            find_active_spell(&game, &Spell::Shield(1)).timer(),
            1,
            "Shield still in effect, about to wear off"
        );
        assert_eq!(game.state, GameState::Active);

        // Final round
        let spell = Spell::MagicMissile;
        player_turn_test_helper(&mut game, spell);
        assert_eq!(
            game.player.active_effects.len(),
            2,
            "Immediate effects do not get added ({spell:?})"
        );
        assert_eq!(game.player.mana, 167 /* previous */ - 53 /* missile */);
        assert_eq!(game.boss.hp, 2);
        assert_eq!(game.state, GameState::Active);
        game.boss_turn();
        assert!(
            find_active_spell(&game, &Spell::Recharge(0)).is_none(),
            "Shield wore off"
        );
        assert_eq!(game.boss.hp, 0);
        assert_eq!(game.player.hp, 1);
        assert_eq!(game.state, GameState::PlayerWon);
    }
}
