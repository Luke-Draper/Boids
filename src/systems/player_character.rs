
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        math::Vector3,
        timing::Time,
        transform::{Transform, TransformBundle},
    },
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, DispatcherBuilder, Join, Read, ReadStorage, System, SystemData,
        World, WriteStorage,
    },
    error::Error,
    input::{InputBundle, InputHandler, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        sprite::Sprites,
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    ui::{Anchor, LineMode, RenderUi, UiBundle, UiText, UiTransform},
    utils::application_root_dir,
};
/*
#[derive(SystemDesc)]
pub struct BoidPlayerCharacterSystem;

impl<'s> System<'s> for BoidPlayerCharacterSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, PlayerControl>,
        ReadStorage<'s, Boid>,
        ReadStorage<'s, Wing>,
        ReadStorage<'s, Velocity>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (mut transforms, player_controlled, boids, wings, velocities, input): Self::SystemData,
    ) {
        for (transform, player_control, boid, wing, velocity) in (
            &mut transforms,
            &player_controlled,
            &boids,
            &wings,
            &velocities,
        )
            .join()
        {
            let x = input.axis_value("x").unwrap_or(0.0);
            let y = input.axis_value("y").unwrap_or(0.0);
            let mut angle_update = false;
            if (x > 0.001) || (x < -0.001) || (y > 0.001) || (y < -0.001) {
                angle_update = true;
            }
            let flap_key = input.action_is_down("flap").unwrap_or(false);

            if let Some((x, y)) = input.mouse_position() {}

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let side_name = match paddle.side {
                        Side::Left => "left",
                        Side::Right => "right",
                    };
                    println!("Side {:?} moving {}", side_name, mv_amount);
                }
            }
        }
    }
}
*/