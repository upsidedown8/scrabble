use crate::{
    game::tile::Letter,
    util::fsm::{Fsm, FsmSequence, StateId},
};
use std::{collections::HashMap, hash::Hash};

/// Represents a state in a finite state machine. `is_terminal` determines
/// whether it is an acceptance state.
#[derive(Debug, Default)]
pub struct State {
    pub(super) is_terminal: bool,
    pub(super) transitions: HashMap<Letter, StateId>,
}

impl State {
    /// Gets the transition label with greatest value, which is the label
    /// that was added most recently, since insertion occurs in alphabetical
    /// order.
    pub fn last_transition(&self) -> Option<&Letter> {
        self.transitions.keys().max()
    }
    fn hash_recursive(&self, builder: &FsmBuilder, state: &mut String) {
        // hash the data for the current node
        if self.is_terminal {
            state.push('Y');
        } else {
            state.push('N');
        }

        state.push('[');

        for (key, &value) in self.transitions.iter() {
            state.push(char::from(*key));
            state.push('[');

            // do not hash the `StateId`, as it is not relevant for determining
            // whether 2 states are identical.
            builder.state(value).hash_recursive(builder, state);

            state.push(']');
        }

        state.push(']');
    }
    /// Recursively computes a hash for the state based on whether it is terminal
    /// and its transitions.
    pub(self) fn state_hash(&self, builder: &FsmBuilder) -> PerfectHash {
        let mut state = String::new();

        self.hash_recursive(builder, &mut state);

        PerfectHash(state)
    }
}

/// Used to check whether two states are identical.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Hash, Debug, Default)]
pub struct PerfectHash(String);

/// Used to construct a finite state machine.
#[derive(Debug)]
pub struct FsmBuilder {
    pub(super) states: HashMap<StateId, State>,
    previous_seq: Vec<Letter>,
    register: HashMap<PerfectHash, StateId>,
    position_stack: Vec<StateId>,
}

impl Default for FsmBuilder {
    fn default() -> Self {
        let mut states = HashMap::new();

        states.insert(
            StateId(0),
            State {
                is_terminal: false,
                transitions: HashMap::default(),
            },
        );

        Self {
            states,
            previous_seq: Vec::new(),
            register: HashMap::new(),
            position_stack: vec![StateId(0)],
        }
    }
}
impl FsmBuilder {
    /// Constructs an immutable [`Fsm`] from the builder.
    pub fn build<'a, F: Fsm<'a>>(mut self) -> F {
        // find shortcuts up to the initial state.
        self.replace_or_register(StateId(0));

        F::from(self)
    }
    /// Inserts a word into the [`FsmBuilder`]. Words must be inserted in
    /// alphabetical order.
    pub fn insert(&mut self, seq: impl FsmSequence) {
        let seq: Vec<_> = seq.into_iter().collect();
        let prefix_len = Self::common_prefix_len(&self.previous_seq, &seq);

        // traverse backwards to last node
        self.position_stack.truncate(prefix_len + 1);
        // the most recent state is the last value.
        let last_state_id = self.position_stack[prefix_len];

        self.replace_or_register(last_state_id);
        self.add_suffix(last_state_id, &seq[prefix_len..]);
        self.previous_seq = seq;
    }
    /// Gets a reference to a [`State`] by id.
    #[inline]
    fn state(&self, id: StateId) -> &State {
        &self.states[&id]
    }
    /// Gets a mutable reference to a [`State`] by id.
    #[inline]
    fn state_mut(&mut self, id: StateId) -> &mut State {
        self.states.get_mut(&id).unwrap()
    }
    /// Finds the length of the substring that is common to both strings,
    /// and starts at the beginning.
    #[inline]
    fn common_prefix_len(a: &[Letter], b: &[Letter]) -> usize {
        a.iter().zip(b).take_while(|&(a, b)| a == b).count()
    }
    /// Inserts a suffix into the fsm. The suffix should be a new branch.
    fn add_suffix(&mut self, last_state_id: StateId, suffix: &[Letter]) {
        let mut curr_state_id = last_state_id;

        for &ch in suffix {
            // the suffix should be a completely new branch.
            debug_assert!(self.state(curr_state_id).transitions.get(&ch).is_none());

            let id = StateId(self.states.len());
            self.states.insert(id, State::default());
            self.state_mut(curr_state_id).transitions.insert(ch, id);
            curr_state_id = id;

            // update the stack
            self.position_stack.push(id);
        }

        // the last state must be terminal
        self.state_mut(curr_state_id).is_terminal = true;
    }
    /// Attempts to replace nodes in the register with identical
    /// pre-existing nodes to cut down the graph size.
    fn replace_or_register(&mut self, state_id: StateId) {
        if let Some(&child_transition) = self.state(state_id).last_transition() {
            let child_id = self.state(state_id).transitions[&child_transition];
            self.replace_or_register(child_id);

            // check whether any node in the register is identical
            let child_hash = self.state(child_id).state_hash(self);

            match self.register.get(&child_hash) {
                Some(&node_id) => {
                    self.state_mut(state_id)
                        .transitions
                        .insert(child_transition, node_id);
                    self.states.remove(&child_id);
                }
                _ => {
                    self.register.insert(child_hash, child_id);
                }
            }
        }
    }
}
