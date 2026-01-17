use bevy::prelude::*;
use bevy::window::{WindowMode, WindowPlugin};

use crate::game::setup_orchestrator;

mod ui;
mod galaxy;
mod assets;
mod game;

pub fn main() -> Result<(), String>{

    let mut app = App::new();
    app
    .add_plugins((
            // Full screen
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Index(0)),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        ))
    .add_message::<game::GameEvent>()
    .add_systems(PreStartup, assets::load_assets)
    .add_systems(Startup, (game:: setup_orchestrator, galaxy::setup.after(setup_orchestrator), ui::setup_ui))
    .add_systems(Update, ui::button_hover)
    .add_systems(FixedUpdate, (game::game_loop, galaxy::draw_topology));
    app.run();
    Ok(())
}
