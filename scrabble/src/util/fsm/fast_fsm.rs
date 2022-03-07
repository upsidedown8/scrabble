use crate::{
    game::tile::Letter,
    util::fsm::{
        small_fsm::{SmallFsm, SmallStateId, Transition},
        Fsm, FsmBuilder, FsmSequence, StateId,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::{hash_map, HashMap};

/// A state in the [`FastFsm`]. Stores only the transitions to other states,
/// in a hashmap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub(super) transitions: HashMap<Letter, StateId>,
}

/// A time optimised finite state machine.
///
/// States are layed out in a single array, with the initial (and by definition
/// non-terminal) state at index zero, followed by all other non-terminal
/// states, then all terminal states. This way an `is_terminal` flag does not
/// need to be stored for each state.
///
/// This implementation of the [`Fsm`] trait is time optimised, as a hashmap for
/// transitions is fast to access, but may not be the most compact representation of
/// the transitions from each state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FastFsm {
    pub(super) states: Vec<State>,
    pub(super) terminal_count: usize,
}

impl FastFsm {
    /// Gets the number of states
    pub fn state_count(&self) -> usize {
        self.states.len()
    }
    /// Gets the number of transitions
    pub fn transition_count(&self) -> usize {
        self.states
            .iter()
            .map(|state| state.transitions.len())
            .sum()
    }
}
impl From<FsmBuilder> for FastFsm {
    fn from(mut builder: FsmBuilder) -> Self {
        // the initial state (at index 0) should be non-terminal
        assert!(
            !builder.states[&StateId(0)].is_terminal,
            "Initial state should be non-terminal"
        );

        let terminal_count = builder.states.values().filter(|&v| v.is_terminal).count();
        let non_terminal_count = builder.states.len() - terminal_count;

        // stores the mapping old id -> new id.
        let mut terminal_id = 0;
        let mut non_terminal_id = 1; // skip zero id (used for initial state)
        let mut state_id_map = HashMap::new();

        // ensure that the position of the initial state is unchanged
        // by removing it from the hashmap (hashmap traversal is not
        // in order).
        let initial_state_transitions = builder.states.remove(&StateId(0)).unwrap().transitions;
        state_id_map.insert(StateId(0), StateId(0));

        for (&old_state_id, old_state) in builder.states.iter() {
            let (offset, id) = match old_state.is_terminal {
                true => (non_terminal_count, &mut terminal_id),
                false => (0, &mut non_terminal_id),
            };

            state_id_map.insert(old_state_id, StateId(offset + *id));
            *id += 1;
        }

        let mut terminal_states = Vec::with_capacity(non_terminal_count);
        let mut non_terminal_states = Vec::with_capacity(builder.states.len());
        let map_transitions = |transitions: HashMap<_, _>| {
            transitions
                .into_iter()
                .map(|(k, v)| (k, state_id_map[&v]))
                .collect()
        };

        // add the initial state first
        non_terminal_states.push(State {
            transitions: map_transitions(initial_state_transitions),
        });

        // traverse the old states and add to the new states
        for (_, old_state) in builder.states {
            let state = State {
                transitions: map_transitions(old_state.transitions),
            };

            match old_state.is_terminal {
                true => terminal_states.push(state),
                false => non_terminal_states.push(state),
            }
        }

        let mut states = non_terminal_states;
        states.append(&mut terminal_states);

        Self {
            states,
            terminal_count,
        }
    }
}
impl From<SmallFsm> for FastFsm {
    fn from(small_fsm: SmallFsm) -> Self {
        let mut states = Vec::with_capacity(small_fsm.states.len());

        // add the states in the same order as the small fsm.
        for id in 0..small_fsm.states.len() {
            let (start, end) = small_fsm.transition_limits(StateId(id));
            let transitions = (start..end)
                .map(|pos| small_fsm.transitions[pos])
                .map(|Transition(letter, SmallStateId(state_id))| {
                    (letter, StateId(state_id as usize))
                })
                .collect::<HashMap<_, _>>();

            states.push(State { transitions });
        }

        Self {
            states,
            terminal_count: small_fsm.terminal_count,
        }
    }
}
impl<'a> Fsm<'a> for FastFsm {
    type TransitionsIter = FastFsmTransitions<'a>;

    fn transitions(&'a self, StateId(id): StateId) -> Self::TransitionsIter {
        FastFsmTransitions {
            iter: self.states[id].transitions.iter(),
        }
    }

    fn is_terminal(&self, StateId(id): StateId) -> bool {
        // since the terminal states are at the end, if there are N
        // states with k terminal states, then there are (N - k) non-terminal
        // states. If id < (N - k) then it is non-terminal, so if id >= (N - k)
        // it is terminal.

        id >= self.states.len() - self.terminal_count
    }

    fn initial_state(&self) -> StateId {
        // when sorting states, the initial state is always at the beginning.

        StateId(0)
    }

    fn traverse_from(&self, state: StateId, seq: impl FsmSequence) -> Option<StateId> {
        let mut curr_state = state;

        for item in seq.into_iter() {
            let StateId(id) = curr_state;

            curr_state = *self.states[id].transitions.get(&item)?;
        }

        Some(curr_state)
    }
}

pub struct FastFsmTransitions<'a> {
    iter: hash_map::Iter<'a, Letter, StateId>,
}
impl<'a> Iterator for FastFsmTransitions<'a> {
    type Item = (Letter, StateId);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&k, &v)| (k, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build() -> FastFsm {
        let mut builder = FsmBuilder::default();

        builder.insert("bat");
        builder.insert("batman");
        builder.insert("bats");
        builder.insert("cat");
        builder.insert("cats");

        builder.build()
    }

    #[test]
    fn convert() {
        let fast_fsm_1 = build();
        let small_fsm = SmallFsm::from(fast_fsm_1.clone());
        let fast_fsm_2 = FastFsm::from(small_fsm);

        assert_eq!(fast_fsm_1, fast_fsm_2);
    }

    #[test]
    fn accepts() {
        let fast_fsm = build();

        // check for matches
        assert!(fast_fsm.accepts("bat"));
        assert!(fast_fsm.accepts("batman"));
        assert!(fast_fsm.accepts("bats"));
        assert!(fast_fsm.accepts("cat"));
        assert!(fast_fsm.accepts("cats"));

        // check for no match
        assert!(!fast_fsm.accepts("batma"));
        assert!(!fast_fsm.accepts("zzzzz"));
        assert!(!fast_fsm.accepts(""));
    }

    #[test]
    fn traverse() {
        let fast_fsm = build();

        // check for partial paths
        assert!(fast_fsm.traverse("batma").is_some());
        assert!(!fast_fsm.is_terminal(fast_fsm.traverse("batma").unwrap()));
        assert!(fast_fsm.is_terminal(fast_fsm.traverse("batman").unwrap()));
    }

    #[test]
    fn transitions() {
        let fast_fsm = build();

        let transition_count = |seq| {
            fast_fsm
                .transitions(fast_fsm.traverse(seq).unwrap())
                .count()
        };

        // check for transition count
        assert_eq!(2, transition_count("bat"));
        assert_eq!(0, transition_count("batman"));
        assert_eq!(2, transition_count(""));
    }
}
