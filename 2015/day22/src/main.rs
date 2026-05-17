use std::{
    collections::BinaryHeap,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use enum_map::{EnumMap, enum_map};
use strum::IntoEnumIterator;
use thiserror::Error;

const PLAYER_HEALTH: i64 = 50;
const PLAYER_MANA: i64 = 500;

fn main() {
    let input_fname = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("no input provided");
            return;
        }
    };

    let boss = match read_input(input_fname) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read input: {}", e);
            return;
        }
    };

    let player = Player {
        health: PLAYER_HEALTH,
        mana: PLAYER_MANA,
    };

    let part1 = match find_min_mana(player, boss, false) {
        Some(v) => v,
        None => {
            eprintln!("no winning options found");
            return;
        }
    };
    println!("Part 1: {}", part1);

    let part2 = match find_min_mana(player, boss, true) {
        Some(v) => v,
        None => {
            eprintln!("no winning options found");
            return;
        }
    };
    println!("Part 2: {}", part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    pub health: i64,
    pub mana: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Boss {
    pub health: i64,
    pub attack: i64,
}

fn read_input(path: impl AsRef<Path>) -> Result<Boss, ReadInputError> {
    let file = BufReader::new(File::open(path).map_err(ReadInputError::FileOpen)?);

    let mut health = None;
    let mut attack = None;
    for line in file.lines() {
        let line = line.map_err(ReadInputError::ReadLine)?;

        let (key, value) = line
            .split_once(':')
            .ok_or_else(|| ReadInputError::BadLine(line.clone()))?;

        match key.trim() {
            "Hit Points" => {
                health = Some(match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ReadInputError::BadLine(line.clone())),
                })
            }
            "Damage" => {
                attack = Some(match value.trim().parse() {
                    Ok(v) => v,
                    Err(_) => return Err(ReadInputError::BadLine(line.clone())),
                })
            }
            _ => {
                return Err(ReadInputError::BadLine(line.clone()));
            }
        }
    }

    let health = match health {
        Some(v) => v,
        None => return Err(ReadInputError::NoHealth),
    };

    let attack = match attack {
        Some(v) => v,
        None => return Err(ReadInputError::NoAttack),
    };

    Ok(Boss { health, attack })
}

#[derive(Debug, Error)]
enum ReadInputError {
    #[error("failed to open file: {0}")]
    FileOpen(io::Error),
    #[error("failed to read line: {0}")]
    ReadLine(io::Error),
    #[error("bad line: {0}")]
    BadLine(String),
    #[error("missing health")]
    NoHealth,
    #[error("missing attack")]
    NoAttack,
}

#[derive(Debug, Clone, Copy, strum::EnumIter, enum_map::Enum)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const COSTS: EnumMap<Spell, i64> = EnumMap::from_array([53, 73, 113, 173, 229]);
}

#[derive(Debug, Clone, Copy, enum_map::Enum)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pub player: Player,
    pub boss: Boss,
    pub effect_timers: EnumMap<Effect, i64>,
}

impl State {
    fn new(player: Player, boss: Boss) -> Self {
        Self {
            player,
            boss,
            effect_timers: enum_map! {
                Effect::Shield => 0,
                Effect::Poison => 0,
                Effect::Recharge => 0,
            },
        }
    }

    fn apply_effects(&mut self) {
        let mut new_timers = self.effect_timers;

        for (effect, timer) in self.effect_timers {
            if timer > 0 {
                match effect {
                    Effect::Shield => {}
                    Effect::Poison => self.boss.health -= 3,
                    Effect::Recharge => self.player.mana += 101,
                }

                new_timers[effect] -= 1;
            }
        }

        self.effect_timers = new_timers;
    }

