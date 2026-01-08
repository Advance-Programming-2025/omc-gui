use bevy::prelude::*;
use bevy::window::{WindowMode, WindowPlugin};

use omc_galaxy::Orchestrator;
use std::env;

mod ui;
mod galaxy;
mod assets;

pub fn main() -> Result<(), String>{

    // Load env
    dotenv::dotenv().ok();
    //Init and check orchestrator
    let mut orchestrator = Orchestrator::new()?;

    //Give the absolute path for the init file
    let file_path = env::var("INPUT_FILE")
        .expect("Imposta INPUT_FILE nel file .env o come variabile d'ambiente");

    orchestrator.initialize_galaxy_by_file(file_path.as_str().trim())?;

    let topology = orchestrator.get_topology();

    let mut app = App::new();
    app
    .insert_resource(galaxy::GalaxyTopologyResource{topology})
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
    .add_systems(PreStartup, assets::load_assets)
    .add_systems(Startup, (galaxy::setup, galaxy::draw_topology.after(galaxy::setup), ui::setup_ui))
    .add_systems(FixedUpdate, ui::button_hover);
    app.run();
    Ok(())
}
