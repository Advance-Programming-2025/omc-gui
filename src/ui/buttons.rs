use bevy::prelude::*;

use crate::ecs::components::{
    ButtonActions, DropdownItem, ExpButtonActions, Explorer, RatioButton,
};
use crate::ecs::markers::RatioText;
use crate::ecs::resources::{
    EntityClickRes, ExpState, GameState, LogTextRes, OrchestratorResource, StartupConfig,
};
use crate::game::logs::update_logs;

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
                debug!("Button Pressed!");
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

pub(crate) fn ratio_action(
    mut action_query: Query<(&Interaction, &RatioButton), (Changed<Interaction>, With<Button>)>,
    mut ratio: ResMut<StartupConfig>,
    mut text: Single<(&mut Text, &RatioText)>,
) {
    let current = ratio.ratio;

    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                RatioButton::Increase => {
                    ratio.ratio = i32::min(current + 5, 100);
                }
                RatioButton::Decrease => {
                    ratio.ratio = i32::max(current - 5, 0);
                }
            }
            text.0.0 = format!("Sunray to asteroid ratio: {}%", ratio.ratio)
        }
    }
}

pub(crate) fn game_menu_action(
    mut action_query: Query<(&Interaction, &ButtonActions), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::StartGame => {
                    state.set(GameState::Playing);
                    info!("game started");
                }
                ButtonActions::StopGame => {
                    state.set(GameState::Paused);
                    debug!("game should pause now...");
                }
                ButtonActions::Blind => {
                    state.set(GameState::Override);
                    debug!("entering manual override mode");

                    let mut targets = Vec::new();
                    for id in 0..orchestrator.orchestrator.planets_info.len() {
                        if !orchestrator.orchestrator.planets_info.is_dead(&(id as u32)) {
                            targets.push(id as u32);
                        }
                    }

                    debug!("targets: {:?}", targets);

                    if let Err(s) = orchestrator
                        .orchestrator
                        .send_celestial_from_gui(targets, false)
                    {
                        error!("{}", s);
                    }

                    debug!("done sending sunrays");
                }
                ButtonActions::Nuke => {
                    state.set(GameState::Override);
                    debug!("entering manual override mode");

                    let mut targets = Vec::new();
                    for id in 0..orchestrator.orchestrator.planets_info.len() {
                        if !orchestrator.orchestrator.planets_info.is_dead(&(id as u32)) {
                            targets.push(id as u32);
                        }
                    }

                    if let Err(s) = orchestrator
                        .orchestrator
                        .send_celestial_from_gui(targets, true)
                    {
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
    mut state: ResMut<NextState<GameState>>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::ManualAsteroid => {
                    state.set(GameState::Override);
                    if let Some(id) = selected_planet.planet {
                        if let Err(e) = orchestrator
                            .orchestrator
                            .send_celestial_from_gui(vec![id], true)
                        {
                            error!(e)
                        }
                    }
                }
                ButtonActions::ManualSunray => {
                    state.set(GameState::Override);
                    if let Some(id) = selected_planet.planet {
                        if let Err(e) = orchestrator
                            .orchestrator
                            .send_celestial_from_gui(vec![id], false)
                        {
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
    mut orchestrator: ResMut<OrchestratorResource>,
    selected_entity: Res<EntityClickRes>,
    mut explorers: Query<&mut Explorer>,
    mut log_text: ResMut<LogTextRes>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ExpButtonActions::CreateBasic(basic) => {
                    info!("creating basic resource manually...");
                    if let Some(id) = selected_entity.explorer {
                        let res = orchestrator
                            .orchestrator
                            .send_generate_resource_request(id, *basic);
                        if let Err(_) = res {
                            update_logs(
                                &mut log_text,
                                "Basic resource generation failed!".to_string(),
                            );
                        } else {
                            info!("basic resource was generated");
                        }
                    }
                }
                ExpButtonActions::CreateComplex(complex) => {
                    if let Some(id) = selected_entity.explorer {
                        info!("creating complex resource manually...");
                        let res = orchestrator
                            .orchestrator
                            .send_combine_resource_request(id, *complex);
                        if let Err(_) = res {
                            update_logs(
                                &mut log_text,
                                "Complex resource generation failed!".to_string(),
                            );
                        } else {
                            info!("complex resource was generated");
                        }
                    }
                }
                ExpButtonActions::ExpModeChange => {
                    if let Some(id) = selected_entity.explorer {
                        if let Some(mut target) = explorers.iter_mut().find(|exp| exp.id == id) {
                            match target.state {
                                ExpState::Auto => {
                                    let res = orchestrator.orchestrator.send_stop_explorer_ai(id);

                                    if let Err(e) = res {
                                        update_logs(
                                            &mut log_text,
                                            format!("explorer {} error: no manual mode\n", id),
                                        );
                                        error!("error in expmodechange: {}", e.to_string());
                                    } else {
                                        target.state = ExpState::Manual;
                                        update_logs(
                                            &mut log_text,
                                            format!("exp {} is in manual mode\n", id),
                                        );
                                    }
                                }
                                ExpState::Manual => {
                                    let res = orchestrator.orchestrator.send_start_explorer_ai(id);

                                    if let Err(err_msg) = res {
                                        update_logs(
                                            &mut log_text,
                                            format!("explorer {} error: no auto mode\n", id),
                                        );
                                        error!("error in expmodechange: {}", err_msg);
                                    } else {
                                        target.state = ExpState::Auto;
                                        update_logs(
                                            &mut log_text,
                                            format!("exp {} is in auto mode\n", id),
                                        );
                                    }
                                }
                                ExpState::Dead => {
                                    update_logs(&mut log_text, format!("exp {} has died!\n", id));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn explorer_move_action(
    mut action_query: Query<(&Interaction, &DropdownItem), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            state.set(GameState::Override);

            if let Err(e) = orchestrator
                .orchestrator
                .send_move_explorer_from_gui(action.explorer_id, action.planet_id)
            {
                error!("error in explorer move:{}", e);
            }
        }
    }
}
