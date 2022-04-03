use crate::handlers::live::room::RoomHandle;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

/// Type containing a thread-safe handle to all the game rooms.
#[derive(Clone, Debug, Default)]
pub struct RoomsHandle(Arc<RwLock<Rooms>>);
impl Deref for RoomsHandle {
    type Target = RwLock<Rooms>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug, Default)]
pub struct Rooms(HashMap<i32, RoomHandle>);
impl Rooms {
    /// Inserts a room into the list of rooms.
    pub fn insert(&mut self, id_room: i32, handle: RoomHandle) {
        let Rooms(map) = self;
        map.insert(id_room, handle);
    }
    /// Gets a reference to a room.
    pub fn room(&self, id_room: i32) -> Option<RoomHandle> {
        let Rooms(map) = self;
        map.get(&id_room).cloned()
    }
}
