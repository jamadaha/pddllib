use crate::task::{action::Action, Goal, Task};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fact {
    internal: u64,
}

impl Fact {
    pub fn new(predicate: usize, args: Vec<usize>) -> Self {
        debug_assert!(args.len() <= 3);
        let internal = predicate as u64
            + args
                .into_iter()
                .enumerate()
                .map(|(i, p)| (p as u64) << 16 * (i + 1))
                .sum::<u64>();
        Self { internal }
    }
    pub fn predicate(&self) -> usize {
        (self.internal as u16) as usize
    }

    pub fn args(&self) -> Vec<usize> {
        let mut parameters: Vec<usize> = Vec::new();
        let mut index = self.internal;
        index = index >> 16;
        while index != 0 {
            parameters.push((index as u16) as usize);
            index = index >> 16;
        }
        parameters
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    facts: BTreeSet<Fact>,
}

impl State {
    pub fn new(facts: Vec<Fact>) -> Self {
        State {
            facts: facts.into_iter().collect(),
        }
    }
    #[inline(always)]
    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }
    #[inline(always)]
    pub fn has_nullary(&self, task: &Task, predicate: usize) -> bool {
        self.has_fact(task, &Fact::new(predicate, vec![]))
    }
    #[inline(always)]
    pub fn has_unary(
        &self,
        task: &Task,
        predicate: usize,
        arg: &usize,
    ) -> bool {
        self.has_fact(task, &Fact::new(predicate, vec![*arg]))
    }
    #[inline(always)]
    pub fn has_nary(
        &self,
        task: &Task,
        predicate: usize,
        args: &Vec<usize>,
    ) -> bool {
        self.has_fact(task, &Fact::new(predicate, args.to_owned()))
    }
    #[inline(always)]
    pub fn has_fact(&self, task: &Task, fact: &Fact) -> bool {
        task.static_facts.contains(fact) || self.facts.contains(fact)
    }
    pub fn apply(&self, action: &Action, args: &Vec<usize>) -> Self {
        let mut state = self.clone();
        for atom in action.effect.iter() {
            let args = atom.map_args(args);
            let fact = Fact::new(atom.predicate, args);
            match atom.value {
                true => {
                    state.facts.insert(fact);
                }
                false => {
                    state.facts.remove(&fact);
                }
            }
        }
        state
    }
    pub fn covers(&self, task: &Task, goal: &Goal) -> bool {
        goal.iter().all(|(f, v)| self.has_fact(task, f) == *v)
    }
}
