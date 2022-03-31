use crate::{
    game::tile::Letter,
    util::fsm::{FastFsm, Fsm, FsmBuilder, FsmSequence, StateId},
};
use serde::{Deserialize, Serialize};

/// Used to identify a [`State`]. Uses fewer bits that [`StateId`] so
/// that it takes up smaller storage space.
#[repr(transparent)]
#[derive(Hash, Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct SmallStateId(pub(super) u32);

/// A state in the [`SmallFsm`]. Stores an index into the transitions array.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct State(TransitionStartId);

/// An index into the transitions array in [`SmallFsm`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionStartId(u32);

/// A transition, mapping from one state to another.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition(pub(super) Letter, pub(super) SmallStateId);

/// A memory optimised finite state machine.
///
/// States contain a 'pointer' to the transitions array, so can
/// be used to iterate over transitions. States are ordered by whether they
/// are terminal, so the position of a state can be compared to the number of
/// terminal states to determine whether the state is terminal.
///
/// This implementation of the [`Fsm`] trait is memory optimised, as the array
/// implementation is very compact, but requires a linear traversal of states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
            Some(&State(TransitionStartId(end))) => end as usize,
            _ => self.transition_count(),
        };

        (start as usize, end)
    }
}
impl From<FsmBuilder> for SmallFsm {
    fn from(builder: FsmBuilder) -> Self {
        Self::from(FastFsm::from(builder))
    }
}
impl From<FastFsm> for SmallFsm {
    fn from(fast_fsm: FastFsm) -> Self {
        assert!(fast_fsm.state_count() < u32::MAX as usize);
        assert!(fast_fsm.transition_count() < u32::MAX as usize);

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
            for (k, StateId(id)) in state.transitions {
                // can reuse the state_id as the ordering is unchanged.
                transitions.push(Transition(k, SmallStateId(id as u32)));
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
    type TransitionsIter = FastFsmTransitions<'a>;

    fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    fn state_count(&self) -> usize {
        self.states.len()
    }

    fn transitions(&'a self, state_id: StateId) -> Self::TransitionsIter {
        let (start, end) = self.transition_limits(state_id);
        FastFsmTransitions {
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

        for item in seq.into_iter() {
            curr_state = self
                .transitions(curr_state)
                .find(|(k, _)| *k == item)
                .map(|(_, state)| state)?;
        }

        Some(curr_state)
    }
}

/// Used to iterate over the transitions in the [`SmallFsm`].
pub struct FastFsmTransitions<'a> {
    slice_iter: std::slice::Iter<'a, Transition>,
}

impl<'a> Iterator for FastFsmTransitions<'a> {
    type Item = (Letter, StateId);

    fn next(&mut self) -> Option<Self::Item> {
        self.slice_iter
            .next()
            .map(|&Transition(item, SmallStateId(id))| (item, StateId(id as usize)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build() -> SmallFsm {
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
        let small_fsm_1 = build();
        let fast_fsm = FastFsm::from(small_fsm_1.clone());
        let small_fsm_2 = SmallFsm::from(fast_fsm);

        assert_eq!(
            small_fsm_1.transition_count(),
            small_fsm_2.transition_count()
        );
        assert_eq!(small_fsm_1.state_count(), small_fsm_2.state_count());
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
