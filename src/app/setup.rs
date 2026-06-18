use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::{
    app::scaling::{scale_offset_size, scale_planet_size},
    ecs::{
        components::{Explorer, Planet},
        markers::Background,
        resources::{ActiveNotification, ExpState, GalaxySnapshot, PlanetInfoRes, PlanetSizeRes},
    },
    galaxy::selection::choose_on_click,
    utils::{
        assets::{ExplorerAssets, PlanetAssets},
        constants::{EXP_SPRITE_NUM, GALAXY_RADIUS, PLANET_SPRITE_NUM},
    },
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}


/// Galaxy setup system
/// 
/// The function achieves the following, in order:
/// - loads the background image
/// - spawns all planets with their respective sprite and transform
/// - spawns the explorers on the planet with ID 0
/// - spawns the active notification object 
pub fn setup(
    galaxy: Res<GalaxySnapshot>,
    planets: Res<PlanetInfoRes>,
    window: Single<&Window>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    planet_assets: Res<PlanetAssets>,
    explorer_assets: Res<ExplorerAssets>,
) {
    //create and load background image through sprites
    let background: Handle<Image> = asset_loader.load("embedded://images/sky.png");

    info!(
        "Queried window size is {} by {}",
        window.width(),
        window.height()
    );

    commands.spawn((Sprite::from_image(background), Background));

    let planet_num = galaxy.planet_num;

    let planet_radius = scale_planet_size(planet_num);

    commands.insert_resource(PlanetSizeRes {
        planet_rad: planet_radius,
        exp_rad: planet_radius * 0.8,
    });

    for (&i, _info) in planets.map.iter() {
        // spawn all the planets in a circle, with even spacing
        // Tau = 2 * pi, so all the planets go around the circle
        let angle = TAU * (i as f32) / (planet_num as f32);

        // extract x and y position via trigonometry
        let x = GALAXY_RADIUS * angle.cos();
        let y = GALAXY_RADIUS * angle.sin();

        let image_index = match planets.map.get_info(i).unwrap().name {
            omc_galaxy::utils::registry::PlanetType::BlackAdidasShoe => 0,
            omc_galaxy::utils::registry::PlanetType::Ciuc => 1,
            omc_galaxy::utils::registry::PlanetType::HoustonWeHaveABorrow => 2,
            omc_galaxy::utils::registry::PlanetType::ImmutableCosmicBorrow => 3,
            omc_galaxy::utils::registry::PlanetType::OneMillionCrabs => 4,
            omc_galaxy::utils::registry::PlanetType::Rustrelli => 5,
            omc_galaxy::utils::registry::PlanetType::RustyCrab => 6,
            omc_galaxy::utils::registry::PlanetType::TheCompilerStrikesBack => 7,
        };

        // Handle is based on Arc, so cloning is fine
        // the modulo with SPRITE_NUM is used to minimize runtime crashes
        // in case the index is out of bounds
        let planet_image_handle = planet_assets.handles[(image_index) % PLANET_SPRITE_NUM].clone();

        // load the planet in the game world
        commands
            .spawn((
                Planet { id: i },
                Sprite {
                    image: planet_image_handle,
                    custom_size: Some(Vec2::splat(planet_radius * 2.)),
                    ..Default::default()
                },
                Transform::from_xyz(x, y, 2.0),
                Pickable::default(),
            ))
            .observe(choose_on_click);

        // create the explorers on the starter planet
        if i == 0 {
            for j in 0..EXP_SPRITE_NUM {
                let explorer_image_handle = explorer_assets.handles[j].clone();
                let (offset_x, offset_y): (f32, f32) = if j == 0 {
                    scale_offset_size(planet_radius, true)
                } else {
                    scale_offset_size(planet_radius, false)
                };
                commands
                    .spawn((
                        Explorer {
                            id: j as u32,
                            state: ExpState::Auto,
                            current_planet: i,
                            position_offset: (offset_x, offset_y),
                        },
                        Sprite {
                            image: explorer_image_handle,
                            custom_size: Some(Vec2::splat(planet_radius * 0.8)),
                            ..Default::default()
                        },
                        Transform::from_xyz(x + offset_x, y - offset_y, 3.0),
                        Pickable::default(),
                    ))
                    .observe(choose_on_click);
            }
        }
    }

    // adds the struct to hold info about the current notification
    commands.insert_resource(ActiveNotification {
        message: None,
        active_time: Timer::from_seconds(1.0, TimerMode::Once),
    });
}
