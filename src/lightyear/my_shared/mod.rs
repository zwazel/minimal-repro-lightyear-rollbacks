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
use renderer::MyRendererPlugin;
use server::NetworkingState as ServerNetworkingState;

use crate::{my_states::GameState, FIXED_TIMESTEP_HZ};

pub mod lib;
pub mod movement;
pub mod physics;
mod renderer;

pub struct MySharedPlugin;

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
