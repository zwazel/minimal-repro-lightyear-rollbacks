use avian3d::{
    prelude::{Physics, PhysicsDebugPlugin, SleepingPlugin},
    sync::{SyncConfig, SyncPlugin},
    PhysicsPlugins,
};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::MyLightyearPlugin;

pub(crate) const FIXED_TIMESTEP_HZ: f64 = 64.0;

mod lightyear;

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
    ))
    .insert_resource(SyncConfig {
        transform_to_position: false,
        position_to_transform: true,
    })
    .insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)));

    app.run();
}
