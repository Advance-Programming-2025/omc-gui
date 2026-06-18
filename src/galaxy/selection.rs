use bevy::prelude::*;
use omc_galaxy::Status;
use std::collections::BTreeMap;

use crate::{
    ecs::{
        components::{Explorer, Planet, UiExplorerText, UiPlanetText},
        resources::{EntityClickRes, ExplorerInfoRes, PlanetInfoRes, PlanetSizeRes},
    },
    utils::traits::Printable,
};

/// Updates the entity chosen by the user by giving visual feedback
///
/// This system scales the sprite of the selected entity to make it more evident
/// and updates the [EntityClickRes] resource so that the [update_selected_entity] system can
/// update the menu information
pub(crate) fn choose_on_click(
    click: On<Pointer<Click>>,
    mut params: ParamSet<(
        Query<(&mut Sprite, &Planet)>,
        Query<(&mut Sprite, &Explorer)>,
    )>,
    mut chosen_entity: ResMut<EntityClickRes>,
    size: Res<PlanetSizeRes>,
) {
    info!("Picking event was triggered");

    //reset all sprite dimensions to normal
    for (mut sprite, _) in &mut params.p0() {
        sprite.custom_size = Some(Vec2::splat(size.planet_rad * 2.));
    }

    for (mut sprite, _) in &mut params.p1() {
        sprite.custom_size = Some(Vec2::splat(size.exp_rad));
    }

    if let Ok((mut sprite, planet)) = params.p0().get_mut(click.entity) {
        info!("picked info for planet {}", planet.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(size.planet_rad * 2.5));

        chosen_entity.planet = Some(planet.id);
        chosen_entity.explorer = None;
    }

    if let Ok((mut sprite, explorer)) = params.p1().get_mut(click.entity) {
        info!("picked info for explorer {}", explorer.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(size.exp_rad * 1.5));

        chosen_entity.explorer = Some(explorer.id);
        chosen_entity.planet = None;
    }
}

/// Updates the information regarding the selected entity
///
/// According to the type and ID of the selected entity, the fields in the
/// selected entity menu are updated accordingly: for the planet, that includes the name, ID,
/// status, rocket availability and number of charged cells; for the explorer, that includes
/// the ID, state, visited planet and bag contents
pub(crate) fn update_selected_entity(
    selected_entity: Res<EntityClickRes>,
    planet_status: Res<PlanetInfoRes>,
    explorer_status: Res<ExplorerInfoRes>,
    mut params: ParamSet<(
        Query<(&mut Text, &UiPlanetText)>,
        Query<(&mut Text, &UiExplorerText)>,
    )>,
) {
    // exit early if the state is the same to avoid extra computation
    if !selected_entity.is_changed() && !planet_status.is_changed() {
        return;
    }

    info!("update_selected_entity: {:?}", selected_entity);

    // case: the chosen entity is a planet
    if let Some(planet_id) = selected_entity.planet {
        info!("updating planet {}", planet_id);
        let map = &planet_status.map;

        if let Some(planet_info) = map.get_info(planet_id) {
            for (mut text, field_type) in &mut params.p0() {
                match field_type {
                    UiPlanetText::Name => {
                        **text = format!("Name: {:?}", planet_info.name);
                    }
                    UiPlanetText::Id => {
                        **text = format!("Planet ID: {:?}", planet_id);
                    }
                    UiPlanetText::Status => {
                        **text = format!("Status: {:?}", planet_info.status);
                    }
                    UiPlanetText::Rocket => {
                        if planet_info.rocket {
                            **text = "Rocket: AVAILABLE".to_string();
                        } else {
                            **text = "Rocket: NOT PRESENT".to_string();
                        }
                    }
                    UiPlanetText::Energy => {
                        let current_energy = planet_info.charged_cells_count;
                        let max_energy = planet_info.energy_cells.len();
                        **text = format!("Charged cells: {} out of {}", current_energy, max_energy);
                    }
                }
            }

            // set all of the explorer fields to empty
            for (mut text, _) in &mut params.p1() {
                **text = "".to_string();
            }
        }
    }

    // case: the chosen entity is an explorer
    if let Some(explorer_id) = selected_entity.explorer {
        let map = &explorer_status.map;

        if let Some(explorer_info) = map.get(&explorer_id) {
            for (mut text, field_type) in &mut params.p1() {
                match field_type {
                    UiExplorerText::Id => {
                        **text = format!("Explorer {:?}", explorer_id);
                    }
                    UiExplorerText::Status => {
                        let status = match explorer_info.status {
                            Status::Dead => "dead".to_string(),
                            Status::Paused => "paused".to_string(),
                            Status::Running => "running".to_string(),
                        };
                        **text = format!("Status: {}", status);
                    }
                    UiExplorerText::Visiting => {
                        **text = format!("Visiting planet {}", explorer_info.current_planet_id);
                    }
                    UiExplorerText::ResourceBag => {
                        let bag = &explorer_info.bag;
                        let mut counter_map: BTreeMap<String, usize> = BTreeMap::new();
                        for res in bag {
                            let name = format!("{}", res.to_print());
                            let current_count = counter_map.get(&name).unwrap_or(&0);
                            counter_map.insert(name, *current_count + 1);
                        }
                        **text = format!("Bag: \n{:?}", counter_map);
                    }
                }
            }

            // set all of the planet fields to empty
            for (mut text, _) in &mut params.p0() {
                **text = "".to_string();
            }
        }
    }
}
