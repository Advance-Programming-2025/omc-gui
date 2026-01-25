use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::{assets::{PlanetAssets, SPRITE_NUM}, game::GameSnapshot};

#[derive(Component)]
pub(crate) struct Planet{
    id: usize
}

#[derive(Component)]
pub(crate) struct Edge{
    connects: (u32,u32)
}

#[derive(Event)]
pub(crate) struct PlanetDespawn{
    pub planet_id: u32
}

const PLANET_RAD: f32 = 50.;
const GALAXY_RADIUS: f32 = 250.;
//const MAX_PLANET_TYPES: usize = 7;

pub fn setup(
    topology: Res<GameSnapshot>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    planet_assets: Res<PlanetAssets>,
) {

    commands.spawn(Camera2d);

    //create and load background image through sprites
    let background: Handle<Image> = asset_loader.load("sky.png");

    commands.spawn(Sprite{
        image: background,
        custom_size: Some(Vec2::new(1920., 1080.)), // default to FHD
        ..Default::default()
    });

    let planet_num = topology.snapshot.planet_num;

    for i in 0..planet_num {
        
        // spawn all the planets in a circle, with even spacing
        // Tau = 2 * pi, so all the planets go around the circle
        let angle = TAU * (i as f32) / (planet_num as f32);

        // extract x and y position via trigonometry
        let x = GALAXY_RADIUS * angle.cos();
        let y = GALAXY_RADIUS * angle.sin();

        //Handle is based on Arc, so cloning is fine
        let image_handle = planet_assets.handles[i % SPRITE_NUM].clone();

        commands.spawn((
            Planet{id: i},
            Sprite {
                image: image_handle,
                custom_size: Some(Vec2::splat(PLANET_RAD * 2.)),
                ..Default::default()
            },
            Transform::from_xyz(
                x,
                y,
                2.0,
            ),
        ));
    }

}

pub fn draw_topology(
    mut commands: Commands,
    snapshot: Res<GameSnapshot>,
    planets: Query<(&Planet, &Transform)>
) {
    let snap = &snapshot.snapshot;
    if snapshot.is_changed() {
        let gtop = &snap.edges; //TODO do something BETTER than this

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
                    let transform = Transform::from_xyz(
                        midpoint.x,
                        midpoint.y,
                        1.
                    ).with_rotation(Quat::from_rotation_z(segment_rotation));

                    commands.spawn((
                        Sprite{
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(length, 1.)),
                            ..default()
                        }, 
                        transform,
                        Edge{
                            connects: (*a,*b)
                        }
                        ));
        }
    }
}

pub fn destroy_link(
    event: On<PlanetDespawn>,
    mut commands: Commands,
    edge_query: Query<(&Edge, Entity)>,
    planet_query: Query<(&Planet, Entity)>
) {
    //despawn all its links
    for (e,s) in edge_query {
        if e.connects.0 == event.planet_id || e.connects.1 == event.planet_id {
            commands.entity(s).despawn();
        }
    }

    //despawn the planet itself
    for (p, e) in planet_query {
        if p.id == event.planet_id as usize {
            commands.entity(e).despawn();
        }
    }
}