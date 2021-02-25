use super::super::components::boid::BoidSpecies;
use amethyst::{
    assets::AssetLoaderSystemData,
    config::Config,
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
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct BoidSpeciesConfiguration {
    pub species: BoidSpecies,
    pub vision_angle: f32, // angle that it can detect other entites (from straight i.ei full view = 180 degrees)
    pub vision_radius: f32, // range it can detect other entities
    pub max_altitude: f32, // maximum flight altitude
    pub max_turn_angle: f32, // maximum angle it can change per second
    pub max_pitch_turn_angle: f32, // maximum angle it can change pitch per second
    pub max_pitch: f32,    // maximum lift angle
    pub min_pitch: f32,    // minimum lift angle
    pub flap_vector: Vector3<f32>, // angle a flap applies force
    pub flap_magnitude: f32, // force a flap applies
    pub flap_cooldown: f32, // cooldown from flap till another can be performed
    pub glide_decay: f32,  // altitude lost per second while gliding
    pub max_hunger: f32,   // maximum hunger, hunger = 0 makes seek food immediately
    pub flap_hunger_reduction: f32, // amount hunger reduced when flapping
    pub sleep_time: f32,   // time spent sleeping when caught in a sleep trap
    pub step_distance: f32, // distance per step
    pub step_cooldown: f32, // cooldown from step till another can be performed
}
impl Default for BoidSpeciesConfiguration {
    fn default() -> Self {
        BoidSpeciesConfiguration {
            species: BoidSpecies::Test,
            vision_angle: 3.14159,
            vision_radius: 1.0,
            max_altitude: 1.0,
            max_turn_angle: 10.0,
            max_pitch_turn_angle: 30.0,
            max_pitch: 1.39626,  // 80 degrees
            min_pitch: -1.39626, // -80 degrees
            flap_vector: Vector3::new(0.0, 1.5707963, 0.0),
            flap_magnitude: 1.0,
            flap_cooldown: 0.5,
            glide_decay: 0.05,
            max_hunger: 10.0,
            flap_hunger_reduction: 0.05,
            sleep_time: 5.0,
            step_distance: 0.5,
            step_cooldown: 0.5,
        }
    }
}
