use std::{
    f32::consts::PI,
    ops::{Add, Mul},
    time::Duration,
};

use avian3d::{
    math::{Quaternion, Scalar, Vector},
    prelude::*,
};
use leafwing_input_manager::prelude::*;
use lightyear::prelude::{client::Replicate as ClientReplicate, *};

use bevy::{ecs::entity::MapEntities, prelude::*};
use serde::{Deserialize, Serialize};

use super::physics::lib::{
    JumpImpulse, MaxMovementSpeed, MaxSlopeAngle, MovementAcceleration, MovementDampingFactor,
};

pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

pub const PLAYER_REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    Main,
    // apply physics steps
    Physics,
}

#[derive(Channel)]
pub struct Channel1;

#[derive(Component, Reflect, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Deref)]
#[reflect(Component)]
pub struct PlayerId(pub ClientId);

impl Default for PlayerId {
    fn default() -> Self {
        Self(ClientId::Local(0))
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, PartialEq, Debug, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct PhysicalPlayerBodyMarker {
    pub head_entity: Option<Entity>,
    #[deref]
    pub yaw: Scalar,
}

impl Default for PhysicalPlayerBodyMarker {
    fn default() -> Self {
        Self {
            head_entity: None,
            yaw: 0.0,
        }
    }
}

impl MapEntities for PhysicalPlayerBodyMarker {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.head_entity = self
            .head_entity
            .map(|entity| entity_mapper.map_entity(entity));
    }
}

impl Mul<f32> for &PhysicalPlayerBodyMarker {
    type Output = PhysicalPlayerBodyMarker;

    fn mul(self, rhs: Scalar) -> Self::Output {
        PhysicalPlayerBodyMarker {
            head_entity: self.head_entity,
            yaw: self.yaw * rhs,
        }
    }
}

impl Add for PhysicalPlayerBodyMarker {
    type Output = PhysicalPlayerBodyMarker;

    fn add(self, rhs: Self) -> Self::Output {
        PhysicalPlayerBodyMarker {
            head_entity: self.head_entity,
            yaw: self.yaw + rhs.yaw,
        }
    }
}

#[derive(
    Component, Reflect, Default, Serialize, Deserialize, PartialEq, Debug, Clone, Deref, DerefMut,
)]
#[reflect(Component)]
pub struct PhysicalPlayerHeadMarker {
    pub pitch: Scalar,
}

impl Mul<f32> for &PhysicalPlayerHeadMarker {
    type Output = PhysicalPlayerHeadMarker;

    fn mul(self, rhs: Scalar) -> Self::Output {
        PhysicalPlayerHeadMarker {
            pitch: self.pitch * rhs,
        }
    }
}

impl Add for PhysicalPlayerHeadMarker {
    type Output = PhysicalPlayerHeadMarker;

    fn add(self, rhs: Self) -> Self::Output {
        PhysicalPlayerHeadMarker {
            pitch: self.pitch + rhs.pitch,
        }
    }
}

#[derive(Bundle)]
pub(crate) struct PhysicalPlayerHeadBundle {
    name: Name,
    player_marker: PhysicalPlayerHeadMarker,
    player_id: PlayerId,
    spatial: SpatialBundle,
}

impl PhysicalPlayerHeadBundle {
    pub(crate) fn new(player_id: ClientId) -> Self {
        Self {
            // use player id in name
            name: Name::new(format!("PhysicalPlayerHead-{}", player_id)),
            player_id: PlayerId(player_id),
            player_marker: PhysicalPlayerHeadMarker::default(),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 2.0, 0.0)),
        }
    }
}

#[derive(Bundle)]
pub(crate) struct PhysicalPlayerBodyBundle {
    name: Name,
    player_marker: PhysicalPlayerBodyMarker,
    physics: PhysicsBundle,
    ground_caster: ShapeCaster,
    inputs: InputManagerBundle<PlayerActions>,
    pre_predicted: PrePredicted,
    replicate: ClientReplicate,
    player_id: PlayerId,
}

impl PhysicalPlayerBodyBundle {
    pub(crate) fn new(
        input_map: InputMap<PlayerActions>,
        collider: Collider,
        player_id: ClientId,
    ) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            name: Name::new(format!("PhysicalPlayerBody-{}", player_id)),
            player_id: PlayerId(player_id),
            player_marker: PhysicalPlayerBodyMarker::default(),
            physics: PhysicsBundle::player(),
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            inputs: InputManagerBundle::<PlayerActions> {
                action_state: ActionState::default(),
                input_map,
            },
            pre_predicted: PrePredicted::default(),
            replicate: ClientReplicate {
                group: PLAYER_REPLICATION_GROUP,
                ..default()
            },
        }
    }

    pub(crate) fn with_head(mut self, head: Entity) -> Self {
        self.player_marker.head_entity = Some(head);
        self
    }
}

#[derive(Bundle)]
pub(crate) struct PhysicsBundle {
    pub(crate) collider: Collider,
    pub(crate) collider_density: ColliderDensity,
    pub(crate) rigid_body: RigidBody,
    pub(crate) locked_axes: LockedAxes,
    pub(crate) movement: CharacterMovementBundle,
}

impl PhysicsBundle {
    pub(crate) fn player() -> Self {
        Self {
            collider: Collider::capsule(0.4, 1.0),
            collider_density: ColliderDensity(1.0),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: CharacterMovementBundle::default(),
        }
    }
}

#[derive(Bundle)]
pub(crate) struct CharacterMovementBundle {
    acceleration: MovementAcceleration,
    max_speed: MaxMovementSpeed,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl CharacterMovementBundle {
    pub(crate) fn new(
        acceleration: Scalar,
        max_speed: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            max_speed: MaxMovementSpeed(max_speed),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for CharacterMovementBundle {
    fn default() -> Self {
        Self::new(1.0, 10.0, 0.9, 7.0, PI * 0.45)
    }
}

// Inputs
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect)]
pub enum PlayerActions {
    Move,
    LookAround,
    Jump,
}

impl Actionlike for PlayerActions {
    fn input_control_kind(&self) -> leafwing_input_manager::InputControlKind {
        match self {
            PlayerActions::Move | PlayerActions::LookAround => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}
