use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, timing::Time, transform::{Transform, TransformBundle}},
    input::InputBundle,
    ecs::{Component, DenseVecStorage,DispatcherBuilder,NullStorage, World},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        rendy::hal::command::ClearColor,sprite::Sprites,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    ui::{RenderUi, UiBundle,Anchor, LineMode, UiText, UiTransform},
    utils::application_root_dir,
    error::Error,
};
use super::velocity::Velocity;
use super::player_control::PlayerControl;

pub enum WingFlapStage {
    Up,
    Middle,
    Down,
    Ground
}
pub enum BoidSpecies {
    Sparrow,
    Robin,
    Cardinal,
    Bluejay,
    Eagle,
    Duck,
    Goose,
    Falcon
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoidSpeciesStats {
    pub species: BoidSpecies,
    pub vision_angle: f32, // angle that it can detect other entites
    pub vision_radius: f32, // range it can detect other entities
    pub max_altitude: f32, // maximum flight altitude
    pub max_turn_angle: f32, // maximum angle it can change per second
    pub max_pitch_turn_angle: f32, // maximum angle it can change pitch per second
    pub max_pitch: f32, // maximum lift angle
    pub min_pitch: f32, // minimum lift angle
    pub flap_vector: f32, // angle a flap applies force
    pub flap_magnitude: f32, // force a flap applies
    pub flap_cooldown: f32, // cooldown from flap till another can be performed
    pub glide_decay: f32, // altitude lost per second while gliding
    pub max_hunger; f32, // maximum hunger, hunger = 0 makes seek food immediately
    pub flap_hunger_reduction; f32, // amount hunger reduced when flapping
    pub sleep_time; f32, // time spent sleeping when caught in a sleep trap
    pub step_distance; f32, // distance per step
    pub step_cooldown: f32, // cooldown from step till another can be performed
}

// states: ground : stepping, standing, landing, taking off, eating, sleeping, captured
//         flying : flapping, diving, gliding : attack
pub struct Boid {
    flap_stage: WingFlapStage,
    flap_time: f64, // Time.absolute_time_seconds() // time since game start adjusted for game speed
    step_time: f64, // Time.absolute_time_seconds() // time since game start adjusted for game speed
    hunger: f32,
    flock_id: u8, // separation is applied regardless of flock id but cohesion and alignment are only applied to the same flock
}
impl Component for Boid {
    type Storage = VecStorage<Self>;
}

pub fn initialize_boid(
    world: &mut World
) {
    initialize_boid(world, Vector3::new(100.0, 100.0, 0.0), Velocity {0.0, Vector3::new(0.0, 0.0, 0.0)}, 0.0, 0.0)
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

pub fn initialize_player_boid(
    world: &mut World
) {
    initialize_player_boid(world, Vector3::new(100.0, 100.0, 0.0), Velocity {0.0, Vector3::new(0.0, 0.0, 0.0)}, 0.0, 0.0)
}

pub fn initialize_player_boid(
    world: &mut World,
    position: Vector3<f32>,
    velocity: Velocity,
    direction: f32,
    pitch: f32,
) {
    setup_boid(world, position, velocity, direction, pitch).with(PlayerControl {}).build();
}

fn setup_boid (
    world: &mut World,
    position: Vector3<f32>,
    velocity: Velocity,
    direction: f32,
    pitch: f32,
) {
    let mut trans = Transform::default();
    trans
        .set_translation_xyz(position[0],position[1],position[2])
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
        .with(trans)
        .with(velocity)
        .with(Boid {WingFlapStage::Up, 0.0})
}