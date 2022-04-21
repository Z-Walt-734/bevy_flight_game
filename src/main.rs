use bevy::prelude::shape as bevy_shape;
use bevy::prelude::*;
// use bevy_rapier3d::prelude::shape as rapier_shape;
// use bevy_rapier3d::prelude::*;

// rotation steps
// const TIME_STEP: f32 = 1.0 / 60.0;
// how "slippery" slow downs are
const DECELERATION_SLIDE: f32 = 1.2;
// how far ships bank when max turn
const MAX_BANK_ANGLE: f32 = 1.0;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct TranslationAcceleration {
    accelerations: Vec3,
}
// #[derive(Component)]
// struct RotationAcceleration {
//     accelerations: Quat,
// }
#[derive(Component)]
struct TransformAcceleration {
    translation: TranslationAcceleration,
    // rotation: RotationAcceleration,
}
#[derive(Component)]
struct IsManeuvering(bool);
#[derive(Component)]
struct ShipSpecs {
    max_speeds: Vec3,
    acceleration_rates: Vec3,
}

// Bundles
#[derive(Bundle)]
struct PlayerBundle {
    _p: Player,
    #[bundle]
    transform: (Transform, GlobalTransform),
    acceleration: TransformAcceleration,
    is_maneuvering: IsManeuvering,
    ship_specs: ShipSpecs,
}

#[derive(Bundle)]
struct PlayerBox {
    transform: Transform,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            _p: Player,
            transform: (
                Transform::from_xyz(0.0, 0.0, 1.0),
                GlobalTransform::identity(),
            ),
            acceleration: TransformAcceleration { ..default() },
            is_maneuvering: IsManeuvering(false),
            ship_specs: ShipSpecs { ..default() },
        }
    }
}

impl Default for TransformAcceleration {
    fn default() -> TransformAcceleration {
        TransformAcceleration {
            translation: TranslationAcceleration { ..default() },
            // rotation: RotationAcceleration { ..default() },
        }
    }
}

impl Default for TranslationAcceleration {
    fn default() -> TranslationAcceleration {
        TranslationAcceleration {
            accelerations: Vec3::ZERO,
        }
    }
}

// impl Default for RotationAcceleration {
//     fn default() -> RotationAcceleration {
//         RotationAcceleration {
//             accelerations: Quat::IDENTITY,
//         }
//     }
// }

impl Default for ShipSpecs {
    fn default() -> ShipSpecs {
        ShipSpecs {
            // TODO make these unique to each ship
            // 1.0 should probably be max
            max_speeds: Vec3::new(1.0, 1.0, 1.0),
            // point .02 to .05 seems to be a good range
            acceleration_rates: Vec3::new(0.03, 0.03, 0.03),
        }
    }
}

// Systems
fn player_control_system(
    input: Res<Input<KeyCode>>,
    // time: Res<Time>,
    mut query: Query<(&mut TransformAcceleration, &ShipSpecs), With<Player>>,
) {
    // let keys = input.get_pressed();
    let (mut transform_acceleration, ship_specs) = query.single_mut();
    let mut is_maneuvering = false;

    input.get_pressed().for_each(|k| match k {
        KeyCode::W => {
            is_maneuvering = true;

            if transform_acceleration.translation.accelerations.y > -ship_specs.max_speeds.y {
                transform_acceleration.translation.accelerations +=
                    Vec3::new(0.0, -ship_specs.acceleration_rates.y, 0.0);
            }
        }
        KeyCode::A => {
            is_maneuvering = true;

            if transform_acceleration.translation.accelerations.x < ship_specs.max_speeds.x {
                transform_acceleration.translation.accelerations +=
                    Vec3::new(ship_specs.acceleration_rates.x, 0.0, 0.0);
            }
        }
        KeyCode::S => {
            is_maneuvering = true;

            if transform_acceleration.translation.accelerations.y < ship_specs.max_speeds.y {
                transform_acceleration.translation.accelerations +=
                    Vec3::new(0.0, ship_specs.acceleration_rates.y, 0.0);
            }
        }
        KeyCode::D => {
            is_maneuvering = true;

            if transform_acceleration.translation.accelerations.x > -ship_specs.max_speeds.x {
                transform_acceleration.translation.accelerations +=
                    Vec3::new(-ship_specs.acceleration_rates.x, 0.0, 0.0);
            }
        }
        _ => {}
    });
    // Slow down if not accelerating
    if !is_maneuvering {
        let x = transform_acceleration.translation.accelerations.x;
        let y = transform_acceleration.translation.accelerations.y;

        if x != 0.0 {
            transform_acceleration.translation.accelerations.x = x / DECELERATION_SLIDE;
        }
        if y != 0.0 {
            transform_acceleration.translation.accelerations.y = y / DECELERATION_SLIDE;
        }
    }
}

fn move_ship_system(mut query: Query<(&mut Transform, &TransformAcceleration, &ShipSpecs)>) {
    for (mut transform, transform_acceleration, ship_specs) in query.iter_mut() {
        transform.translation += transform_acceleration.translation.accelerations;

        let rot_z = (transform_acceleration.translation.accelerations.x / ship_specs.max_speeds.x)
            * -MAX_BANK_ANGLE;
        let rot_x = (transform_acceleration.translation.accelerations.y / ship_specs.max_speeds.y)
            * -MAX_BANK_ANGLE;
        let rotation_percent = Quat::from_euler(EulerRot::XYZ, rot_x, 0.0, rot_z);
        transform.rotation = rotation_percent;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ship: Handle<Scene> = asset_server.load("ships/Pancake/glTF/Pancake.gltf#Scene0");
    commands
        .spawn_bundle(PlayerBundle {
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn_scene(ship);
        });
    // In place in prep for collision camera follower
    // .with_children(|p| {
    //     p.spawn_bundle(PerspectiveCameraBundle {
    //         transform: Transform::from_xyz(0.0, 8.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     });
    // });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 8.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 50.0 })),
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
        .add_system(player_control_system)
        // .add_system(move_camera_system)
        .add_system(move_ship_system)
        .run();
}
