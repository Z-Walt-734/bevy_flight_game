use bevy::prelude::shape as bevy_shape;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod player;
mod ship;

fn test_setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, -10.0, 500.0),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..Default::default()
    });
}

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(camera::setup_camera)
        .add_startup_system(player::spawn_player)
        .add_system(player::player_control_system)
        .add_system(ship::move_ship_system)
        .add_system(camera::move_camera_system)
        .add_system(test_setup)
        .run();
}

//ship
