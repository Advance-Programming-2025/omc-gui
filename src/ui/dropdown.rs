use crate::ecs::resources::EntityClickRes;
use crate::ecs::{
    components::{DropdownItem, DropdownList, Edge},
    resources::ExplorerInfoRes,
};
use bevy::prelude::*;

pub fn populate_dropdown(
    mut commands: Commands,
    edges: Query<&Edge>,
    list: Single<Entity, With<DropdownList>>,
    explorer_status: Res<ExplorerInfoRes>,
    target_entity: Res<EntityClickRes>,
) {
    if target_entity.explorer.is_none() {
        return;
    }

    commands.entity(*list).despawn_children();
    let explorer_id = target_entity.explorer.unwrap();
    let planet_id = explorer_status
        .map
        .get_current_planet(&explorer_id)
        .unwrap_or(0);

    let mut neighbors = Vec::new();

    for edge in edges {
        if edge.connects.0 == planet_id {
            neighbors.push(edge.connects.1);
        } else if edge.connects.1 == planet_id {
            neighbors.push(edge.connects.0);
        }
    }

    neighbors.sort_unstable();
    neighbors.dedup();

    commands.entity(*list).with_children(|parent| {
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
    });
}