    fn player_turn(
        &mut self,
        spell: Spell,
        hard_mode: bool,
    ) -> Result<Option<Outcome>, PlayerTurnError> {
        if hard_mode {
            self.player.health -= 1;
            if let Some(outcome) = self.get_outcome() {
                return Ok(Some(outcome));
            }
        }

        self.apply_effects();

        if let Some(outcome) = self.get_outcome() {
            return Ok(Some(outcome));
        }

        match spell {
            Spell::MagicMissile => {
                let cost = Spell::COSTS[Spell::MagicMissile];
                if cost > self.player.mana {
                    return Err(PlayerTurnError::NotEnoughMana);
                }
                self.player.mana -= cost;
                self.boss.health -= 4;
            }
            Spell::Drain => {
                let cost = Spell::COSTS[Spell::Drain];
                if cost > self.player.mana {
                    return Err(PlayerTurnError::NotEnoughMana);
                }
                self.player.mana -= cost;
                self.boss.health -= 2;
                self.player.health += 2;
            }
            Spell::Shield => {
                if self.effect_timers[Effect::Shield] != 0 {
                    return Err(PlayerTurnError::DoubleCast);
                }
                let cost = Spell::COSTS[Spell::Shield];
                if cost > self.player.mana {
                    return Err(PlayerTurnError::NotEnoughMana);
                }
                self.player.mana -= cost;
                self.effect_timers[Effect::Shield] = 6;
            }
            Spell::Poison => {
                if self.effect_timers[Effect::Poison] != 0 {
                    return Err(PlayerTurnError::DoubleCast);
                }
                let cost = Spell::COSTS[Spell::Poison];
                if cost > self.player.mana {
                    return Err(PlayerTurnError::NotEnoughMana);
                }
                self.player.mana -= cost;
                self.effect_timers[Effect::Poison] = 6
            }
            Spell::Recharge => {
                if self.effect_timers[Effect::Recharge] != 0 {
                    return Err(PlayerTurnError::DoubleCast);
                }
                let cost = Spell::COSTS[Spell::Recharge];
                if cost > self.player.mana {
                    return Err(PlayerTurnError::NotEnoughMana);
                }
                self.player.mana -= cost;
                self.effect_timers[Effect::Recharge] = 5;
            }
        }

        Ok(self.get_outcome())
    }

    fn boss_turn(&mut self) -> Option<Outcome> {
        self.apply_effects();

        if let Some(outcome) = self.get_outcome() {
            return Some(outcome);
        }

        let armor = if self.effect_timers[Effect::Shield] > 0 {
            7
        } else {
            0
        };
        self.player.health -= (self.boss.attack - armor).max(1);

        self.get_outcome()
    }

    fn get_outcome(&self) -> Option<Outcome> {
        if self.boss.health <= 0 {
            Some(Outcome::Player)
        } else if self.player.health <= 0 {
            Some(Outcome::Boss)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Player,
    Boss,
}

#[derive(Debug, Error)]
enum PlayerTurnError {
    #[error("spell already in effect")]
    DoubleCast,
    #[error("not enough mana")]
    NotEnoughMana,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StateNode {
    pub state: State,
    pub cost: i64,
}

impl StateNode {
    fn player_turn(
        &mut self,
        spell: Spell,
        hard_mode: bool,
    ) -> Result<Option<Outcome>, PlayerTurnError> {
        let r = self.state.player_turn(spell, hard_mode);

        if r.is_ok() {
            self.cost += Spell::COSTS[spell];
        }

        r
    }

    fn boss_turn(&mut self) -> Option<Outcome> {
        self.state.boss_turn()
    }
}

impl Ord for StateNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.state.cmp(&self.state))
    }
}

impl PartialOrd for StateNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_mana(player: Player, boss: Boss, hard_mode: bool) -> Option<i64> {
    let mut queue = BinaryHeap::new();
    queue.push(StateNode {
        state: State::new(player, boss),
        cost: 0,
    });

    while let Some(state) = queue.pop() {
        for spell in Spell::iter() {
            let mut next = state;
            match next.player_turn(spell, hard_mode) {
                Ok(v) => match v {
                    Some(Outcome::Player) => return Some(next.cost),
                    Some(Outcome::Boss) => continue,
                    None => {}
                },
                Err(_) => {
                    continue;
                }
            }
            match next.boss_turn() {
                Some(Outcome::Player) => return Some(next.cost),
                Some(Outcome::Boss) => continue,
                None => {}
            }

            queue.push(next);
        }
    }

    None
}
