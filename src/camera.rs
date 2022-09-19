use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::ship;

#[derive(Component)]
pub struct CameraTracker;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert(Collider::capsule_x(2.0, 6.0))
        // .insert(RigidBody::Dynamic)
        .insert(Sensor::default())
        // .insert(ActiveEvents::COLLISION_EVENTS)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|p| {
            p.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 8.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            });
        })
        .insert(
            CameraTracker);
}

pub fn move_camera_system(
    player_query: Query<
        (&Transform, &ship::MaxSpeeds),
        (With<crate::player::Player>, Without<CameraTracker>),
    >,
    mut cam_query: Query<(&mut Transform, Entity), With<CameraTracker>>,
    narrow_phase: Res<RapierContext>,
) {
    let mut cam_query = cam_query.single_mut();
    let player_query = player_query.single();

    // let mut arena = Arena::new();
    // let idx = arena.insert(cam_query.1);

    // let entity: ColliderHandle = ColliderHandle(cam_query.1);
    let mut intersecting = false;
    for (_, _, not_intersecting) in narrow_phase.intersections_with(cam_query.1) {
        intersecting = !not_intersecting;

        // if !intersecting {
        //     cam_query.0.translation += player_query.current_speeds;
        //     info!("moving: {:?}", intersecting);
        // }
    }
    // info!("{:?}", player_query.0.translation);
    if !intersecting {
        let mut move_distance = player_query.0.translation - cam_query.0.translation; // (player_query.translation - cam_query.0.translation) * Vec3::new(2.0, 2.0, 2.0);
        move_distance.z = player_query.1.max_speeds.z;
        let mut reduction_factor = 25.0;
        if move_distance.x.abs() > 10.0 || move_distance.y.abs() > 5.0 {
            reduction_factor = 10.0;
        }

        cam_query.0.translation +=
            move_distance / Vec3::new(reduction_factor * 2.0, reduction_factor, 1.0);
    }
}
