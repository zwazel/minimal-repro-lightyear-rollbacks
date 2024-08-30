use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;
use client::{ComponentSyncMode, NetworkingState as ClientNetworkingState};
use lib::{
    Channel1, FixedSet, PhysicalPlayerBodyMarker, PhysicalPlayerHeadMarker, PlayerActions,
    PlayerId, SERVER_REPLICATION_INTERVAL,
};
use lightyear::{
    prelude::*,
    utils::avian3d::{position, rotation},
};
use lightyear::client::prediction::Predicted;
use lightyear::prelude::client::{Rollback, RollbackState};
use renderer::MyRendererPlugin;
use server::NetworkingState as ServerNetworkingState;

use crate::{my_states::GameState, FIXED_TIMESTEP_HZ};

pub mod lib;
pub mod movement;
pub mod physics;
mod renderer;

pub struct MySharedPlugin;

const ROLLBACK_THRESHOLD: f32 = 0.0001;

/// Returns true if the difference in position is greater than a threshold
fn should_rollback_position(this: &Position, that: &Position) -> bool {
    !this.abs_diff_eq(that.0, ROLLBACK_THRESHOLD)
}

fn should_rollback_linear_velocity(this: &LinearVelocity, that: &LinearVelocity) -> bool {
    !this.abs_diff_eq(that.0, ROLLBACK_THRESHOLD)
}

impl Plugin for MySharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MyRendererPlugin,
            LeafwingInputPlugin::<PlayerActions>::default(),
        ))
        .configure_sets(
            FixedUpdate,
            (
                // make sure that any physics simulation happens after the Main SystemSet
                // (where we apply user's actions)
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .in_set(FixedSet::Physics),
                (FixedSet::Main, FixedSet::Physics).chain(),
            ),
        )
        .add_systems(OnEnter(ClientNetworkingState::Connected), go_ingame)
        .add_systems(OnEnter(ServerNetworkingState::Started), go_ingame);

        app.add_systems(Last, debug_position);

        app.register_type::<PlayerId>()
            .register_type::<PhysicalPlayerHeadMarker>()
            .register_type::<PhysicalPlayerBodyMarker>();

        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<PhysicalPlayerHeadMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();

        app.register_component::<PhysicalPlayerBodyMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn()
            .add_map_entities();

        app.register_component::<Name>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });

        // General Physics stuff
        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            // .add_should_rollback(should_rollback_position)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp)
            .add_correction_fn(rotation::lerp);

        // NOTE: interpolation/correction is only needed for components that are visually displayed!
        // we still need prediction to be able to correctly predict the physics on the client
        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            // .add_should_rollback(should_rollback_linear_velocity)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);
    }
}

fn go_ingame(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Started { paused: false });
}

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        mode: Mode::HostServer,
    }
}


pub fn debug_position(
    tick_manager: Res<TickManager>,
    rollback: Res<Rollback>,
    q: Query<(Entity, &Position), With<Predicted>>,
) {
    let is_rollback = rollback.is_rollback();
    let tick = tick_manager.tick_or_rollback_tick(rollback.as_ref());
    for (e, pos) in q.iter() {
        info!(?tick, ?e, ?pos, ?is_rollback, "last")
    }
}