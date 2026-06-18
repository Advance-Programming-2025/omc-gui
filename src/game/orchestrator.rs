use std::collections::VecDeque;

use bevy::prelude::*;
use omc_galaxy::Orchestrator;

use crate::{
    ecs::resources::{
        EntityClickRes, ExplorerInfoRes, GalaxySnapshot, GameTimer, LogTextRes,
        OrchestratorResource, PlanetInfoRes, StartupConfig,
    },
    utils::constants::GAME_TICK,
};

/// Orchestrator and lower level initialization
/// 
/// The system creates an orchestrator object to then place it in its corresponding resource.
/// It also initializes the galaxy topology in the orchestrator, gets the necessary maps, starts the AIs,
/// starts the game tick timer and initializes the logging resource
pub fn setup_orchestrator(mut commands: Commands, start_config: Res<StartupConfig>) {
    dotenv::dotenv().ok();

    let mut orchestrator = Orchestrator::new().expect("Failed to create orchestrator");

    if let Some(path) = &start_config.topology_path {
        orchestrator
        .initialize_galaxy_by_file(
            path
                .to_str()
                .expect("failed to load path from file. try changing the galaxy file."),
        )
        .expect("Failed to initialize galaxy");
    } else {
        let res = orchestrator.initialize_galaxy_by_random_selection(10);
        if let Err(e) = res {
            error!("Error in random galaxy initialization: {}", e)
        }
    }

    let (topology, planet_num) = orchestrator.get_topology();

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

    let first_string = String::from("Orchestrator has started.\nWelcome to the game!");
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
