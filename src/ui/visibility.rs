use bevy::prelude::*;
use omc_galaxy::Status;

use crate::ecs::components::{Explorer, Planet};
use crate::ecs::markers::{
    AliveExplorerButton, AlivePlanetActions, AlivePlanetButton, ManualExplorer,
};
use crate::ecs::resources::{EntityClickRes, ExpState, PlanetInfoRes};
use crate::utils::traits::Visible;

pub fn update_button_visibility<T>(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Node, With<T>>,
) where
    T: Component + Visible,
{
    if !selected.is_changed() {
        return;
    }

    let select_in = selected.into_inner();

    for mut node in &mut query {
        if T::is_selected(select_in) {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }
}

pub fn update_manual_explorer_visibility(
    selected: Res<EntityClickRes>,
    explorers: Query<&Explorer>,
    mut query: Query<&mut Node, With<ManualExplorer>>,
) {
    // gets from all explorers the one with the right ID, then
    // checks if the state is manual, otherwise its menu will get hidden
    let visible = if let Some(id) = selected.explorer {
        explorers
            .iter()
            .find(|exp| exp.id == id)
            .map(|valid| matches!(valid.state, ExpState::Manual))
            .unwrap_or(false)
    } else {
        false
    };

    for mut node in &mut query {
        if visible {
            node.display = Display::Flex;
        } else {
            node.display = Display::None;
        }
    }
}

/// Hide the random explorer button if all explorers are dead
pub fn update_alive_explorer_button_visibility(
    explorers: Query<&Explorer>,
    mut query: Query<&mut Node, With<AliveExplorerButton>>,
) {
    let has_alive = explorers
        .iter()
        .any(|exp| !matches!(exp.state, ExpState::Dead));
    for mut node in &mut query {
        node.display = if has_alive {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// Hide the random planet button if all planets are dead
pub fn update_alive_planet_button_visibility(
    planet_info: Res<PlanetInfoRes>,
    planets: Query<&Planet>,
    mut query: Query<&mut Node, With<AlivePlanetButton>>,
) {
    let has_alive = planets.iter().any(|planet| {
        planet_info
            .map
            .get_info(planet.id)
            .map_or(false, |info| info.status != Status::Dead)
    });
    for mut node in &mut query {
        node.display = if has_alive {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// Hide the Send asteroid/Send sunray buttons when the selected planet is dead
pub fn update_alive_planet_actions_visibility(
    selected: Res<EntityClickRes>,
    planet_info: Res<PlanetInfoRes>,
    mut query: Query<&mut Node, With<AlivePlanetActions>>,
) {
    let visible = selected
        .planet
        .and_then(|id| planet_info.map.get_info(id))
        .map_or(false, |info| info.status != Status::Dead);

    for mut node in &mut query {
        node.display = if visible {
            Display::Flex
        } else {
            Display::None
        };
    }
}
