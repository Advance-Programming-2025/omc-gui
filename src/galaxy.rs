use bevy::prelude::*;
use std::{f32::consts::TAU, sync::{Arc, RwLock}};

use crate::assets::{PlanetAssets, SPRITE_NUM};

#[derive(Resource)]
pub(crate) struct GalaxyTopologyResource {
    pub topology: Arc<RwLock<Vec<Vec<bool>>>>
}

#[derive(Component)]
pub(crate) struct Planet{
    id: usize
}

const PLANET_RAD: f32 = 50.;
const GALAXY_RADIUS: f32 = 250.;
//const MAX_PLANET_TYPES: usize = 7;

pub fn setup(
    topology: Res<GalaxyTopologyResource>,
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

    // get the galaxy topology via the resource.
    // here, we only need it for the amount of planets to spawn.
    let initial_gtop = topology
        .as_ref()
        .topology
        .try_read()
        .unwrap(); //TODO do something safer than this

    let planet_num = initial_gtop.len();

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
    topology: Res<GalaxyTopologyResource>,
    planets: Query<(&Planet, &Transform)>
) {
    let gtop = topology
        .as_ref()
        .topology
        .try_read()
        .unwrap();

    for (p1,p1t) in planets {
        for (p2, p2t) in planets{

            // using < avoids calculating "double edges" (not a bug but unneeded work)
            if p1.id < p2.id && gtop[p1.id][p2.id] == true {

                let start = &p1t.translation;
                let end = &p2t.translation;
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

                commands.spawn((Sprite{
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(length, 1.)),
                    ..default()
                }, transform));
            }
        }
    }
}