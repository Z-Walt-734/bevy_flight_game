use bevy::prelude::*;

const MAX_BANK_ANGLE: f32 = 0.5;

// Has
#[derive(Component)]
pub struct MaxSpeeds {
    pub max_speeds: Vec3,
}
impl Default for MaxSpeeds {
    fn default() -> MaxSpeeds {
        MaxSpeeds {
            max_speeds: Vec3::new(1.0, 1.0, 0.5),
        }
    }
}

#[derive(Component)]
pub struct Accelerations {
    pub accelerations: Vec3,
}
impl Default for Accelerations {
    fn default() -> Accelerations {
        Accelerations {
            accelerations: Vec3::new(0.03, 0.03, 0.03),
        }
    }
}

#[derive(Component)]
pub struct CurrentSpeeds {
    pub current_speeds: Vec3,
}
impl Default for CurrentSpeeds {
    fn default() -> CurrentSpeeds {
        CurrentSpeeds {
            current_speeds: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
// NOTE Placeholder code

// #[derive(Component)]
// pub struct ShipId {
//     name: String,
// }
// impl Default for ShipId {
//     fn default() -> ShipId {
//         ShipId {
//             name: "pancake".to_string(),
//         }
//     }
// }
// #[derive(Component)]
// pub struct Skins {
//     ship_id: ShipId,
//     collider: Collider,
//     rigid_body: RigidBody,
// }
// impl Default for Skins {
//     fn default() -> Skins {
//         Skins {
//             ship_id: ShipId {
//                 ..Default::default()
//             },
//             collider: Collider::ball(1.0),
//             rigid_body: RigidBody::Dynamic,
//         }
//     }
// }

// fn spawn_ship(asset_server: Res<AssetServer>) {
//     let ship: Handle<Scene> = asset_server.load("ships/Pancake/glTF/Pancake.gltf#Scene0");
// }

// #[derive(Component)]
// pub struct WeaponDamage {
//     id: String,
//     energy: f32,
//     physical: f32,
// }
// #[derive(Component)]
// pub enum WeaponType {
//     Energy,
//     Physical,
// }
// #[derive(Component)]
// pub struct Shield {
//     id: String,
//     energy: f32,
//     physical: f32,
// }

// Bundles
#[derive(Bundle)]
pub struct ShipBundle {
    pub max_speeds: MaxSpeeds,
    pub accelerations: Accelerations,
    pub current_movements: CurrentSpeeds,
    // pub skins: Skins,
    #[bundle]
    pub position: TransformBundle,
    // pub scene_instance: SceneInstance,
}

// Recomended not use default
impl Default for ShipBundle {
    fn default() -> ShipBundle {
        ShipBundle {
            max_speeds: MaxSpeeds {
                ..Default::default()
            },
            accelerations: Accelerations {
                ..Default::default()
            },
            current_movements: CurrentSpeeds {
                ..Default::default()
            },
            // skins: Skins {
            //     ..Default::default()
            // },
            position:TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0))
            // scene_instance: SceneInstance {
            //     ..Default::default()
            // },
        }
    }
}

pub fn move_ship_system(mut query: Query<(&CurrentSpeeds, &MaxSpeeds, &mut Transform)>) {
    for (current_speeds, max_speeds, mut transform) in query.iter_mut() {
        transform.translation += current_speeds.current_speeds;
        // info!("Moving check {:?}", transform.translation);
        let rot_z = (current_speeds.current_speeds.x / max_speeds.max_speeds.x) * -MAX_BANK_ANGLE;
        let rot_x = (current_speeds.current_speeds.y / max_speeds.max_speeds.y) * -MAX_BANK_ANGLE;
        let rotation_percent = Quat::from_euler(EulerRot::XYZ, rot_x, 0.0, rot_z);
        transform.rotation = rotation_percent;
    }
}
