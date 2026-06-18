use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowPlugin};

use bevy_tweening::TweeningPlugin;

use bevy_embedded_assets::EmbeddedAssetPlugin;

use crate::ecs::markers::{ExplorerOnlyButton, PlanetOnlyButton};
use crate::ecs::resources::GameState;
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
    app.add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins((
            // Full screen
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
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
        .init_state::<GameState>()
        .add_systems(PreStartup, utils::assets::load_assets)
        .add_systems(Startup, (app::setup::setup_camera,))
        .add_systems(
            OnEnter(GameState::WaitingStart),
            (ui::start::start_splash, utils::debug::log_state_entry),
        )
        .add_systems(OnEnter(GameState::Playing), utils::debug::log_state_entry)
        .add_systems(OnEnter(GameState::Override), utils::debug::log_state_entry)
        .add_systems(
            OnEnter(GameState::GameOver),
            (ui::game_over::spawn_game_over_splash, utils::debug::log_state_entry),
        )
        .add_systems(
            OnEnter(GameState::Paused),
            (
                game::game::flush_events_before_pause,
                utils::debug::log_state_entry,
            ),
        )
        .add_systems(
            OnExit(GameState::Paused),
            game::game::flush_events_before_resume,
        )
        .add_systems(
            Update,
            ui::start::start_menu_actions.run_if(in_state(GameState::WaitingStart)),
        )
        .add_systems(
            Update,
            ui::game_over::game_over_actions.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            OnExit(GameState::WaitingStart),
            (
                ui::start::cleanup_start_menu.before(setup_orchestrator),
                game::orchestrator::setup_orchestrator,
                app::setup::setup.after(setup_orchestrator),
                ui::menu::draw_entity_info_menu.after(setup_orchestrator),
                ui::menu::draw_game_options_menu,
                ui::notification::draw_notifications,
            ),
        )
        .add_systems(
            Update,
            (
                ui::buttons::button_hover,
                ui::buttons::ratio_action,
                ui::buttons::game_menu_action,
                ui::buttons::manual_planet_action,
                ui::buttons::manual_explorer_action,
                ui::buttons::explorer_move_action,
                ui::buttons::cycle_explorer_action,
                ui::buttons::cycle_planet_action,
                ui::buttons::random_entity_action,
                ui::scroll::send_scroll_events,
                ui::visibility::update_button_visibility::<ExplorerOnlyButton>,
                ui::visibility::update_button_visibility::<PlanetOnlyButton>,
                ui::visibility::update_manual_explorer_visibility,
                ui::visibility::update_alive_explorer_button_visibility,
                ui::visibility::update_alive_planet_button_visibility,
                ui::visibility::update_alive_planet_actions_visibility,
            )
                .run_if(not(in_state(GameState::WaitingStart)).and(not(in_state(GameState::GameOver)))),
        )
        .add_systems(
            Update,
            (
                ui::dropdown::fill_neighbors_dropdown,
                ui::dropdown::fill_basic_dropdown,
                ui::dropdown::fill_complex_dropdown,
                ui::menu::update_game_state_text,
                ui::notification::update_notification,
                galaxy::celestial::despawn_celestial,
                galaxy::selection::update_selected_entity,
                game::logs::log_text,
                explorers::movement::change_explorer_mode_text,
                app::scaling::scale_background,
            )
                .run_if(not(in_state(GameState::WaitingStart)).and(not(in_state(GameState::GameOver)))),
        )
        .add_systems(
            FixedUpdate,
            (game::game::game_loop, galaxy::topology::draw_topology)
                .run_if(not(in_state(GameState::WaitingStart)).and(not(in_state(GameState::GameOver)))),
        )
        .add_observer(galaxy::topology::destroy_link)
        .add_observer(galaxy::celestial::move_celestial)
        .add_observer(explorers::movement::move_explorer)
        .add_observer(ui::scroll::on_scroll_handler)
        .add_observer(ui::notification::set_notification);
    app.run();
    Ok(())
}
