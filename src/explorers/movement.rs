use bevy::prelude::*;

use crate::{
    ecs::{
        components::{Explorer, Planet},
        events::MoveExplorerEvent, markers::ExpModeText, resources::EntityClickRes,
    },
    utils::assets::SFXAssets,
};

/// Move the explorer to a different planet
/// 
/// The system moves an explorer to a different planet by changing the
/// explorer's transform to the appropriate coordiantes with respect to the
/// destination planet. This function does NOT call the orchestrator and only
/// moves the explorer from a graphical point of view
pub fn move_explorer(
    event: On<MoveExplorerEvent>,
    mut commands: Commands,
    sfx: Res<SFXAssets>,
    mut param_set: ParamSet<(
        Query<(&mut Explorer, &mut Transform)>,
        Query<(&Planet, &Transform), Without<Explorer>>,
    )>,
) {
    let (explorer_id, planet_id) = (event.id, event.destination);

    // First, get the target planet's transform
    let mut target_transform = None;
    for (planet, &transform) in param_set.p1().iter() {
        if planet.id == planet_id {
            target_transform = Some(transform);
            break;
        }
    }

    // Then, update the explorer
    for (mut explorer, mut transform) in param_set.p0().iter_mut() {
        if explorer.id == explorer_id {
            match target_transform {
                Some(target) => {
                    // semantically move the explorer
                    explorer.current_planet = planet_id;
                    // graphically move the explorer
                    *transform = Transform::from_translation(Vec3 {
                        x: target.translation.x + explorer.position_offset.0,
                        y: target.translation.y + explorer.position_offset.1,
                        z: 3.,
                    });

                    // play sfx
                    if let Some(source) = sfx.handles.get("explorer_pop") {
                        commands.spawn(
                            AudioPlayer::new(source.clone()), // cloning handles is a shallow copy
                        );
                    }
                }
                None => {
                    warn!(
                        "explorer tried to move to planet that doesn't exist ({})",
                        planet_id
                    );
                }
            }
        }
    }
}

pub(crate) fn change_explorer_mode_text (
    parents: Query<&Children, With<ExpModeText>>,
    mut texts: Query<&mut Text>,
    selected: Res<EntityClickRes>,
    explorers: Query<&Explorer>,
) {
    for children in &parents {
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                if let Some(id) = selected.explorer {
                    if let Some(exp) = explorers.iter().find(|e| e.id == id) {
                        text.0 = format!("Explorer mode: {:?}", exp.state);
                    }
                }
            }
        }
    }
}