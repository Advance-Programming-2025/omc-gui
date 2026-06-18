use std::{collections::VecDeque, fmt::Display, path::PathBuf};

use bevy::prelude::*;
use omc_galaxy::{Orchestrator, PlanetInfoMap, utils::ExplorerInfoMap};

use crate::utils::constants::{DEFAULT_RANDOM_PLANETS, DEFAULT_SUNRAY_RATIO};

/// Main holder for the orchestrator, all interactions with parts of the simulation should pass through here
#[derive(Resource)]
pub struct OrchestratorResource {
    pub orchestrator: Orchestrator,
}

/// State map for the explorer AI
#[derive(Resource, Debug)]
pub enum ExpState {
    Auto,
    Manual,
    Dead,
}

/// All possible states of the game
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    WaitingStart,
    Playing,
    Paused,
    Override,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out_str = match self {
            GameState::WaitingStart => String::from("Waiting to start"),
            GameState::Playing => String::from("Playing"),
            GameState::Paused => String::from("Paused"),
            GameState::Override => String::from("OVERRIDE"),
        };
        write!(f, "{}", out_str)
    }
}

/// Keeps track of the galaxy topology as the game progresses
#[derive(Resource, Clone)]
pub struct GalaxySnapshot {
    pub edges: Vec<(u32, u32)>,
    pub planet_num: usize,
}

/// Holds the initially computed planet size, so that other components can utilize it
#[derive(Resource)]
pub struct PlanetSizeRes {
    pub planet_rad: f32,
    pub exp_rad: f32,
}

/// Holds the entity that the user has currently selected in order to update the state of the entity menu
#[derive(Resource, Debug)]
pub struct EntityClickRes {
    pub planet: Option<u32>,
    pub explorer: Option<u32>,
}

/// A direct copy of the orchestrator's `PlanetInfoMap`
#[derive(Resource)]
pub struct PlanetInfoRes {
    pub map: PlanetInfoMap,
}

/// A direct copy of the orchestrator's `ExplorerInfoMap`
#[derive(Resource)]
pub struct ExplorerInfoRes {
    pub map: ExplorerInfoMap,
}

/// Resource keeping track of all the loggable events that have happened in the game 
#[derive(Resource)]
pub struct LogTextRes {
    pub text: VecDeque<String>,
}

/// Main game timer, is used to actually decide when the game tick has occurred
#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(pub Timer);

/// Holds the message that is supposed to be displayed and how long to display it
#[derive(Resource)]
pub struct ActiveNotification {
    pub message: Option<String>,
    pub active_time: Timer,
}

/// Holds the initial parameters used to start the game.
/// * `topology_path` = OS path to the file that contains a galaxy topology
/// * `ratio` = sunray to asteroid spawn probability
/// * `random_planets` = number of planets in a randomly generated galaxy
#[derive(Resource, Clone)]
pub struct StartupConfig {
    pub topology_path: Option<PathBuf>,
    pub ratio: i32,
    pub random_planets: u32
}

impl Default for StartupConfig {
    fn default() -> Self {
        Self {
            topology_path: Some("assets/topologies/default.txt".into()),
            ratio: DEFAULT_SUNRAY_RATIO,
            random_planets: DEFAULT_RANDOM_PLANETS
        }
    }
}

#[derive(Resource)]
pub struct GalaxyScale {
    pub scale: f32,
}
