use crate::{
    game::tile::Letter,
    util::fsm::{Fsm, FsmSequence, StateId},
};
use std::collections::{HashMap, HashSet};

/// Represents a state in a finite state machine. `is_terminal` determines
/// whether it is an acceptance state.
#[derive(Debug)]
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
}

/// Used to construct a finite state machine.
#[derive(Debug)]
pub struct FsmBuilder {
    pub(super) states: HashMap<StateId, State>,
    previous_seq: Vec<Letter>,
    register: HashSet<StateId>,
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
            register: HashSet::new(),
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

        let prefix = &seq[0..prefix_len];
        let suffix = &seq[prefix_len..];

        let last_state_id = self.traverse(prefix);

        self.replace_or_register(last_state_id);
        self.add_suffix(last_state_id, suffix);
        self.previous_seq = seq;
    }
    /// Gets a reference to a [`State`] by id.
    fn state(&self, id: StateId) -> &State {
        &self.states[&id]
    }
    /// Gets a mutable reference to a [`State`] by id.
    fn state_mut(&mut self, id: StateId) -> &mut State {
        self.states.get_mut(&id).expect("State to be present")
    }
    /// Finds the length of the substring that is common to both strings,
    /// and starts at the beginning.
    fn common_prefix_len(a: &[Letter], b: &[Letter]) -> usize {
        a.iter().zip(b).take_while(|&(a, b)| a == b).count()
    }
    /// Traverses a path through the fsm.
    fn traverse(&self, prefix: &[Letter]) -> StateId {
        // start with the initial node.
        let mut curr_state_id = StateId(0);

        for ch in prefix {
            curr_state_id = self.state(curr_state_id).transitions[ch];
        }

        curr_state_id
    }
    /// Inserts a suffix into the fsm. The suffix should be a new branch.
    fn add_suffix(&mut self, last_state_id: StateId, suffix: &[Letter]) {
        let mut curr_state_id = last_state_id;

        for &ch in suffix {
            // the suffix should be a completely new branch.
            debug_assert!(self.state(curr_state_id).transitions.get(&ch).is_none());

            let id = StateId(self.states.len());
            self.states.insert(
                id,
                State {
                    is_terminal: false,
                    transitions: HashMap::new(),
                },
            );

            self.state_mut(curr_state_id).transitions.insert(ch, id);
            curr_state_id = id;
        }

        self.state_mut(curr_state_id).is_terminal = true;
    }
    /// Attempts to replace nodes in the register with identical
    /// pre-existing nodes to cut down the graph size.
    fn replace_or_register(&mut self, state_id: StateId) {
        if let Some(&child_transition) = self.state(state_id).last_transition() {
            let child_id = self.state(state_id).transitions[&child_transition];

            self.replace_or_register(child_id);

            // if any node in the register is identical to the child node
            let identical_node = self
                .register
                .iter()
                .copied()
                .find(|&node_id| self.states_eq(node_id, child_id));

            match identical_node {
                Some(node_id) => {
                    self.state_mut(state_id)
                        .transitions
                        .insert(child_transition, node_id);
                    self.delete_state(child_id);
                }
                None => {
                    self.register.insert(child_id);
                }
            }
        }
    }
    /// Remove a state by id (does not remove children).
    fn delete_state(&mut self, id: StateId) {
        self.states.remove(&id);
    }
    /// Recursively checks whether two states are identical.
    ///     - both are terminal or not terminal
    ///     - all children are identical
    fn states_eq(&self, a_id: StateId, b_id: StateId) -> bool {
        let a_state = self.state(a_id);
        let b_state = self.state(b_id);

        // quick check: same terminal state
        if a_state.is_terminal != b_state.is_terminal {
            return false;
        }

        // check that children are equal for both:
        self.children_eq(a_state, b_state)
    }
    /// Recursively checks that the children of two states are identical.
    fn children_eq(&self, a_state: &State, b_state: &State) -> bool {
        // quick check: same number of children
        if a_state.transitions.len() != b_state.transitions.len() {
            return false;
        }

        // check that the transition on each state is the same.
        // this performs a breadth-first check for a depth of 1,
        // which reduces the number of recursive calls, at a cost of
        // iterating the hashmaps twice.
        if a_state
            .transitions
            .keys()
            .zip(b_state.transitions.keys())
            .any(|(&a, &b)| a != b)
        {
            return false;
        }

        // call `states_eq` to check that the nodes for each transition
        // are identical.
        a_state
            .transitions
            .values()
            .zip(b_state.transitions.values())
            .all(|(&a_id, &b_id)| self.states_eq(a_id, b_id))
    }
}
