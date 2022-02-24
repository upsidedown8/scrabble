use crate::util::fsm::{Fsm, FsmBuilder, FsmSequence, StateId};
use std::{
    collections::{hash_map::Keys, HashMap},
    hash::Hash,
};

/// A state in the [`FastFsm`]. Stores only the transitions to other states,
/// in a hashmap.
#[derive(Debug)]
pub struct State<T> {
    pub(super) transitions: HashMap<T, StateId>,
}

/// A time optimised finite state machine.
///
/// States are layed out in a single array, with the initial (and by definition
/// non-terminal) state at index zero, followed by all other non-terminal
/// states, then all terminal states. This way an `is_terminal` flag does not
/// need to be stored for each state.
///
/// This implementation of the [`Fsm`] trait is time optimised, as a hashmap for
/// transitions is fast to access, but may not the most compact representation of
/// the transitions from each state.
#[derive(Debug)]
pub struct FastFsm<T> {
    pub(super) states: Vec<State<T>>,
    pub(super) terminal_count: usize,
}

impl<T: Eq + Hash> From<FsmBuilder<T>> for FastFsm<T> {
    fn from(builder: FsmBuilder<T>) -> Self {
        // the initial state (at index 0) should be non-terminal
        assert!(!builder.states[&StateId(0)].is_terminal);
        // the array to store the new states in
        let mut states = Vec::with_capacity(builder.states.len());

        // find the number of terminal states
        let mut terminal_count = 0;
        for old_state in builder.states.values() {
            if old_state.is_terminal {
                terminal_count += 1;
            }
        }

        // stores the mapping old id -> new id.
        let mut terminal_id = 0;
        let mut non_terminal_id = 0;
        let mut state_id_map = HashMap::new();
        for (&old_state_id, old_state) in builder.states.iter() {
            let state_id = StateId(if old_state.is_terminal {
                terminal_id += 1;
                terminal_count + terminal_id
            } else {
                non_terminal_id += 1;
                non_terminal_id
            });

            state_id_map.insert(old_state_id, state_id);
        }

        // traverse the old states and add to the new states
        for (_, old_state) in builder.states {
            let mut transitions = HashMap::new();

            for (k, old_state_id) in old_state.transitions {
                transitions.insert(k, state_id_map[&old_state_id]);
            }

            states.push(State { transitions });
        }

        Self {
            states,
            terminal_count,
        }
    }
}
impl<'a, T: 'a + Eq + Hash> Fsm<'a, T> for FastFsm<T> {
    type TransitionsIter = Keys<'a, T, StateId>;

    fn transitions(&'a self, StateId(id): StateId) -> Self::TransitionsIter {
        self.states[id].transitions.keys()
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

    fn traverse_from<'b>(&self, state: StateId, seq: impl FsmSequence<'b, T>) -> Option<StateId> {
        let mut curr_state = state;

        for item in seq.into_iter() {
            let StateId(id) = curr_state;

            curr_state = *self.states[id].transitions.get(&item)?;
        }

        Some(curr_state)
    }
}
