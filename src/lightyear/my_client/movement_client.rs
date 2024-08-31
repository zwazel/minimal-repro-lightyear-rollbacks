use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::{
    inputs::leafwing::input_buffer::InputBuffer,
    prelude::{
        client::{Predicted, Rollback},
        is_host_server, TickManager,
    },
};

use crate::{
    lightyear::my_shared::{
        lib::{
            FixedSet, PhysicalPlayerBodyMarker, PhysicalPlayerHeadMarker, PlayerActions, PlayerId,
        },
        movement::shared_movement,
        physics::lib::{Grounded, JumpImpulse, MaxMovementSpeed, MovementAcceleration},
    },
    my_states::InGameUnpaused,
};

pub struct MyClientMovementPlugin;

impl Plugin for MyClientMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (movement_client
                .run_if(in_state(InGameUnpaused).and_then(not(is_host_server)))
                .in_set(FixedSet::Main),),
        );
    }
}

fn movement_client(
    mut player_body_controller: Query<
        (
            &Transform,
            &MovementAcceleration,
            &MaxMovementSpeed,
            &JumpImpulse,
            &mut LinearVelocity,
            &mut Rotation,
            &mut PhysicalPlayerBodyMarker,
            &ActionState<PlayerActions>,
            &InputBuffer<PlayerActions>,
            Has<Grounded>,
            &Children,
        ),
        (
            With<PlayerId>,
            With<Predicted>,
            Without<PhysicalPlayerHeadMarker>,
        ),
    >,
    mut player_head_query: Query<
        (Entity, &mut Transform, &mut PhysicalPlayerHeadMarker),
        (Without<PhysicalPlayerBodyMarker>, With<Predicted>),
    >,
    tick_manager: Res<TickManager>,
    rollback: Option<Res<Rollback>>,
) {
    // max number of stale inputs to predict before default inputs used
    const MAX_STALE_TICKS: u16 = 6;
    // get the tick, even if during rollback
    let tick = rollback
        .as_ref()
        .map(|rb| tick_manager.tick_or_rollback_tick(rb))
        .unwrap_or(tick_manager.tick());

    for (
        transform,
        movement_acceleration,
        max_speed,
        jump_impulse,
        mut body_velocity,
        mut body_rotation,
        mut phyiscal_body,
        action_state,
        input_buffer,
        is_grounded,
        children,
    ) in &mut player_body_controller
    {
        let (head_entity, _, _) = children
            .iter()
            .map(|entity| player_head_query.get(*entity).ok())
            .flatten()
            .next()
            .unwrap();
        let (_, mut head_transform, mut head) = player_head_query.get_mut(head_entity).unwrap();

        if input_buffer.get(tick).is_some() {
            shared_movement(
                transform,
                jump_impulse,
                movement_acceleration,
                max_speed,
                &mut body_velocity,
                &mut body_rotation,
                &mut phyiscal_body,
                &mut head_transform,
                &mut head,
                is_grounded,
                action_state,
            );
            continue;
        }

        if let Some((prev_tick, prev_input)) = input_buffer.get_last_with_tick() {
            let staleness = (tick - prev_tick).max(0) as u16;
            if staleness > MAX_STALE_TICKS {
                // input too stale, apply default input (ie, nothing pressed)
                shared_movement(
                    transform,
                    jump_impulse,
                    movement_acceleration,
                    max_speed,
                    &mut body_velocity,
                    &mut body_rotation,
                    &mut phyiscal_body,
                    &mut head_transform,
                    &mut head,
                    is_grounded,
                    action_state,
                );
            } else {
                // apply a stale input within our acceptable threshold.
                // we could use the staleness to decay movement forces as desired.
                shared_movement(
                    transform,
                    jump_impulse,
                    movement_acceleration,
                    max_speed,
                    &mut body_velocity,
                    &mut body_rotation,
                    &mut phyiscal_body,
                    &mut head_transform,
                    &mut head,
                    is_grounded,
                    prev_input,
                );
            }
        } else {
            // no inputs in the buffer yet, can happen during initial connection.
            // apply the default input (ie, nothing pressed)
            shared_movement(
                transform,
                jump_impulse,
                movement_acceleration,
                max_speed,
                &mut body_velocity,
                &mut body_rotation,
                &mut phyiscal_body,
                &mut head_transform,
                &mut head,
                is_grounded,
                action_state,
            );
        }
    }
}
