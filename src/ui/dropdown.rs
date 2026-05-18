use std::collections::HashSet;

use crate::ecs::components::ExpButtonActions;
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
    let Some(explorer_id) = target_entity.explorer else {
        return;
    };

    let planet_id = explorer_status
        .map
        .get_current_planet(&explorer_id)
        .unwrap_or(0);

    let planets_info = orchestrator.orchestrator.get_planets_info();
    let single_info = planets_info.get_info(planet_id);

    let empty = HashSet::new();

    let resources = single_info
        .and_then(|p| p.supported_resources.as_ref())
        .unwrap_or(&empty);

    for (list, dropd) in lists.iter() {
        if !matches!(dropd, ListType::BasicList) {
            continue;
        }

        fill_dropdown(
            &mut commands,
            list,
            planet_id,
            explorer_id,
            resources,
            ExpButtonActions::CreateBasic,
            "No resources.",
            |r| format!("{r:?}"),
        );
    }
}

pub fn fill_complex_dropdown(
    mut commands: Commands,
    lists: Query<(Entity, &ListType)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    explorer_status: Res<ExplorerInfoRes>,
    target_entity: Res<EntityClickRes>,
) {
    let Some(explorer_id) = target_entity.explorer else {
        return;
    };

    let planet_id = explorer_status
        .map
        .get_current_planet(&explorer_id)
        .unwrap_or(0);

    let planets_info = orchestrator.orchestrator.get_planets_info();
    let single_info = planets_info.get_info(planet_id);

    let empty = HashSet::new();

    let resources = single_info
        .and_then(|p| p.supported_combination.as_ref())
        .unwrap_or(&empty);

    for (list, dropd) in lists.iter() {
        if !matches!(dropd, ListType::ComplexList) {
            continue;
        }

        fill_dropdown(
            &mut commands,
            list,
            planet_id,
            explorer_id,
            resources,
            ExpButtonActions::CreateComplex,
            "No combinations.",
            |r| format!("{r:?}"),
        );
    }
}

fn fill_dropdown<T, FAction, FText>(
    commands: &mut Commands,
    list: Entity,
    planet_id: u32,
    explorer_id: u32,
    resources: &HashSet<T>,
    make_action: FAction,
    empty_text: &str,
    make_text: FText,
) where
    T: Copy,
    FAction: Fn(T) -> ExpButtonActions,
    FText: Fn(T) -> String,
{
    commands.entity(list).despawn_children();

    commands.entity(list).with_children(|parent| {
        if !resources.is_empty() {
            for resource in resources {
                parent
                    .spawn((
                        Button,
                        make_action(*resource),
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
                            Text::new(make_text(*resource)),
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
                        Text::new(empty_text),
                        TextFont {
                            font_size: 12.0,
                            ..Default::default()
                        },
                    ));
                });
        }
    });
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
