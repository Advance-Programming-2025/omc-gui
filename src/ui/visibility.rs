use bevy::prelude::*;

use crate::ecs::components::{Explorer, ExplorerOnlyButton, ManualExplorer, PlanetOnlyButton};
use crate::ecs::resources::{EntityClickRes, ExpState};

pub fn update_planet_buttons_visibility(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Visibility, With<PlanetOnlyButton>>,
) {
    if !selected.is_changed() {
        return;
    }

    for mut visibility in &mut query {
        if selected.planet.is_some() {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn update_explorer_buttons_visibility(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Visibility, With<ExplorerOnlyButton>>,
) {
    if !selected.is_changed() {
        return;
    }

    for mut visibility in &mut query {
        if selected.explorer.is_some() {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn update_manual_explorer_visibility(
    selected: Res<EntityClickRes>,
    explorers: Query<&Explorer>,
    mut query: Query<&mut Visibility, With<ManualExplorer>>,
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

    for mut visibility in &mut query {
        if visible {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
