extern crate amethyst;

mod pong;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle},
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    Application, GameDataBuilder,
};

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("binding.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let assets_dir = app_root.join("assets/");
    let mut game = Application::new(assets_dir, Pong::default(), game_data).unwrap();
    game.run();

    Ok(())
}
