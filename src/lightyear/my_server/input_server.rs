use bevy::prelude::*;
use lightyear::prelude::{
    is_host_server, InputChannel, InputMessage, MainSet, NetworkTarget, ServerConnectionManager,
    ServerMessageEvent,
};

use crate::lightyear::my_shared::lib::PlayerActions;

pub struct MyServerInputPlugin;

impl Plugin for MyServerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            replicate_inputs
                .after(MainSet::EmitEvents)
                .run_if(is_host_server),
        );
    }
}

fn replicate_inputs(
    mut connection: ResMut<ServerConnectionManager>,
    mut input_events: ResMut<Events<ServerMessageEvent<InputMessage<PlayerActions>>>>,
) {
    for mut event in input_events.drain() {
        let client_id = *event.context();

        // Optional: do some validation on the inputs to check that there's no cheating

        // rebroadcast the input to other clients
        connection
            .send_message_to_target::<InputChannel, _>(
                &mut event.message,
                NetworkTarget::AllExceptSingle(client_id),
            )
            .unwrap()
    }
}
