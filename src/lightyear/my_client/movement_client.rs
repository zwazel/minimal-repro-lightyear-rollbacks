use avian3d::{math::Vector, prelude::*};
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
        physics::{
            Grounded, JumpImpulse, MaxMovementSpeed, MaxSlopeAngle, MovementAcceleration,
            MovementDampingFactor,
        },
    },
    my_states::InGameUnpaused,
};

pub struct MyClientMovementPlugin;

impl Plugin for MyClientMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                movement_client
                    .run_if(in_state(InGameUnpaused).and_then(not(is_host_server)))
                    .in_set(FixedSet::Main),
                (update_grounded, apply_movement_damping).in_set(FixedSet::Physics),
            ),
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

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<PhysicalPlayerBodyMarker>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}
