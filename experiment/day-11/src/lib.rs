mod parser;
pub use parser::*;

use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Eq)]
pub struct Monke {
    pub items: Vec<usize>,
    pub index: usize,
    pub test_divisor: usize,
    pub operation: Operation,
    pub throw_to_monkey_if_test: usize,
    pub throw_to_monkey_if_not_test: usize,
}

impl Hash for Monke {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.index);
        state.finish();
    }
}

impl PartialEq for Monke {
    fn eq(&self, other: &Monke) -> bool {
        self.index.eq(&other.index)
    }
}

#[derive(Debug, Clone)]
pub struct MonkeBusiness {
    pub monkees: Vec<Monke>,
    pub inspection_count: HashMap<MonkeIdx, usize>,
    pub is_part_two: bool,
    pub divisor_product: usize,
}

pub type WorryLevel = usize;
pub type MonkeIdx = usize;

impl MonkeBusiness {
    pub fn play_monke(&self, monkee_idx: usize) -> Vec<(MonkeIdx, WorryLevel)> {
        let current_monke = self.monkees.get(monkee_idx).unwrap();
        let mut items = vec![];

        if !self.is_part_two {
            for item_index in 0..current_monke.items.len() {
                let mut bored_worry_level = current_monke
                    .operation
                    .apply(current_monke.items[item_index]);
                bored_worry_level /= 3;

                let remainder = bored_worry_level % current_monke.test_divisor;
                if remainder == 0 {
                    items.push((current_monke.throw_to_monkey_if_test, bored_worry_level));
                } else {
                    items.push((current_monke.throw_to_monkey_if_not_test, bored_worry_level));
                }
            }
        } else {
            for item_index in 0..current_monke.items.len() {
                let bored_worry_level = current_monke
                    .operation
                    .apply(current_monke.items[item_index]);
                let remainder = bored_worry_level % current_monke.test_divisor;

                // Since we don't divide by 3 anymore the actual values that we get if we
                // keep composing addition and multiplication operations repeatedly will
                // get ridiculously large (over a hundred digits within a few dozen iterations).

                // The decisions that the monkeys make for every inspection depends only on the individual
                // test divisor they store. If we could devise a way to store manageable worry levels
                // which don't impact the decisions that the monkeys would make, that'd be ideal.

                // Consider reducing all values by just one of the monkey's divisor. If we only look at the decisions
                // that monkey makes, it remains the same regardless of data because it only ever
                // has to the divisibility by its divisor. The data it produces after applying its operation to the "reduced" value
                // is modulo-equivalent to the data it would've generated after applying its operation to the actual value.
                // So the decision wouldn't change.

                // However, that could screw things up for the other monkeys because
                // that reduced number may end up leading the other monkeys to a different decision
                // because we have no idea if that number would've been "modulo"-equivalent to the actual larger number
                // for the other monkeys.

                // If instead, we choose a modulo-equivalence that works for every monkey, (i.e. the least common multiple of their divisors),
                // that would guarantee that all monkeys make the same decision but ultimately get to store significantly
                // smaller worry values.

                // In our scenario, the test divisors are actually primes so their least common multiple
                // is simply their product, which we store in `self.divisor_product`.

                if remainder == 0 {
                    items.push((
                        current_monke.throw_to_monkey_if_test,
                        bored_worry_level % self.divisor_product,
                    ));
                } else {
                    items.push((
                        current_monke.throw_to_monkey_if_not_test,
                        bored_worry_level % self.divisor_product,
                    ));
                }
            }
        }

        items
    }

    pub fn play_round(&mut self) {
        for current_monke_index in 0..self.monkees.len() {
            let items_to_throw = self.play_monke(current_monke_index);
            let current_monke = self.monkees.get(current_monke_index).unwrap();

            self.inspection_count
                .entry(current_monke_index)
                .and_modify(|value| *value += current_monke.items.len())
                .or_insert(current_monke.items.len());

            self.monkees
                .get_mut(current_monke_index)
                .unwrap()
                .items
                .clear();
            for (monke_idx, worry_level) in items_to_throw {
                self.monkees
                    .get_mut(monke_idx)
                    .unwrap()
                    .items
                    .push(worry_level);
            }
        }
    }
}
