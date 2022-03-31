//! Module containing an implementation of a Finite State Machine (FSM)
//! which can be used to efficiently traverse the space of available words.
//! Module containing a word tree.

mod builder;
mod fast_fsm;
mod small_fsm;

use crate::game::tile::Letter;
use serde::{Deserialize, Serialize};
use std::{iter, str::Chars};

pub use builder::FsmBuilder;
pub use fast_fsm::FastFsm;
pub use small_fsm::SmallFsm;

/// Used to identify a state in a Finite State Machine ([`Fsm`]).
#[repr(transparent)]
#[derive(Hash, Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct StateId(usize);

/// Trait implemented by [`SmallFsm`] and [`FastFsm`]. Provides
/// operations for constructing and traversing a finite state
/// machine. Once constructed, finite state machines are immutable.
///
/// A finite state machine implementation is constructed using
/// [`FsmBuilder`] (but can be serialized for faster load times).
pub trait Fsm<'a>: From<FsmBuilder> {
    /// An iterator over the transitions from a state.
    type TransitionsIter: Iterator<Item = (Letter, StateId)> + 'a;

    /// Gets the number of transitions.
    fn transition_count(&self) -> usize;
    /// Gets the number of states.
    fn state_count(&self) -> usize;
    /// Gets an iterator over the transitions from a state.
    fn transitions(&'a self, state: StateId) -> Self::TransitionsIter;
    /// Checks whether a state is terminal (ends a valid sequence).
    fn is_terminal(&self, state: StateId) -> bool;
    /// Gets the initial state id.
    fn initial_state(&self) -> StateId;
    /// Checks whether a letter sequence is accepted by the
    /// finite state machine.
    fn accepts(&self, seq: impl FsmSequence) -> bool {
        // default implementation: traverse and check result
        self.traverse(seq)
            .map(|state_id| self.is_terminal(state_id))
            .unwrap_or(false)
    }
    /// Traverses a sequence through the finite state machine.
    fn traverse(&self, seq: impl FsmSequence) -> Option<StateId> {
        // default implementation: traverse from initial state
        self.traverse_from(self.initial_state(), seq)
    }
    /// Traverses a sequence through the finite state machine
    /// from a given initial `state`.
    fn traverse_from(&self, state: StateId, seq: impl FsmSequence) -> Option<StateId>;
}

/// A sequence provided as input to a finite state machine.
pub trait FsmSequence {
    /// Type performing the iteration.
    type Iter: Iterator<Item = Letter>;

    /// Returns an iterator over type `T`.
    fn into_iter(self) -> Self::Iter;
}

/// Used to iterate over the [`Letter`]s in a string.
pub struct FsmCharsIter<'a> {
    inner: Chars<'a>,
}

impl<'a> Iterator for FsmCharsIter<'a> {
    type Item = Letter;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(Letter::new)
    }
}
impl<'a> FsmSequence for &'a str {
    type Iter = FsmCharsIter<'a>;

    fn into_iter(self) -> Self::Iter {
        FsmCharsIter {
            inner: self.chars(),
        }
    }
}
impl<'a> FsmSequence for &'a String {
    type Iter = FsmCharsIter<'a>;

    fn into_iter(self) -> Self::Iter {
        FsmCharsIter {
            inner: self.chars(),
        }
    }
}
impl<'a> FsmSequence for &'a [Letter] {
    type Iter = iter::Copied<std::slice::Iter<'a, Letter>>;

    fn into_iter(self) -> Self::Iter {
        self.iter().copied()
    }
}
impl FsmSequence for Letter {
    type Iter = iter::Once<Letter>;

    fn into_iter(self) -> Self::Iter {
        iter::once(self)
    }
}
impl FsmSequence for char {
    type Iter = std::option::IntoIter<Letter>;

    fn into_iter(self) -> Self::Iter {
        Letter::new(self).into_iter()
    }
}
