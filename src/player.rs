use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::ship::{Accelerations, CurrentSpeeds, MaxSpeeds, ShipBundle};
pub const DECELERATION_SLIDE: f32 = 1.2;
const PLAYER_TEST_CHOICE: &str = "Pancake";

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_scene_path = format!(
        "ships/Pancake/glTF/{ship}.gltf#Scene0",
        ship = PLAYER_TEST_CHOICE
    );
    let ship_obj_file = format!(
        "./assets/ships/{ship}/OBJ/{ship}.obj",
        ship = PLAYER_TEST_CHOICE
    );
    let ship: Handle<Scene> = asset_server.load(&ship_scene_path);
    let ship_file = BufReader::new(File::open(ship_obj_file).unwrap());

    let mut ship_vertices: Vec<Vec3> = vec![]; //Vec3::new(0.0, 0.0, 0.0);
    let mut ship_indices: Vec<[u32; 3]> = vec![];

    ship_file.lines().for_each(|line| {
        if let Ok(l) = line {
            match (l.chars().nth(0), l.chars().nth(1)) {
                (Some('v'), Some(' ')) => {
                    let data: Vec<&str> = l.split(" ").collect();
                    ship_vertices.push(Vec3::new(
                        data[1].parse::<f32>().unwrap(),
                        data[2].parse::<f32>().unwrap(),
                        data[3].parse::<f32>().unwrap(),
                    ))
                }
                (Some('v'), Some('n')) => {
                    let data: Vec<&str> = l.split(" ").collect();
                    ship_vertices.push(Vec3::new(
                        data[1].parse::<f32>().unwrap(),
                        data[2].parse::<f32>().unwrap(),
                        data[3].parse::<f32>().unwrap(),
                    ))
                }
                (Some('f'), Some(' ')) => {
                    let data: Vec<&str> = l.split(" ").collect();

                    for ind_set in &data[1..] {
                        let mut indices: [u32; 3] = [0, 0, 0];
                        let points: Vec<_> = ind_set.split('/').collect();
                        indices[0] = points[0].parse::<u32>().unwrap();
                        indices[1] = points[1].parse::<u32>().unwrap();
                        indices[2] = points[2].parse::<u32>().unwrap();
                        ship_indices.push(indices);
                    }
                }
                _ => {}
            }
        }
    });

    commands
        .spawn_bundle(ShipBundle {
            // position: Transform::from_xyz(0.0, -100.0, 0.0),
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn_scene(ship);
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::convex_hull(&ship_vertices).unwrap())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Player);
}

pub fn player_control_system(
    input: Res<Input<KeyCode>>,
    // time: Res<Time>,
    mut query: Query<(&mut CurrentSpeeds, &MaxSpeeds, &Accelerations), With<Player>>,
) {
    // let keys = input.get_pressed();
    let (mut current_speeds, max_speeds, accelerations) = query.single_mut();
    let mut is_maneuvering = false;

    input.get_pressed().for_each(|k| match k {
        KeyCode::W => {
            is_maneuvering = true;
            if current_speeds.current_speeds.y > -max_speeds.max_speeds.y {
                current_speeds.current_speeds +=
                    Vec3::new(0.0, -accelerations.accelerations.y, 0.0);
            }
        }
        KeyCode::A => {
            is_maneuvering = true;
            if current_speeds.current_speeds.x < max_speeds.max_speeds.x {
                current_speeds.current_speeds += Vec3::new(accelerations.accelerations.x, 0.0, 0.0);
            }
        }
        KeyCode::S => {
            is_maneuvering = true;
            if current_speeds.current_speeds.y < max_speeds.max_speeds.y {
                current_speeds.current_speeds += Vec3::new(0.0, accelerations.accelerations.y, 0.0);
            }
        }
        KeyCode::D => {
            is_maneuvering = true;
            if current_speeds.current_speeds.x > -max_speeds.max_speeds.x {
                current_speeds.current_speeds +=
                    Vec3::new(-accelerations.accelerations.x, 0.0, 0.0);
            }
        }
        _ => {}
    });
    // Slow down if not accelerating
    if !is_maneuvering {
        let x = current_speeds.current_speeds.x;
        let y = current_speeds.current_speeds.y;
        if x != 0.0 {
            current_speeds.current_speeds.x = x / DECELERATION_SLIDE;
        }
        if y != 0.0 {
            current_speeds.current_speeds.y = y / DECELERATION_SLIDE;
        }
    }
    current_speeds.current_speeds.z = max_speeds.max_speeds.z;
}
