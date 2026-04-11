use bevy::prelude::*;
use omc_galaxy::OrchestratorEvent;

use crate::{
    ecs::{
        events::{Celestial, CelestialBody, MoveExplorerEvent},
        resources::{
            ExplorerInfoRes, GameState, GameTimer, LogTextRes, OrchestratorResource, PlanetInfoRes,
        },
    },
    game::logs::update_logs,
    utils::constants::EXPLORER_NUM,
};

pub fn game_loop(
    mut commands: Commands,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut planets: ResMut<PlanetInfoRes>,
    mut explorers: ResMut<ExplorerInfoRes>,
    mut timer: ResMut<GameTimer>,
    log_text: ResMut<LogTextRes>,
    state: Res<GameState>,
    time: Res<Time>,
) {
    match *state {
        GameState::Playing => {
            timer.tick(time.delta());

            if timer.is_finished() {
                println!("ENTERED TIMER");

                let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);

                handle_tick(&mut commands, events, log_text);

                // update the planet state map after the events occurrederr
                planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
                // yeah
                explorers.as_mut().map = orchestrator.orchestrator.get_explorer_states();
                // launch either an asteroid or a sunray with a random choice
                // TODO make it so the user can choose the amount of asteroids (slider perhaps)
                let _ = orchestrator.orchestrator.choose_random_action(0.8,0.1);
                // handle all of the previous events
                if let Err(s) = orchestrator.orchestrator.handle_game_messages() {
                    error!("could not handle the messages of this tick: {}", s);
                }

                println!("EXITING TIMER");
                timer.reset();
            }
        }
        GameState::Override => {
            //if there are manually imputted events, run those immediately
            //else, keep going

            if orchestrator.orchestrator.gui_messages.len() > 0 {
                let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);
                handle_tick(&mut commands, events, log_text);

                // handle all of the previous events
                let _ = orchestrator.orchestrator.handle_game_messages();
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
        _ => {}
    }
}

fn handle_tick(
    commands: &mut Commands,
    events: Vec<OrchestratorEvent>,
    mut log_text: ResMut<LogTextRes>,
) {
    for ev in events {
        match ev {
            OrchestratorEvent::PlanetDestroyed { planet_id } => {
                // handle the destruction of a planet
                info!("game-loop: planet {} has died, ", planet_id);
                update_logs(&mut log_text, format!("planet {} died!\n", planet_id));
            }
            OrchestratorEvent::SunrayReceived { planet_id } => {
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
                info!("game-loop: planet {} should get a sunray, ", planet_id);
                // TODO This is kinda useless I should get rid of it
            }
            OrchestratorEvent::AsteroidSent { planet_id } => {
                info!("game-loop: planet {} should get an asteroid, ", planet_id);
                commands.trigger(Celestial {
                    planet_id,
                    kind: CelestialBody::Asteroid,
                });
                update_logs(
                    &mut log_text,
                    format!("planet {} got an asteroid\n", planet_id),
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
                update_logs(
                    &mut log_text,
                    format!("exp {} moved to pl {}\n", explorer_id, destination),
                );
            }
        }
    }
}
