use super::player_control::PlayerControl;
use super::velocity::Velocity;
use amethyst::{
    assets::AssetLoaderSystemData,
    core::{math::Vector3, transform::Transform},
    ecs::{prelude::EntityBuilder, Component, VecStorage, World},
    prelude::*,
    renderer::{
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Material, MaterialDefaults, Mesh,
    },
};
use serde::{Deserialize, Serialize};

pub enum WingFlapStage {
    Up,
    Middle,
    Down,
    Ground,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BoidSpecies {
    Test,
    Sparrow,
    Robin,
    Cardinal,
    Bluejay,
    Eagle,
    Duck,
    Goose,
    Falcon,
}

// states: ground : stepping, standing, landing, taking off, eating, sleeping, captured
//         flying : flapping, diving, gliding : attack
pub struct Boid {
    //    pub species: BoidSpecies,
    pub flap_stage: WingFlapStage,
    pub flap_time: f64, // Time.absolute_time_seconds() // time since game start adjusted for game speed
    pub step_time: f64, // Time.absolute_time_seconds() // time since game start adjusted for game speed
    pub hunger: f32,
    pub flock_id: u8, // separation is applied regardless of flock id but cohesion and alignment are only applied to the same flock
}
impl Component for Boid {
    type Storage = VecStorage<Self>;
}

pub fn initialize_boid_default(world: &mut World) {
    initialize_boid(
        world,
        Vector3::new(0.0, 0.0, 0.0),
        Velocity {
            velocity: 0.0,
            direction: Vector3::new(0.0, 0.0, 0.0),
        },
        0.0,
        0.0,
    )
}

pub fn initialize_boid(
    world: &mut World,
    position: Vector3<f32>,
    velocity: Velocity,
    direction: f32,
    pitch: f32,
) {
    setup_boid(world, position, velocity, direction, pitch).build();
}

pub fn initialize_player_boid_default(world: &mut World) {
    initialize_player_boid(
        world,
        Vector3::new(0.0, 0.0, 0.0),
        Velocity {
            velocity: 0.0,
            direction: Vector3::new(0.0, 0.0, 0.0),
        },
        0.0,
        0.0,
    )
}

pub fn initialize_player_boid(
    world: &mut World,
    position: Vector3<f32>,
    velocity: Velocity,
    direction: f32,
    pitch: f32,
) {
    setup_boid(world, position, velocity, direction, pitch)
        .with(PlayerControl {})
        .build();
}

fn setup_boid(
    world: &mut World,
    position: Vector3<f32>,
    velocity: Velocity,
    direction: f32,
    pitch: f32,
) -> EntityBuilder<'_> {
    let mut trans = Transform::default();
    trans
        .set_translation_xyz(position[0], position[1], position[2])
        .set_rotation_euler(direction, pitch, 0.0);

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cone(100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                ..material_defaults
            },
            (),
        )
    });

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(trans)
        .with(velocity)
        .with(Boid {
            flap_stage: WingFlapStage::Up,
            flap_time: 0.0,
            step_time: 0.0,
            hunger: 0.0,
            flock_id: 0,
        })
}
