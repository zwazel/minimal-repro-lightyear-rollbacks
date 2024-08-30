use avian3d::math::{Scalar, Vector};
use bevy::prelude::*;

/// A marker component indicating that an entity is on the ground.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub(crate) struct Grounded;

/// The acceleration used for character movement.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct MovementAcceleration(pub(crate) Scalar);

/// The acceleration used for character movement.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct MaxMovementSpeed(pub(crate) Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct MovementDampingFactor(pub(crate) Scalar);

/// The strength of a jump.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct JumpImpulse(pub(crate) Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct ControllerGravity(pub(crate) Vector);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct MaxSlopeAngle(pub(crate) Scalar);
