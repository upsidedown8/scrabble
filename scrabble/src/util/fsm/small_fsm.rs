use crate::{
    game::tile::Letter,
    util::fsm::{FastFsm, Fsm, FsmBuilder, FsmSequence, StateId},
};
use serde::{Deserialize, Serialize};

/// A state in the [`SmallFsm`]. Stores an index into the transitions array.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct State(TransitionStartId);

/// An index into the transitions array in [`SmallFsm`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TransitionStartId(usize);

/// A transition, mapping from one state to another.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Transition(pub(super) Letter, pub(super) StateId);

/// A memory optimised finite state machine.
///
/// States contain a 'pointer' to the transitions array, so can
/// be used to iterate over transitions. States are ordered by whether they
/// are terminal, so the position of a state can be compared to the number of
/// terminal states to determine whether the state is terminal.
///
/// This implementation of the [`Fsm`] trait is memory optimised, as the array
/// implementation is very compact, but requires a linear traversal of states.
#[derive(Debug, Serialize, Deserialize)]
pub struct SmallFsm {
    pub(super) states: Vec<State>,
    pub(super) transitions: Vec<Transition>,
    pub(super) terminal_count: usize,
}

impl SmallFsm {
    /// Gets the start and end of the transition array for a state.
    pub fn transition_limits(&self, StateId(id): StateId) -> (usize, usize) {
        let State(TransitionStartId(start)) = self.states[id];
        let end = match self.states.get(id + 1) {
            Some(&State(TransitionStartId(end))) => end,
            _ => self.transitions.len(),
        };

        (start, end)
    }
}
impl From<FsmBuilder> for SmallFsm {
    fn from(builder: FsmBuilder) -> Self {
        Self::from(FastFsm::from(builder))
    }
}
impl From<FastFsm> for SmallFsm {
    fn from(fast_fsm: FastFsm) -> Self {
        // reuse the code for the fast fsm.
        let FastFsm {
            states,
            terminal_count,
        } = fast_fsm;

        let mut small_states = Vec::with_capacity(states.len());
        let mut transitions = Vec::new();

        // add the states in the same order as the fast fsm.
        let mut transition_id = 0;
        for state in states {
            small_states.push(State(TransitionStartId(transition_id)));

            // add each transition to the array
            for (k, state_id) in state.transitions {
                // can reuse the state_id as the ordering is unchanged.
                transitions.push(Transition(k, state_id));
                transition_id += 1;
            }
        }

        Self {
            states: small_states,
            transitions,
            terminal_count,
        }
    }
}
impl<'a> Fsm<'a> for SmallFsm {
    type TransitionsIter = FastFsmTransitionsIter<'a>;

    fn transitions(&'a self, state_id: StateId) -> Self::TransitionsIter {
        let (start, end) = self.transition_limits(state_id);
        FastFsmTransitionsIter {
            slice_iter: self.transitions[start..end].iter(),
        }
    }

    fn is_terminal(&self, StateId(id): StateId) -> bool {
        id >= self.states.len() - self.terminal_count
    }

    fn initial_state(&self) -> StateId {
        StateId(0)
    }

    fn traverse_from(&self, state_id: StateId, seq: impl FsmSequence) -> Option<StateId> {
        let mut curr_state = state_id;

        'outer: for item in seq.into_iter() {
            let (start, end) = self.transition_limits(curr_state);

            for i in start..end {
                let Transition(k, next_state) = &self.transitions[i];

                if item.eq(k) {
                    curr_state = *next_state;

                    // move to next item
                    continue 'outer;
                }
            }

            // no matching transition found
            return None;
        }

        Some(curr_state)
    }
}

/// Used to iterate over the transitions in the [`SmallFsm`].
pub struct FastFsmTransitionsIter<'a> {
    slice_iter: std::slice::Iter<'a, Transition>,
}

impl<'a> Iterator for FastFsmTransitionsIter<'a> {
    type Item = Letter;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice_iter.next().map(|Transition(item, _)| *item)
    }
}
