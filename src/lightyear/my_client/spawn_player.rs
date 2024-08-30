use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::client::{ClientConnection, Interpolated, NetClient, Predicted};
use lightyear::prelude::is_host_server;
use crate::{
    lightyear::my_shared::lib::{
        PhysicalPlayerBodyBundle, PhysicalPlayerBodyMarker, PhysicalPlayerHeadBundle,
        PhysicalPlayerHeadMarker, PhysicsBundle, PlayerActions, PlayerId,
    },
    my_states::InGame,
};

pub struct SpawnPlayerClientPlugin;

impl Plugin for SpawnPlayerClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGame), spawn_physical_player.run_if(not(is_host_server)))
            .add_systems(Update, add_non_replicated_to_players);
    }
}

// TODO: if head and body both contribute to movement, they should both be part
//  of the same replication group
fn spawn_physical_player(connection: Res<ClientConnection>, mut commands: Commands) {
    commands
        .spawn((
            PhysicalPlayerBodyBundle::new(
                InputMap::new([(PlayerActions::Jump, KeyCode::Space)])
                    .with_dual_axis(
                        PlayerActions::Move,
                        KeyboardVirtualDPad::new(
                            KeyCode::KeyW,
                            KeyCode::KeyS,
                            KeyCode::KeyA,
                            KeyCode::KeyD,
                        ),
                    )
                    .with_dual_axis(PlayerActions::LookAround, MouseMove::default()),
                Collider::capsule(0.4, 1.0),
                connection.client.id(),
            ),
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 5.0, 0.0)),
        ))
        .with_children(|commands| {
            commands.spawn(PhysicalPlayerHeadBundle::new(connection.client.id()));
        });
}

/// When we receive other players (whether they are predicted or interpolated), we want to add the physics components
/// so that our predicted entities can predict collisions with them correctly
fn add_non_replicated_to_players(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    player_query: Query<
        (Entity, &PlayerId),
        (
            Or<(Added<Interpolated>, Added<Predicted>)>,
            With<PhysicalPlayerBodyMarker>,
        ),
    >,
    player_head: Query<(Entity, &PlayerId), Added<PhysicalPlayerHeadMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let client_id = connection.client.id();
    for (entity, player_id) in player_query.iter() {
        if player_id.0 == client_id {
            // only need to do this for other players' entities
            info!("Skipping adding physics to own player entity: {:?}", entity);
            continue;
        }
        info!(
            "Adding physics to player entity: {:?} / client: {:?}",
            entity, player_id
        );
        commands.entity(entity).insert((
            PhysicsBundle::player(),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::from(css::YELLOW)),
                ..default()
            },
        ));
    }

    for (entity, player_id) in player_head.iter() {
        if player_id.0 == client_id {
            continue;
        }
        info!(
            "Adding render stuff to player head entity: {:?} / client: {:?}",
            entity, player_id
        );

        commands.entity(entity).insert((PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::from(css::RED)),
            ..default()
        },));
    }
}
