use bevy::prelude::*;

use crate::ecs::components::{ExplorerOnlyButton, PlanetOnlyButton};
use crate::ecs::resources::EntityClickRes;

pub fn update_planet_buttons_visibility(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Visibility, With<PlanetOnlyButton>>,
) {
    if !selected.is_changed() {
        return;
    }

    for mut visibility in &mut query {
        if selected.planet.is_some() {
            *visibility = Visibility::Visible;
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
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
