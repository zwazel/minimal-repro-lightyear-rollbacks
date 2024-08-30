use avian3d::{
    prelude::{Physics, PhysicsDebugPlugin, SleepingPlugin},
    sync::{SyncConfig, SyncPlugin},
    PhysicsPlugins,
};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::MyLightyearPlugin;
use my_states::MyStatesPlugin;
use my_ui::MyUiPlugin;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;

mod lightyear;
pub mod my_states;
mod my_ui;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        WorldInspectorPlugin::default(),
        PhysicsDebugPlugin::new(FixedUpdate),
        PhysicsPlugins::new(FixedUpdate)
            .build()
            .disable::<SyncPlugin>()
            .disable::<SleepingPlugin>(),
        SyncPlugin::new(PostUpdate),
        MyLightyearPlugin,
        MyStatesPlugin,
        MyUiPlugin,
    ))
    .insert_resource(SyncConfig {
        transform_to_position: false,
        position_to_transform: true,
    })
    .insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)))
    .add_systems(Startup, setup_camera);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
