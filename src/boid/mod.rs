use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{transform::Transform, math::Vector2},
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use rand::random;

pub struct Boid {
    pub pos: Vector2<f32>,
    pub direction: f32,
    pub velocity: f32,
}

impl Component for Boid {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_boid(world: &mut World, pos: Vector2<f32>) {
    let mut trans = Transform::default();

    trans.set_translation_xyz(pos[0], pos[1], 0.0);

    let direction = random::<f32>();
    let velocity = random::<f32>();

    world
        .create_entity()
        .with(Boid {pos, direction, velocity})
        .with(trans)
        .build();
}