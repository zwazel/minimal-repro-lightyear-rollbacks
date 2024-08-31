use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::is_host_server;

use crate::lightyear::my_shared::{
    lib::{FixedSet, PhysicalPlayerBodyMarker, PhysicalPlayerHeadMarker, PlayerActions, PlayerId},
    movement::shared_movement,
    physics::lib::{Grounded, JumpImpulse, MaxMovementSpeed, MovementAcceleration},
};

pub struct MyServerMovementPlugin;

impl Plugin for MyServerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            movement_server
                .run_if(is_host_server)
                .in_set(FixedSet::Main),
        );
    }
}

fn movement_server(
    mut player_head_query: Query<
        (&mut Transform, &mut PhysicalPlayerHeadMarker),
        (Without<PhysicalPlayerBodyMarker>,),
    >,
    mut player_body_controllers: Query<
        (
            &Transform,
            &MovementAcceleration,
            &MaxMovementSpeed,
            &JumpImpulse,
            &mut LinearVelocity,
            &mut Rotation,
            &mut PhysicalPlayerBodyMarker,
            &ActionState<PlayerActions>,
            Has<Grounded>,
        ),
        (
            With<PlayerId>,
            Without<PhysicalPlayerHeadMarker>,
            With<Children>,
        ),
    >,
) {
    for (
        transform,
        movement_acceleration,
        max_speed,
        jump_impulse,
        mut body_velocity,
        mut body_rotation,
        mut player_body,
        action_state,
        is_grounded,
    ) in &mut player_body_controllers
    {
        let (mut head_transform, mut head) = player_head_query
            .get_mut(player_body.head_entity.unwrap())
            .unwrap();

        shared_movement(
            transform,
            jump_impulse,
            movement_acceleration,
            max_speed,
            &mut body_velocity,
            &mut body_rotation,
            &mut player_body,
            &mut head_transform,
            &mut head,
            is_grounded,
            action_state,
        );
    }
}
