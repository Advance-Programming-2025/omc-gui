use bevy::prelude::*;
use common_game::components::resource::BasicResourceType;

use crate::ecs::components::{ButtonActions, DropdownItem, ExpButtonActions};
use crate::ecs::resources::{EntityClickRes, GameState, OrchestratorResource};

pub(crate) fn button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (&interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                println!("Button Pressed!");
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.07, 0.30, 0.53).into();
            }
        }
    }
}

pub(crate) fn game_menu_action(
    mut action_query: Query<(&Interaction, &ButtonActions), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut state: ResMut<GameState>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::StartGame => {
                    if state.set_if_neq(GameState::Playing) {
                        info!("game started");
                    }
                }
                ButtonActions::StopGame => {
                    if state.set_if_neq(GameState::Paused) {
                        println!("game should pause now...");
                    }
                }
                ButtonActions::Blind => {
                    state.set_if_neq(GameState::Override);
                    debug!("entering manual override mode");

                    let mut targets = Vec::new();
                    for id in 0..orchestrator.orchestrator.planets_info.len() {
                        if !orchestrator.orchestrator.planets_info.is_dead(&(id as u32)) {
                            targets.push(id as u32);
                        }
                    }

                    println!("targets: {:?}", targets);

                    if let Err(s) = orchestrator.orchestrator.send_celestial_from_gui(targets, false) {
                        error!("{}", s);
                    }

                    debug!("done sending sunrays");
                }
                ButtonActions::Nuke => {
                    state.set_if_neq(GameState::Override);
                    debug!("entering manual override mode");

                    let mut targets = Vec::new();
                    for id in 0..orchestrator.orchestrator.planets_info.len() {
                        if !orchestrator.orchestrator.planets_info.is_dead(&(id as u32)) {
                            targets.push(id as u32);
                        }
                    }

                    if let Err(s) = orchestrator.orchestrator.send_celestial_from_gui(targets, true) {
                        error!("{}", s);
                    }

                    debug!("done sending asteroids");
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn manual_planet_action(
    mut action_query: Query<(&Interaction, &ButtonActions), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    selected_planet: Res<EntityClickRes>,
    mut state: ResMut<GameState>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::ManualAsteroid => {
                    state.set_if_neq(GameState::Override);
                    if let Some(id) = selected_planet.planet {
                        if let Err(e) = orchestrator.orchestrator.send_celestial_from_gui(vec![id], true) {
                            error!(e)
                        }
                    }
                }
                ButtonActions::ManualSunray => {
                    state.set_if_neq(GameState::Override);
                    if let Some(id) = selected_planet.planet {
                        if let Err(e) = orchestrator.orchestrator.send_celestial_from_gui(vec![id], false) {
                            error!(e)
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn manual_explorer_action(
    mut action_query: Query<
        (&Interaction, &ExpButtonActions),
        (Changed<Interaction>, With<Button>),
    >,
    orchestrator: ResMut<OrchestratorResource>,
    selected_entity: Res<EntityClickRes>,
    mut state: ResMut<GameState>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ExpButtonActions::CreateBasic => {
                    state.set_if_neq(GameState::Override);
                    if let Some(id) = selected_entity.explorer {
                        let _ = orchestrator
                            .orchestrator
                            .send_generate_resource_request(id, BasicResourceType::Carbon);
                    }
                }
                ExpButtonActions::CreateComplex => {
                    state.set_if_neq(GameState::Override);
                    // TODO this has to be reworked post refactoring
                    error!("CreateComplex: function not yet implemented");
                }
                ExpButtonActions::ExpModeChange => {
                    // TODO toggle manual or auto explorer mode
                    error!("ExpModeChange: function not yet implemented");
                }
            }
        }
    }
}

pub(crate) fn explorer_move_action(
    mut action_query: Query<(&Interaction, &DropdownItem), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut state: ResMut<GameState>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            state.set_if_neq(GameState::Override);
            let _ = orchestrator
                .orchestrator
                .send_stop_explorer_ai(action.explorer_id);
            if let Err(e) = orchestrator
                .orchestrator
                .send_move_to_planet(action.explorer_id, action.planet_id)
            {
                error!("error in explorer move:{}", e);
            }
            let _ = orchestrator
                .orchestrator
                .send_start_explorer_ai(action.explorer_id);
            state.set_if_neq(GameState::Playing);
        }
    }
}
