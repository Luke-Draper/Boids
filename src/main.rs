mod components;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, AssetLoaderSystemData},
    core::{math::Vector3, timing::Time, transform::{Transform, TransformBundle}},
    input::{InputBundle, StringBindings},
    ecs::{Component, DenseVecStorage,DispatcherBuilder, World},
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        types::DefaultBackend,
        rendy::{mesh::*, util::types::vertex::PosNormTangTex},
        shape::Shape,
        light::{Light, PointLight},
        palette::rgb::Rgb,
        Mesh, Material, MaterialDefaults,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    ui::{RenderUi, UiBundle,Anchor, LineMode, UiText, UiTransform},
    utils::application_root_dir,
    error::Error,
};

pub struct GameBegin;

impl SimpleState for GameBegin {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_camera(world);
        initialize_sphere(world);
        initialize_light(world);
    }
}

pub fn initialize_camera(world: &mut World) {
    let mut trans = Transform::default();
    trans.set_translation_xyz(0.0, 0.0, 10.0);

    world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(trans)
        .build();
}

pub fn initialize_sphere(world: &mut World) {
    
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

    let mut trans = Transform::default();
    trans.set_translation_xyz(0.0, 0.0, 0.0).set_rotation_euler(1.0, 2.0, 3.0);

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(trans)
        .build();
}

fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world
        .create_entity()
        .with(light)
        .with(transform)
        .build();
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
    amethyst::Logger::from_config(Default::default())
    //.level_for("gfx_glyph", amethyst::LogLevelFilter::max())
    .start();

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let display_config_path = config_path.join("display.ron");
    let binding_path = config_path.join("bindings.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new()
            .with_bindings_from_file(binding_path)?)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 1.0, 1.0]), // background color
                )
                .with_plugin(RenderPbr3D::default()),
        )?;

    let mut game = Application::new(assets_dir, GameBegin, game_data)?;
    game.run();

    Ok(())
}
