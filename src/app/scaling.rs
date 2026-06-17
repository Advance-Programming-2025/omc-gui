use crate::{ecs::markers::Background, utils::constants::{PLANET_SCALING_A, PLANET_SCALING_B, PLANET_SCALING_C}};
use bevy::prelude::*;

pub fn scale_background(
    window: Single<&Window>,
    image: Res<Assets<Image>>,
    single: Single<(&Sprite, &mut Transform), With<Background>>,
) {
    let (sprite, mut bg) = single.into_inner();

    // early return if the image hasn't been loaded yet
    if let Some(background) = image.get(&sprite.image) {
        let size = background.size_f32();

        let scale_x = window.width() / size.x;
        let scale_y = window.height() / size.y;

        let scale = scale_x.max(scale_y);

        bg.scale = Vec3::splat(scale);
    }
}

pub(crate) fn scale_planet_size(planet_num: usize) -> f32 { 
    // PLANET_SCALING_BASE + (PLANET_SCALING_FACTOR/(planet_num as f32)) 
    let exponent = PLANET_SCALING_B * (planet_num as f32);
    PLANET_SCALING_A * f32::exp(exponent) + PLANET_SCALING_C
} 

pub(crate) fn scale_offset_size(planet_size: f32, positive_offset: bool) -> (f32,f32) { 
    if positive_offset {
        (planet_size / 2., planet_size / 2.)
    } else {
        (-planet_size / 2., -planet_size / 2.)
    }
}
