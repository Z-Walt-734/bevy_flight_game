use bevy::prelude::*;
use std::ops::Add;

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct FlightControls;

// fn flight(time: Res<Time>, mut query: Query<&mut Transform, With<FlightControls>>) {
//     for mut transform in query.iter_mut() {
//         transform.rotation *= Quat::from_rotation_x(1.1 * time.delta_seconds());
//         transform.rotation *= Quat::from_rotation_y(1.2 * time.delta_seconds());
//         transform.rotation *= Quat::from_rotation_z(1.3 * time.delta_seconds());
//     }
// }

fn controls(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<FlightControls>>,
) {
    let mut transform = query.single_mut();
    let mut x_rotation = 0.0;
    let mut y_rotation = 0.0;

    if input.pressed(KeyCode::W) {
        //pull up
        // delta_transform = delta_transform + Quat::from_rotation_x(-1.0 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_x(-1.0 * time.delta_seconds());
        info!("Push Down");
    }
    if input.pressed(KeyCode::S) {
        //pull up
        // delta_transform = delta_transform + Quat::from_rotation_x(1.0 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_x(1.0 * time.delta_seconds());
        info!("Pull Up");
    }
    if input.pressed(KeyCode::A) {
        //pull up
        // delta_transform = delta_transform + Quat::from_rotation_y(1.0 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_y(1.0 * time.delta_seconds());
        info!("Bank Left");
    }
    if input.pressed(KeyCode::D) {
        //pull up
        // delta_transform = delta_transform + Quat::from_rotation_y(-1.0 * time.delta_seconds())
        transform.rotation *= Quat::from_rotation_y(-1.0 * time.delta_seconds());
        info!("Bank Right");
    }

    // transform.rotate(Quat::from_rotation_x(x_rotation * TIME_STEP));
    // transform.rotate(Quat::from_rotation_y(y_rotation * TIME_STEP));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ship: Handle<Scene> =
        asset_server.load("LowPolySpaceshipPack/Models/GLTF/LPSP_LuxuryShip.gltf#Scene0");
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 0.0, 1.0),
            GlobalTransform::identity(),
        ))
        .with_children(|p| {
            p.spawn_scene(ship);
        })
        .insert(FlightControls)
        .with_children(|p| {
            p.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(0.0, 8.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, -10.0, 0.0),
        ..default()
    });
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(flight)
        .add_system(controls)
        .run();
}
