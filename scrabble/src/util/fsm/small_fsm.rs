use crate::util::fsm::{Fsm, FsmBuilder, FsmSequence, StateId};
use std::hash::Hash;

use super::FastFsm;

/// A state in the [`SmallFsm`]. Stores an index into the transitions array.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct State(TransitionStartId);

/// An index into the transitions array in [`SmallFsm`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct TransitionStartId(usize);

/// A transition, mapping from one state to another.
#[derive(Clone, Copy, Debug)]
pub struct Transition<T>(T, StateId);

/// A memory optimised finite state machine.
///
/// States contain a 'pointer' to the transitions array, so can
/// be used to iterate over transitions. States are ordered by whether they
/// are terminal, so the position of a state can be compared to the number of
/// terminal states to determine whether the state is terminal.
///
/// This implementation of the [`Fsm`] trait is memory optimised, as the array
/// implementation is very compact, but requires a linear traversal of states.
#[derive(Debug)]
pub struct SmallFsm<T> {
    states: Vec<State>,
    transitions: Vec<Transition<T>>,
    terminal_count: usize,
}

impl<T> SmallFsm<T> {
    /// Gets the start and end of the transition array for a state.
    fn transition_limits(&self, StateId(id): StateId) -> (usize, usize) {
        let State(TransitionStartId(start)) = self.states[id];
        let end = match self.states.get(id + 1) {
            Some(&State(TransitionStartId(end))) => end,
            _ => self.transitions.len(),
        };

        (start, end)
    }
}
impl<T: Hash + Eq> From<FsmBuilder<T>> for SmallFsm<T> {
    fn from(builder: FsmBuilder<T>) -> Self {
        Self::from(FastFsm::from(builder))
    }
}
impl<T: Hash + Eq> From<FastFsm<T>> for SmallFsm<T> {
    fn from(fast_fsm: FastFsm<T>) -> Self {
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
impl<'a, T: 'a + Hash + Eq> Fsm<'a, T> for SmallFsm<T> {
    type TransitionsIter = FastFsmTransitionsIter<'a, T>;

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

    fn traverse_from<'b>(
        &self,
        state_id: StateId,
        seq: impl FsmSequence<'b, T>,
    ) -> Option<StateId> {
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
pub struct FastFsmTransitionsIter<'a, T> {
    slice_iter: std::slice::Iter<'a, Transition<T>>,
}

impl<'a, T: 'a> Iterator for FastFsmTransitionsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice_iter.next().map(|Transition(item, _)| item)
    }
}
