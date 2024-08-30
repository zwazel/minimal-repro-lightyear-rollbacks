use avian3d::{
    prelude::{Physics, PhysicsDebugPlugin, SleepingPlugin},
    sync::{SyncConfig, SyncPlugin},
    PhysicsPlugins,
};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::MyLightyearPlugin;
use my_states::{GameState, InGamePaused, InGameUnpaused, MyStatesPlugin};
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
    .add_systems(Startup, setup_camera)
    .add_systems(OnEnter(InGamePaused), ungrab_mouse)
    .add_systems(OnEnter(InGameUnpaused), grab_mouse)
    .add_systems(
        Update,
        ((pause_unpause_game,).run_if(in_state(InGamePaused).or_else(in_state(InGameUnpaused))),),
    );

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn ungrab_mouse(mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = primary_window_query.get_single_mut() else {
        return;
    };
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

fn grab_mouse(mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = primary_window_query.get_single_mut() else {
        return;
    };
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

fn pause_unpause_game(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Started { paused, .. } => {
                next_state.set(GameState::Started { paused: !paused })
            }
            _ => {}
        };
    }
}
