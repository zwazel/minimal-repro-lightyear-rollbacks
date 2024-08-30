use bevy::prelude::*;
use lightyear::prelude::*;
use server::{
    ControlledBy, IoConfig, NetConfig, NetcodeConfig, Replicate, ServerConfig, ServerPlugins,
    ServerTransport,
};

use super::{
    lib::SERVER_ADDR,
    my_shared::{lib::SERVER_REPLICATION_INTERVAL, shared_config},
};

pub struct MyServerPlugin;

impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(build_server_plugin())
            .add_systems(Update, handle_connections.run_if(is_host_server));
    }
}

fn handle_connections(
    mut commands: Commands,
    mut connection_event: EventReader<ServerConnectEvent>,
) {
    for event in connection_event.read() {
        #[cfg(debug_assertions)]
        info!("Client connected: {}", event.client_id);
        commands.spawn((
            Name::new(format!("Player {}", event.client_id)),
            Replicate {
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(event.client_id),
                    ..default()
                },
                ..default()
            },
        ));
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
