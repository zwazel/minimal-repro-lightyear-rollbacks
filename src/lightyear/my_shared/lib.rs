use std::time::Duration;

use lightyear::prelude::*;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

pub const PLAYER_REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    Main,
    // apply physics steps
    Physics,
}

#[derive(Channel)]
pub struct Channel1;

#[derive(Component, Reflect, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Deref)]
#[reflect(Component)]
pub struct PlayerId(pub(crate) ClientId);

impl Default for PlayerId {
    fn default() -> Self {
        Self(ClientId::Local(0))
    }
}
