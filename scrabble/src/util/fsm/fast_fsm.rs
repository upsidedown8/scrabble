use crate::{
    game::tile::Letter,
    util::fsm::{small_fsm::Transition, Fsm, FsmBuilder, FsmSequence, StateId},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::Keys, HashMap},
    iter,
};

use super::SmallFsm;

/// A state in the [`FastFsm`]. Stores only the transitions to other states,
/// in a hashmap.
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct FastFsm {
    pub(super) states: Vec<State>,
    pub(super) terminal_count: usize,
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
                .map(|Transition(letter, state_id)| (letter, state_id))
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
    type TransitionsIter = iter::Copied<Keys<'a, Letter, StateId>>;

    fn transitions(&'a self, StateId(id): StateId) -> Self::TransitionsIter {
        self.states[id].transitions.keys().copied()
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
