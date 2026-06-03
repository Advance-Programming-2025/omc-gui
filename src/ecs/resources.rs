use std::{collections::VecDeque, path::PathBuf, fmt::Display};

use bevy::prelude::*;
use omc_galaxy::{Orchestrator, PlanetInfoMap, utils::ExplorerInfoMap};

use crate::utils::constants::DEFAULT_SUNRAY_RATIO;

#[derive(Resource)]
pub struct OrchestratorResource {
    pub orchestrator: Orchestrator,
}

#[derive(Resource, Debug)]
pub enum ExpState {
    Auto,
    Manual,
    Dead,
}

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

#[derive(Resource, Clone)]
pub struct GalaxySnapshot {
    pub edges: Vec<(u32, u32)>,
    pub planet_num: usize,
}

#[derive(Resource, Debug)]
pub struct EntityClickRes {
    pub planet: Option<u32>,
    pub explorer: Option<u32>,
}

#[derive(Resource)]
pub struct PlanetInfoRes {
    pub map: PlanetInfoMap,
}

#[derive(Resource)]
pub struct ExplorerInfoRes {
    pub map: ExplorerInfoMap,
}

#[derive(Resource)]
pub struct LogTextRes {
    pub text: VecDeque<String>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(pub Timer);

#[derive(Resource)]
pub struct ActiveNotification {
    pub message: Option<String>,
    pub active_time: Timer
}

#[derive(Resource)]
pub struct SunrayAsteroidRatio(pub i32);

impl Default for SunrayAsteroidRatio {
    fn default() -> Self {
        Self(DEFAULT_SUNRAY_RATIO)
    }
}

// TODO add this to the start menu and setup

#[derive(Resource, Clone)]
pub struct StartupConfig {
    pub topology_path: PathBuf,
    pub ratio: i32,
}

impl Default for StartupConfig {
    fn default() -> Self {
        Self {
            topology_path: "assets/default.txt".into(),
            ratio: 80
        }
    }
}