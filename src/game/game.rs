use bevy::prelude::*;
use omc_galaxy::OrchestratorEvent;

use crate::{
    ecs::{
        components::Explorer,
        events::{Celestial, CelestialBody, MoveExplorerEvent, Notification},
        resources::{
            EntityClickRes, ExpState, ExplorerInfoRes, GameState, GameTimer, LogTextRes,
            OrchestratorResource, PlanetInfoRes, StartupConfig,
        },
    },
    game::logs::update_logs,
    utils::constants::EXPLORER_NUM,
};

/// The core system at the heart of the game.
///
/// At each engine update the timer is ticked. if the timer runs out, the events
/// in the previous rounds are sent to the [handle_tick] function to trigger the necessary
/// observers and handle other specific logic. Afterwards, the maps are updated with their
/// most recent values and a random action (nothing, sunray, asteroid) is chosen for the
/// following round
pub fn game_loop(
    mut commands: Commands,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut planets: ResMut<PlanetInfoRes>,
    mut explorers: ResMut<ExplorerInfoRes>,
    mut timer: ResMut<GameTimer>,
    log_text: ResMut<LogTextRes>,
    state: Res<State<GameState>>,
    mut mutable_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    game_explorers: Query<&Explorer>,
    sel: Res<EntityClickRes>,
    ratio: Res<StartupConfig>,
) {
    match **state {
        GameState::Paused => {
            // CASE: leftover events in the queue
            if orchestrator.orchestrator.gui_messages.len() > 0 {
                let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);
                handle_tick(&mut commands, events, log_text, game_explorers, sel);

                planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
                explorers.as_mut().map = orchestrator.orchestrator.get_explorer_states();
            }
        }
        GameState::Playing => {
            timer.tick(time.delta());

            if timer.is_finished() {
                debug!("ENTERED TIMER");

                let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);

                handle_tick(&mut commands, events, log_text, game_explorers, sel);

                // get the bag of the explorers to update
                for exp_id in 0..EXPLORER_NUM {
                    if !explorers.as_mut().map.is_dead(&exp_id) {
                        if let Err(s) = orchestrator.orchestrator.send_bag_content_request(exp_id) {
                            error!(s);
                        }
                    }
                }
                // update the planet state map after the events occurred
                planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
                // same thing but explorers
                explorers.as_mut().map = orchestrator.orchestrator.get_explorer_states();

                // if all planets die, end the game
                if planets.map.count_survivors() == 0 {
                    mutable_state.set(GameState::GameOver);
                }
                // launch either an asteroid or a sunray with a random choice
                let _ = orchestrator
                    .orchestrator
                    .choose_random_action(0.5, 1. - ((ratio.ratio as f64) / 100.));
                // handle all of the previous events
                if let Err(s) = orchestrator.orchestrator.handle_game_messages() {
                    error!("could not handle the messages of this tick: {}", s);
                }

                debug!("EXITING TIMER");
                timer.reset();
            }
        }
        GameState::Override => {
            //if there are manually imputted events, run those immediately
            //else, keep going

            drain_stale_events(
                &mut commands,
                orchestrator,
                log_text,
                game_explorers,
                sel,
                planets,
                explorers,
            );

            mutable_state.set(GameState::Playing);
        }
        GameState::WaitingStart => {}
        GameState::GameOver => {}
    }
}

