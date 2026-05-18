use std::collections::VecDeque;

use bevy::prelude::*;
use omc_galaxy::Orchestrator;

use crate::{
    ecs::resources::{
        EntityClickRes, ExplorerInfoRes, GalaxySnapshot, GameState, GameTimer, LogTextRes,
        OrchestratorResource, PlanetInfoRes,
    },
    utils::constants::GAME_TICK,
};

pub fn setup_orchestrator(mut commands: Commands) {
    dotenv::dotenv().ok();

    let mut orchestrator = Orchestrator::new().expect("Failed to create orchestrator");

    let file_path = std::env::var("INPUT_FILE").expect("Set INPUT_FILE in .env or env vars");

    orchestrator
        .initialize_galaxy_by_file(file_path.as_str().trim())
        .expect("Failed to initialize galaxy");

    let (topology, planet_num) = orchestrator.get_topology();

    let first_string = String::from("Orchestrator has started.\nWelcome to the game!");

    let lookup = orchestrator.get_planets_info();

    let exp_info = orchestrator.get_explorer_states();

    if let Err(s) = orchestrator.start_all(&[(0u32, 0u32)], &[(1u32, 0u32)]) {
        error!("{}", s);
    }

    commands.insert_resource(OrchestratorResource { orchestrator });

    commands.insert_resource(GalaxySnapshot {
        edges: topology,
        planet_num,
    });

    commands.insert_resource(PlanetInfoRes { map: lookup });

    commands.insert_resource(ExplorerInfoRes { map: exp_info });

    commands.insert_resource(GameState::WaitingStart);

    commands.insert_resource(LogTextRes {
        text: VecDeque::from([first_string]),
    });

    commands.insert_resource(GameTimer(Timer::from_seconds(
        GAME_TICK,
        TimerMode::Repeating,
    )));

    commands.insert_resource(EntityClickRes {
        planet: None,
        explorer: None,
    });
}
