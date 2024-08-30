use avian3d::{
    prelude::*,
    sync::{SyncConfig, SyncPlugin},
    PhysicsPlugins,
};
use bevy::{
    color::palettes::css,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::MyLightyearPlugin;
use my_states::{GameState, InGame, InGamePaused, InGameUnpaused, MyStatesPlugin};
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
    .add_systems(OnEnter(InGame), spawn_map)
    .add_systems(Update, ((pause_unpause_game,).run_if(in_state(InGame)),));

    app.run();
}

#[derive(Component, Reflect, Default, Debug, PartialEq)]
#[reflect(Component)]
pub struct My3DCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 16.0, 40.0)
                .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
            ..Default::default()
        },
        StateScoped(InGame),
        IsDefaultUiCamera,
        My3DCamera,
    ));
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Point light"),
        PointLightBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0),
            ..default()
        },
        StateScoped(InGame),
    ));

    // A directly-down light to tell where the player is going to land.
    commands.spawn((
        Name::new("Directional light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 4000.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::default().looking_at(-Vec3::Y, Vec3::Z),
            ..Default::default()
        },
        StateScoped(InGame),
    ));

    // Spawn the ground.
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(128.0, 128.0)),
            material: materials.add(Color::WHITE),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        StateScoped(InGame),
    ));

    // Spawn a little platform for the player to jump on.
    commands.spawn((
        Name::new("Platform"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(4.0, 1.0, 4.0)),
            material: materials.add(Color::from(css::GRAY)),
            transform: Transform::from_xyz(-6.0, 2.0, 0.0),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::cuboid(4.0, 1.0, 4.0),
        StateScoped(InGame),
    ));
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
