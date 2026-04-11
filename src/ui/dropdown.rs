use std::collections::HashSet;

use crate::ecs::resources::{EntityClickRes, OrchestratorResource};
use crate::ecs::{
    components::{DropdownItem, Edge, ListType},
    resources::ExplorerInfoRes,
};
use bevy::prelude::*;

pub fn fill_basic_dropdown(
    mut commands: Commands,
    lists: Query<(Entity, &ListType)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    explorer_status: Res<ExplorerInfoRes>,
    target_entity: Res<EntityClickRes>,
) {
    for (list, dropd) in lists {
        // don't run if the user is not looking at an explorer
        if target_entity.explorer.is_none() || !matches!(dropd, ListType::BasicList) {
            continue;
        }

        // clear previous table
        commands.entity(list).despawn_children();

        let explorer_id = target_entity.explorer.unwrap();
        let planet_id = explorer_status
            .map
            .get_current_planet(&explorer_id)
            .unwrap_or(0);

        //get the planet's available resources
        let planets_info = orchestrator.orchestrator.get_planets_info();
        let single_info = planets_info.get_info(planet_id);
        info!("got info of planet {}: {:?}", planet_id, single_info);

        let resources = match single_info {
            Some(safe_planets) => {
                if let Some(x) = &safe_planets.supported_resources{
                    x
                } else {
                    &HashSet::new()
                }
            },
            None => &HashSet::new(),
        };

        info!("RESOURCE MENU: resource hash this turn was: {:?}", resources);

        // create the actual UI element
        commands.entity(list).with_children(|parent| {
            if !resources.is_empty() {
                for basic in resources {
                    parent
                        .spawn((
                            Button,
                            Node {
                                height: Val::Px(28.0),
                                padding: UiRect::horizontal(Val::Px(8.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            DropdownItem {
                                planet_id,
                                explorer_id,
                            },
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new(format!("{:?}", basic)),
                                TextFont {
                                    font_size: 14.0,
                                    ..Default::default()
                                },
                            ));
                        });
                }
            } else {
                parent
                    .spawn((
                        Button,
                        Node {
                            height: Val::Px(28.0),
                            padding: UiRect::horizontal(Val::Px(8.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        DropdownItem {
                            planet_id,
                            explorer_id,
                        },
                    ))
                    .with_children(|item| {
                        item.spawn((
                            Text::new("No resources."),
                            TextFont {
                                font_size: 12.0,
                                ..Default::default()
                            },
                        ));
                    });
            }
        });
    }
}

/// fill the destination menu for explorers with the neighbors of the
/// planet that is currently being visited by the explorer
pub fn fill_neighbors_dropdown(
    mut commands: Commands,
    edges: Query<&Edge>,
    lists: Query<(Entity, &ListType)>,
    explorer_status: Res<ExplorerInfoRes>,
    target_entity: Res<EntityClickRes>,
) {
    for (list, dropd) in lists {
        // don't run if the user is not looking at an explorer
        if target_entity.explorer.is_none() || !matches!(dropd, ListType::MoveList) {
            continue;
        }

        // clear previous table
        commands.entity(list).despawn_children();

        let explorer_id = target_entity.explorer.unwrap();
        let planet_id = explorer_status
            .map
            .get_current_planet(&explorer_id)
            .unwrap_or(0);

        let mut neighbors = Vec::new();

        // get the neigbors of the planet
        for edge in edges {
            if edge.connects.0 == planet_id {
                neighbors.push(edge.connects.1);
            } else if edge.connects.1 == planet_id {
                neighbors.push(edge.connects.0);
            }
        }

        neighbors.sort_unstable();
        neighbors.dedup();

        // create the actual UI element
        commands.entity(list).with_children(|parent| {
            if !neighbors.is_empty() {
                for planet_id in neighbors {
                    parent
                        .spawn((
                            Button,
                            Node {
                                height: Val::Px(28.0),
                                padding: UiRect::horizontal(Val::Px(8.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            DropdownItem {
                                planet_id,
                                explorer_id,
                            },
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new(format!("Planet {}", planet_id)),
                                TextFont {
                                    font_size: 14.0,
                                    ..Default::default()
                                },
                            ));
                        });
                }
            } else {
                parent
                    .spawn((
                        Button,
                        Node {
                            height: Val::Px(28.0),
                            padding: UiRect::horizontal(Val::Px(8.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        DropdownItem {
                            planet_id,
                            explorer_id,
                        },
                    ))
                    .with_children(|item| {
                        item.spawn((
                            Text::new("no destinations!"),
                            TextFont {
                                font_size: 14.0,
                                ..Default::default()
                            },
                        ));
                    });
            }
        });
    }
}
