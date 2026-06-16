use std::{
    collections::{HashSet, VecDeque},
    iter,
};

use anyhow::{Result, bail};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Microchip {
    pub element: u32,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generator {
    pub element: u32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Item {
    Microchip(Microchip),
    Generator(Generator),
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Floor {
    microchips: Vec<Microchip>,
    generators: Vec<Generator>,
}

impl Floor {
    pub fn new(items: impl IntoIterator<Item = Item>) -> Result<Self> {
        let (mut microchips, mut generators) = items.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut microchips, mut generators), item| {
                match item {
                    Item::Microchip(microchip) => microchips.push(microchip),
                    Item::Generator(generator) => generators.push(generator),
                };
                (microchips, generators)
            },
        );

        // guarantee identical hashing for identical floors
        microchips.sort();
        generators.sort();

        if microchips.iter().any(|chip| {
            !generators.is_empty()
                && generators
                    .iter()
                    .all(|generator| generator.element != chip.element)
        }) {
            bail!("invalid floor setup");
        }

        Ok(Self {
            microchips,
            generators,
        })
    }

    fn is_empty(&self) -> bool {
        self.microchips.is_empty() && self.generators.is_empty()
    }

    pub fn items(&self) -> Vec<Item> {
        self.microchips
            .iter()
            .map(|chip| Item::Microchip(*chip))
            .chain(
                self.generators
                    .iter()
                    .map(|generator| Item::Generator(*generator)),
            )
            .collect()
    }
}

pub fn get_min_steps(floors: &[Floor; 4]) -> Option<usize> {
    let is_solution =
        |floors: &[Floor; 4]| floors[0].is_empty() && floors[1].is_empty() && floors[2].is_empty();

    if is_solution(floors) {
        return Some(0);
    }

    let mut queue = VecDeque::from([(0, 0usize, floors.clone())]);
    let mut visited = HashSet::from([(0, floors.clone())]);

    while let Some((n, current_idx, state)) = queue.pop_front() {
        for next_idx in iter::empty()
            .chain((current_idx > 0).then(|| current_idx - 1))
            .chain((current_idx + 1 < floors.len()).then(|| current_idx + 1))
        {
            let current_items = state[current_idx].items();
            let next_items = state[next_idx].items();

            for i in 0..current_items.len() {
                let mut current_items = current_items.clone();
                let mut next_items = next_items.clone();
                let item = current_items.remove(i);
                next_items.push(item);
                let current_floor = match Floor::new(current_items) {
                    Ok(v) => v,
                    Err(_) => {
                        continue;
                    }
                };
                let next_floor = match Floor::new(next_items) {
                    Ok(v) => v,
                    Err(_) => {
                        continue;
                    }
                };
                let mut next_state = state.clone();
                next_state[current_idx] = current_floor;
                next_state[next_idx] = next_floor;
                if !visited.insert((next_idx, next_state.clone())) {
                    continue;
                }
                if is_solution(&next_state) {
                    return Some(n + 1);
                }
                queue.push_back((n + 1, next_idx, next_state));
            }

            for i in 0..(current_items.len() - 1) {
                for j in (i + 1)..current_items.len() {
                    let mut current_items = current_items.clone();
                    let mut next_items = next_items.clone();
                    let item = current_items.remove(i.max(j));
                    next_items.push(item);
                    let item = current_items.remove(i.min(j));
                    next_items.push(item);
                    let current_floor = match Floor::new(current_items) {
                        Ok(v) => v,
                        Err(_) => {
                            continue;
                        }
                    };
                    let next_floor = match Floor::new(next_items) {
                        Ok(v) => v,
                        Err(_) => {
                            continue;
                        }
                    };
                    let mut next_state = state.clone();
                    next_state[current_idx] = current_floor;
                    next_state[next_idx] = next_floor;
                    if !visited.insert((next_idx, next_state.clone())) {
                        continue;
                    }
                    if is_solution(&next_state) {
                        return Some(n + 1);
                    }
                    queue.push_back((n + 1, next_idx, next_state));
                }
            }
        }
    }

    None
}
