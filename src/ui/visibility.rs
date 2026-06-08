use bevy::prelude::*;

use crate::ecs::components::Explorer;
use crate::ecs::markers::ManualExplorer;
use crate::ecs::resources::{EntityClickRes, ExpState};
use crate::utils::traits::Visible;

pub fn update_button_visibility<T>(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Visibility, With<T>>,
) where 
    T: Component + Visible
{
    if !selected.is_changed() {
        return;
    }

    let select_in = selected.into_inner();

    for mut visibility in &mut query {
        if T::is_selected(select_in) {
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
