use bevy::prelude::*;
use omc_galaxy::Status;
use std::collections::BTreeMap;

use crate::{
    ecs::{
        components::{Explorer, Planet, UiExplorerText, UiPlanetText},
        resources::{EntityClickRes, ExplorerInfoRes, PlanetInfoRes},
    },
    utils::{constants::{EXPLORER_SIZE, PLANET_RAD}, traits::Printable},
};

pub(crate) fn choose_on_click(
    click: On<Pointer<Click>>,
    mut params: ParamSet<(
        Query<(&mut Sprite, &Planet)>,
        Query<(&mut Sprite, &Explorer)>,
    )>,
    mut chosen_entity: ResMut<EntityClickRes>,
) {
    info!("Picking event was triggered");

    //reset all sprite dimensions to normal
    for (mut sprite, _) in &mut params.p0() {
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.));
    }

    for (mut sprite, _) in &mut params.p1() {
        sprite.custom_size = Some(Vec2::splat(EXPLORER_SIZE));
    }

    if let Ok((mut sprite, planet)) = params.p0().get_mut(click.entity) {
        info!("picked info for planet {}", planet.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.5));

        chosen_entity.planet = Some(planet.id);
        chosen_entity.explorer = None;
    }

    if let Ok((mut sprite, explorer)) = params.p1().get_mut(click.entity) {
        info!("picked info for explorer {}", explorer.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(EXPLORER_SIZE * 1.5));

        chosen_entity.explorer = Some(explorer.id);
        chosen_entity.planet = None;
    }
}

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

            for (mut text, _) in &mut params.p1() {
                **text = "".to_string();
            }
        }
    }

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

            for (mut text, _) in &mut params.p0() {
                **text = "".to_string();
            }
        }
    }
}
