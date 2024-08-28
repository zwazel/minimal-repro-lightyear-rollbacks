use bevy::prelude::*;

mod my_client;
mod my_server;
mod my_shared;

pub struct MyLightyearPlugin;

impl Plugin for MyLightyearPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            my_client::MyClientPlugin,
            my_server::MyServerPlugin,
            my_shared::MySharedPlugin,
        ));
    }
}
