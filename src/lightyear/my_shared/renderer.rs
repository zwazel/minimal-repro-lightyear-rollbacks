use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::client::*;

pub(crate) struct MyRendererPlugin;

impl Plugin for MyRendererPlugin {
    fn build(&self, app: &mut App) {
        // app .add_plugins((
        //         VisualInterpolationPlugin::<Position>::default(),
        //         VisualInterpolationPlugin::<Rotation>::default(),
        //     ))
        //     .observe(add_visual_interpolation_components::<Position>)
        //     .observe(add_visual_interpolation_components::<Rotation>);
    }
}

// Non-wall entities get some visual interpolation by adding the lightyear
// VisualInterpolateStatus component
//
// We query Without<Confirmed> instead of With<Predicted> so that the server's gui will
// also get some visual interpolation. But we're usually just concerned that the client's
// Predicted entities get the interpolation treatment.
//
// We must trigger change detection so that the SyncPlugin will detect and sync changes
// from Position/Rotation to Transform.
//
// Without syncing interpolated pos/rot to transform, things like sprites, meshes, and text which
// render based on the *Transform* component (not avian's Position) will be stuttery.
//
// (Note also that we've configured avian's SyncPlugin to run in PostUpdate)
fn add_visual_interpolation_components<T: Component>(
    trigger: Trigger<OnAdd, T>,
    q: Query<Entity, (With<T>, With<Predicted>)>,
    mut commands: Commands,
) {
    if !q.contains(trigger.entity()) {
        return;
    }
    debug!("Adding visual interp component to {:?}", trigger.entity());
    commands
        .entity(trigger.entity())
        .insert(VisualInterpolateStatus::<T> {
            trigger_change_detection: true,
            ..default()
        });
}
