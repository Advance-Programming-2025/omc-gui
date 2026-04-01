use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowPlugin};
use bevy_tweening::TweeningPlugin;

use crate::game::orchestrator::setup_orchestrator;

mod app;
mod ecs;
mod explorers;
mod galaxy;
mod game;
mod ui;
mod utils;

pub fn main() -> Result<(), String> {
    let mut app = App::new();
    app.add_plugins((
        // Full screen
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Index(0)),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(LogPlugin {
                // Show INFO for the game, but only ERROR for bevy and wgpu
                // filter: "omc_galaxy=debug,calloop=info,cosmic_text=info,wgpu_core=error,wgpu_hal=error".into(),
                filter: "info,wgpu_core=error,wgpu_hal=error".into(),
                level: Level::TRACE,
                ..default()
            }),
    ))
    .add_plugins(TweeningPlugin)
    .add_systems(PreStartup, utils::assets::load_assets)
    .add_systems(
        Startup,
        (
            game::orchestrator::setup_orchestrator,
            app::setup::setup.after(setup_orchestrator),
            ui::menu::draw_entity_info_menu.after(setup_orchestrator),
            ui::menu::draw_game_options_menu,
        ),
    )
    .add_systems(
        Update,
        (
            ui::buttons::button_hover,
            ui::buttons::game_menu_action,
            ui::buttons::manual_planet_action,
            ui::buttons::manual_explorer_action,
            ui::buttons::explorer_move_action,
            ui::scroll::send_scroll_events,
            ui::visibility::update_explorer_buttons_visibility,
            ui::visibility::update_planet_buttons_visibility,
            ui::dropdown::populate_dropdown,
            ui::menu::update_game_state_text,
            galaxy::celestial::despawn_celestial,
            galaxy::selection::update_selected_entity,
            game::logs::log_text,
        ),
    )
    .add_systems(
        FixedUpdate,
        (game::game::game_loop, galaxy::topology::draw_topology),
    )
    .add_observer(galaxy::topology::destroy_link)
    .add_observer(galaxy::celestial::move_celestial)
    .add_observer(explorers::movement::move_explorer)
    .add_observer(ui::scroll::on_scroll_handler);
    app.run();
    Ok(())
}