/// Routes the events in the tick to the correct functions.
fn handle_tick(
    commands: &mut Commands,
    events: Vec<OrchestratorEvent>,
    mut log_text: ResMut<LogTextRes>,
    explorers: Query<&Explorer>,
    sel: Res<EntityClickRes>,
) {
    for ev in events {
        match ev {
            OrchestratorEvent::PlanetDestroyed { planet_id } => {
                // no need to do anything, PlanetDestroyed is handled by destroy_link
                info!("game-loop: planet {} has died, ", planet_id);
                update_logs(&mut log_text, format!("planet {} died!\n", planet_id));
            }
            OrchestratorEvent::SunrayReceived { planet_id } => {
                // spawn a sunray so that it can be moved to the planet
                info!("game-loop: planet {} got a sunray (UI update), ", planet_id);
                commands.trigger(Celestial {
                    planet_id,
                    kind: CelestialBody::Sunray,
                });
                update_logs(
                    &mut log_text,
                    format!("planet {} got a sunray\n", planet_id),
                );
            }
            OrchestratorEvent::SunraySent { planet_id } => {
                // TODO check if these are still needed
                info!("game-loop: planet {} should get a sunray, ", planet_id);
            }
            OrchestratorEvent::AsteroidSent { planet_id } => {
                // spawn an asteroid so it can be moved to the planet
                commands.trigger(Celestial {
                    planet_id,
                    kind: CelestialBody::Asteroid,
                });
                update_logs(
                    &mut log_text,
                    format!("planet {} got an asteroid\n", planet_id),
                );
            }
            OrchestratorEvent::ExplorerMoveStarted {
                explorer_id,
                destination,
            } => {
                info!(
                    "game-loop: explorer {} move to planet {} initiated",
                    explorer_id, destination
                );
                update_logs(
                    &mut log_text,
                    format!("exp {} moving to pl {}...\n", explorer_id, destination),
                );
            }
            OrchestratorEvent::ExplorerMoved {
                explorer_id,
                destination,
            } => {
                info!(
                    "game-loop: explorer {} has moved to planet {}",
                    explorer_id, destination
                );
                commands.trigger(MoveExplorerEvent {
                    id: explorer_id,
                    destination,
                });

                // if the explorer is in manual mode, send the notification
                if let Some(selected) = sel.explorer {
                    if let Some(expl) = explorers
                        .iter()
                        .find(|exp| exp.id == selected && exp.id == explorer_id)
                    {
                        if matches!(expl.state, ExpState::Manual) {
                            commands.trigger(Notification {
                                message: format!(
                                    "Explorer {} moved to planet {}",
                                    explorer_id, destination
                                ),
                            });
                        }
                    }
                }
                update_logs(
                    &mut log_text,
                    format!("exp {} moved to pl {}\n", explorer_id, destination),
                );
            }
            OrchestratorEvent::ResourceGenerationFailed { message } => {
                warn!("Error in resource generation, notifying user");
                commands.trigger(Notification { message });
            }
        }
    }
}

fn drain_stale_events(
    mut commands: &mut Commands,
    mut orchestrator: ResMut<OrchestratorResource>,
    log_text: ResMut<LogTextRes>,
    game_explorers: Query<&Explorer>,
    sel: Res<EntityClickRes>,
    mut planets: ResMut<PlanetInfoRes>,
    mut explorers: ResMut<ExplorerInfoRes>,
) {
    // handle all of the previous events
    let _ = orchestrator.orchestrator.handle_game_messages();

    if orchestrator.orchestrator.gui_messages.len() > 0 {
        let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);
        handle_tick(&mut commands, events, log_text, game_explorers, sel);

        // update the state maps after the events occurred
        planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
        explorers.as_mut().map = orchestrator.orchestrator.get_explorer_states();
        // get the current state of the explorer bag for the next round (if it's alive)
        for i in 0..EXPLORER_NUM {
            if !explorers.as_mut().map.is_dead(&i) {
                if let Err(s) = orchestrator.orchestrator.send_bag_content_request(i) {
                    error!(s);
                }
            }
        }
    }
}

pub(crate) fn flush_events_before_resume(
    mut commands: Commands,
    orchestrator: ResMut<OrchestratorResource>,
    log_text: ResMut<LogTextRes>,
    game_explorers: Query<&Explorer>,
    sel: Res<EntityClickRes>,
    planets: ResMut<PlanetInfoRes>,
    explorers: ResMut<ExplorerInfoRes>,
) {
    drain_stale_events(
        &mut commands,
        orchestrator,
        log_text,
        game_explorers,
        sel,
        planets,
        explorers,
    );
}

pub(crate) fn flush_events_before_pause(
    mut commands: Commands,
    orchestrator: ResMut<OrchestratorResource>,
    log_text: ResMut<LogTextRes>,
    game_explorers: Query<&Explorer>,
    sel: Res<EntityClickRes>,
    planets: ResMut<PlanetInfoRes>,
    explorers: ResMut<ExplorerInfoRes>,
) {
    let mut dbg = String::new();
    for event in orchestrator.orchestrator.gui_messages.iter().clone() {
        let msg = format!("{:?}", event);
        dbg.push_str(&msg);
    }

    info!("FLUSH: game was paused, flushing the following: {}", dbg);
    drain_stale_events(
        &mut commands,
        orchestrator,
        log_text,
        game_explorers,
        sel,
        planets,
        explorers,
    );
}
