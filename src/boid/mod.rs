
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::{Vector3, timing::Time, transform::{Transform, TransformBundle}},
    input::InputBundle,
    ecs::{Component, DenseVecStorage,DispatcherBuilder, World},
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
use rand::random;

pub struct Boid {
    pub pos: Vector3<f32>, // x left to right, y bottom to top, z scale up towards screen
    pub direction: f32,
    pub velocity: f32,
}

impl Component for Boid {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_boid(
    world: &mut World,
    pos: Vector3<f32>,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let mut trans = Transform::default();

    trans.set_translation_xyz(pos[0], pos[1], pos[2]);

    let direction = random::<f32>();
    let velocity = random::<f32>();

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Boid {
            pos,
            direction,
            velocity,
        })
        .with(trans)
        .build();
}
