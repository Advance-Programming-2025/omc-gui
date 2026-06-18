use bevy::prelude::*;
use bevy_tweening::{CycleCompletedEvent, Tween, TweenAnim, lens::TransformPositionLens};
use omc_galaxy::Status;
use std::time::Duration;

use crate::{
    ecs::{
        components::Planet,
        events::{Celestial, CelestialBody, PlanetDespawn},
        resources::{PlanetInfoRes, PlanetSizeRes},
    },
    utils::{assets::CelestialAssets, constants::GAME_TICK},
};

/// Move a celestial body (sunray or asteroid) towards a planet.
///
/// The moving animation is made by interpolating the movement over the start
/// (the center of the galaxy) and the end (the center of the target planet) using
/// a quadratic in and out function
pub fn move_celestial(
    event: On<Celestial>,
    mut commands: Commands,
    sprites: Res<CelestialAssets>,
    planet_query: Query<(&Planet, &Transform)>,
    size: Res<PlanetSizeRes>,
) {
    info!("MOVE_CELESTIAL: EVENT FROM ID {} ", event.planet_id);

    for (p, t) in planet_query {
        if p.id == event.planet_id {
            let sunray_sprite = match event.kind {
                CelestialBody::Sunray => {
                    info!("spawning sunray sprite");
                    sprites.handles.0.clone()
                }
                CelestialBody::Asteroid => {
                    info!("spawning asteroid sprite");
                    sprites.handles.1.clone()
                }
            };

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(GAME_TICK / 2.),
                TransformPositionLens {
                    start: Vec3::new(0., 0., 2.0),
                    end: Vec3::new(t.translation.x, t.translation.y, 2.0),
                },
            )
            .with_cycle_completed_event(true);

            commands.spawn((
                Celestial {
                    kind: event.kind,
                    planet_id: event.planet_id,
                },
                Sprite {
                    image: sunray_sprite,
                    custom_size: Some(Vec2::splat(size.planet_rad)),
                    ..default()
                },
                Transform::from_xyz(0., 0., 2.0),
                TweenAnim::new(tween),
            ));
        }
    }
}

/// Remove the celestial body after the animation is done
///
/// This system checks for animations that have finished the interpolation cycle,
/// queries for the corresponding sprite and removes it from the game world to then
/// trigger the [PlanetDespawn] System if the planet has been hit and died
pub(crate) fn despawn_celestial(
    mut commands: Commands,
    mut reader: MessageReader<CycleCompletedEvent>,
    status: Res<PlanetInfoRes>,
    celestial: Query<&Celestial>,
) {
    // only run if an animation has finished
    for event in reader.read() {
        debug!("animation finished!");

        // if an asteroid hit a planet and killed it, trigger PlanetDespawn
        if let Ok(c) = celestial.get(event.anim_entity) {
            if c.kind == CelestialBody::Asteroid {
                if status.map.get_status(&c.planet_id) == Status::Dead {
                    debug!("Triggerig PlanetDespawn for {}", c.planet_id);
                    commands.trigger(PlanetDespawn {
                        planet_id: c.planet_id,
                    });
                }
            }
        }

        // remove the celestial body from the game world
        commands.entity(event.anim_entity).despawn();
    }
}
