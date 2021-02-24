use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, timing::Time, transform::{Transform, TransformBundle}},
    input::InputBundle,
    ecs::{Component, VecStorage,DispatcherBuilder,NullStorage, World},
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

pub struct Velocity {
    velocity: f32
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
