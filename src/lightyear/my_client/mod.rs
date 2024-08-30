use bevy::prelude::*;
use lightyear::client::{config::ClientConfig, plugin::ClientPlugins};
use spawn_player::SpawnPlayerClientPlugin;

mod spawn_player;

pub struct MyClientPlugin;

impl Plugin for MyClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((build_client_plugin(), SpawnPlayerClientPlugin));
    }
}

fn build_client_plugin() -> ClientPlugins {
    let config = ClientConfig::default();

    ClientPlugins::new(config)
}
