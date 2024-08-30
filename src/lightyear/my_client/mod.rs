use bevy::prelude::*;
use lightyear::{client::{config::ClientConfig, plugin::ClientPlugins}, connection::client};
use spawn_player::SpawnPlayerClientPlugin;

mod spawn_player;

pub struct MyClientPlugin;

impl Plugin for MyClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((build_client_plugin(), SpawnPlayerClientPlugin));
    }
}

fn build_client_plugin() -> ClientPlugins {
    let net_config = client::NetConfig::Local { id: 0 };
    let config = ClientConfig {
        net: net_config,
        ..default()
    };

    ClientPlugins::new(config)
}
