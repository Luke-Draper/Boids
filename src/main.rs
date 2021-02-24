mod boid;
use crate::boid::*;
mod window;
use crate::window::*;
mod components;
mod utilities;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, timing::Time, transform::{Transform, TransformBundle}},
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

pub struct Player;

impl SimpleState for Player {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Boid>();

        initialise_boid(world, Vector3::new(500.0, 500.0, 0), sprite_sheet_handle);
        initialise_camera(world);
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/bird_3_robin.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/bird_3_robin.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let display_config_path = config_path.join("display.ron");
    let binding_path = config_path.join("bindings.ron");
    let assets_dir = app_root.join("assets");

    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]), // background color
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(assets_dir, Player, game_data)?;
    game.run();

    Ok(())
}
