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
use super::wing::*;
use super::player_control::PlayerControl;
use super::super::utilities::*;

pub struct Boid;
impl Component for Boid {
    type Storage = NullStorage<Self>;
}

pub fn initialise_boid(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    initialise_boid(world, sprite_sheet_handle, Velocity {0.0}, Vector3::new(100.0, 100.0, 0.0), 0.0)
}

pub fn initialise_boid(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    velocity: Velocity
    position: Vector3<f32>
    direction: f32,
) {
    setup_boid(world, sprite_sheet_handle,velocity,position,direction).build();
}

pub fn initialise_player_boid(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    initialise_player_boid(world, sprite_sheet_handle, Velocity {0.0}, Vector3::new(100.0, 100.0, 0.0), 0.0)
}

pub fn initialise_player_boid(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    velocity: Velocity
    position: Vector3<f32>
    direction: f32,
) {
    setup_boid(world, sprite_sheet_handle,velocity,position,direction).with(PlayerControl {}).build();
}

fn setup_boid (
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    velocity: Velocity
    position: Vector3<f32>
    direction: f32,
) {
    let mut trans = Transform::default();
    trans
        .set_translation_xyz(position[0],position[1],position[2])
        .set_rotation_2d(direction)
        .set_scale(scale_from_height(position[2]));

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(trans)
        .with(velocity)
        .with(Wing {WingFlapStage::Up, 0.0})
        .with(Boid {})
}