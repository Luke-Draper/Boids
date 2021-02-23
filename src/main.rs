mod boid;
use crate::boid::*;
mod window;
use crate::window::*;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::math::Vector2,
    utils::application_root_dir,
};

pub struct Player;

impl SimpleState for Player {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Boid>();
    
        initialise_boid(world, Vector2::new(0.5,0.5));
        initialise_camera(world);
    }
}


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let display_config_path = config_path.join("display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
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
