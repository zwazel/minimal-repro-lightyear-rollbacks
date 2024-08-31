use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::{
    lib::{PhysicalPlayerBodyMarker, PhysicalPlayerHeadMarker, PlayerActions},
    physics::lib::{JumpImpulse, MaxMovementSpeed, MovementAcceleration},
};

pub fn shared_movement(
    transform: &Transform,
    jump_impulse: &JumpImpulse,
    movement_acceleration: &MovementAcceleration,
    max_speed: &MaxMovementSpeed,
    linear_velocity: &mut LinearVelocity,
    body_rotation: &mut Rotation,
    body: &mut PhysicalPlayerBodyMarker,
    head_transform: &mut Transform,
    head: &mut PhysicalPlayerHeadMarker,
    is_grounded: bool,
    action_state: &ActionState<PlayerActions>,
) {
    let forward_vector = transform.forward();
    let right_vector = transform.right();
    let axis_pair = action_state.axis_pair(&PlayerActions::Move);
    linear_velocity.x +=
        (axis_pair.x * right_vector.x + axis_pair.y * forward_vector.x) * movement_acceleration.0;
    linear_velocity.z +=
        (axis_pair.x * right_vector.z + axis_pair.y * forward_vector.z) * movement_acceleration.0;

    let max_speed = max_speed.0;
    let speed = linear_velocity.length();
    if speed > max_speed {
        linear_velocity.x *= max_speed / speed;
        linear_velocity.z *= max_speed / speed;
    }

    if is_grounded && action_state.just_pressed(&PlayerActions::Jump) {
        linear_velocity.y = jump_impulse.0;
    }

    let camera_vector = action_state.axis_pair(&PlayerActions::LookAround) * 0.3;
    let max_pitch: f32 = 89.9_f32.to_radians(); // Prevent flipping, slightly less than 90 degrees
    let min_pitch: f32 = -89.9_f32.to_radians(); // Slightly more than -90 degrees

    head.pitch = (head.pitch - camera_vector.y.to_radians())
        .min(max_pitch)
        .max(min_pitch);
    let head_rotation_quat = Quat::from_axis_angle(Vec3::X, head.pitch);

    body.yaw += -camera_vector.x.to_radians();
    let body_rotation_quat = Quat::from_axis_angle(Vec3::Y, body.yaw);

    head_transform.rotation = head_rotation_quat;
    body_rotation.0 = body_rotation_quat;
}
