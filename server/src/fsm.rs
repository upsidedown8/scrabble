//! Stores a `FastFsm` that can be shared across threads.

use crate::error::Result;
use scrabble::util::fsm::FastFsm;
use std::{env, ops::Deref, sync::Arc};

/// A structure that contains a thread safe immutable reference
/// to a `scrabble::util::fsm::Fsm` impl.
#[derive(Clone, Debug)]
pub struct FsmRef(Arc<FastFsm>);
impl FsmRef {
    /// Loads the Fsm from env variables.
    pub fn new_from_env() -> Result<Self> {
        let fsm_path = env::var("FAST_FSM_BIN").expect("`FAST_FSM_BIN` env variable");

        log::info!("loading fast fsm: {fsm_path}");
        let file = std::fs::File::open(&fsm_path)?;
        let rdr = std::io::BufReader::new(file);
        let fast_fsm: FastFsm = bincode::deserialize_from(rdr)?;

        Ok(Self(Arc::new(fast_fsm)))
    }
}
impl Deref for FsmRef {
    type Target = FastFsm;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
