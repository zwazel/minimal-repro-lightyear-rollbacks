use bevy::prelude::*;
use lightyear::prelude::*;
use server::{
    ControlledBy, IoConfig, NetConfig, NetcodeConfig, Replicate, ServerConfig, ServerPlugins,
    ServerTransport, SyncTarget,
};

use super::{
    lib::SERVER_ADDR,
    my_shared::{
        lib::{PhysicalPlayerBodyMarker, PLAYER_REPLICATION_GROUP, SERVER_REPLICATION_INTERVAL},
        shared_config,
    },
};

pub struct MyServerPlugin;

impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(build_server_plugin())
            .add_systems(Update, replicate_players.run_if(is_host_server));
    }
}

// Replicate the pre-predicted entities back to the client(s)
fn replicate_players(
    mut commands: Commands,
    query_body: Query<(Entity, &Replicated), (Added<Replicated>, With<PhysicalPlayerBodyMarker>)>,
) {
    for (entity, replicated) in query_body.iter() {
        let client_id = replicated.client_id();

        // for all player entities we have received, add a Replicate component so that we can start replicating it
        // to other clients
        if let Some(mut e) = commands.get_entity(entity) {
            // we want to replicate back to the original client, since they are using a pre-predicted entity
            let mut sync_target = SyncTarget::default();
            sync_target.prediction = NetworkTarget::All;

            let replicate = Replicate {
                sync: sync_target,
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                // make sure that all entities that are predicted are part of the same replication group
                group: PLAYER_REPLICATION_GROUP,
                ..default()
            };
            e.insert((
                replicate,
                // if we receive a pre-predicted entity, only send the prepredicted component back
                // to the original client
                OverrideTargetComponent::<PrePredicted>::new(NetworkTarget::Single(client_id)),
                // not all physics components are replicated over the network, so add them on the server as well
                //PhysicsBundle::player(), // we always run in host server, the client will insert this himself
            ));
        }
    }
}

fn build_server_plugin() -> ServerPlugins {
    let io = IoConfig {
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };
    let config = ServerConfig {
        shared: shared_config(),
        net: vec![net_config],
        replication: ReplicationConfig {
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    ServerPlugins::new(config)
}
