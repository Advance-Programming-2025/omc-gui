use bevy::prelude::*;

use crate::{ecs::{
    components::{Edge, Explorer, Planet},
    events::PlanetDespawn,
    resources::{ExpState, GalaxySnapshot},
}, utils::assets::SFXAssets};

pub fn destroy_link(
    event: On<PlanetDespawn>,
    mut commands: Commands,
    edge_query: Query<(&Edge, Entity)>,
    planet_query: Query<(&Planet, Entity)>,
    explorer_query: Query<(&mut Explorer, Entity)>,
    sfx: Res<SFXAssets>
) {
    //despawn all its links
    for (e, s) in edge_query {
        if e.connects.0 == event.planet_id || e.connects.1 == event.planet_id {
            commands.entity(s).despawn();
        }
    }

    // despawn any explorer on the planet
    for (mut exp,ent) in explorer_query {
        if exp.current_planet == event.planet_id {
            commands.entity(ent).despawn();
            exp.state = ExpState::Dead;
        }
    }


    //despawn the planet itself
    for (p, e) in planet_query {
        if p.id == event.planet_id {
            commands.entity(e).despawn();
        }
    }

    // play sfx
    if let Some(source) = sfx.handles.get(&String::from("planet_death")){
        commands.spawn(
            AudioPlayer::new(source.clone()) // cloning handles is a shallow copy
        );
    }

}

pub fn draw_topology(
    mut commands: Commands,
    snapshot: Res<GalaxySnapshot>,
    planets: Query<(&Planet, &Transform)>,
    old_links: Query<(&Edge, Entity)>,
) {
    if snapshot.is_changed() {
        for (_, en) in old_links {
            commands.entity(en).despawn();
        }
        let gtop = &snapshot.edges;

        for (a, b) in gtop.iter() {
            let (_, t1) = planets.iter().find(|(p, _)| p.id as u32 == *a).unwrap();
            let (_, t2) = planets.iter().find(|(p, _)| p.id as u32 == *b).unwrap();

            let start = &t1.translation;
            let end = &t2.translation;
            let length = start.distance(*end);

            // diff is the same segment as start and end,
            // but transposed wrt the origin of the
            // coordinate system
            let segment = start - end;

            // finds the rotation of the segment wrt the origin
            // using the arctangent function
            let segment_rotation = segment.y.atan2(segment.x);
            let midpoint = (start + end) / 2.;

            //creates the transform to manipulate the line position
            let transform = Transform::from_xyz(midpoint.x, midpoint.y, 1.)
                .with_rotation(Quat::from_rotation_z(segment_rotation));

            commands.spawn((
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(length, 1.)),
                    ..default()
                },
                transform,
                Edge { connects: (*a, *b) },
            ));
        }
    }
}
