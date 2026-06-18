use crate::{
    ecs::markers::Background,
    utils::constants::{PLANET_SCALING_A, PLANET_SCALING_B, PLANET_SCALING_C},
};
use bevy::prelude::*;

/// Scale background image to fit the screen, regardless of size or
/// fractional scaling.
pub fn scale_background(
    window: Single<&Window>,
    image: Res<Assets<Image>>,
    bg_query: Single<(&Sprite, &mut Transform), With<Background>>,
) {
    let (sprite, mut bg_transform) = bg_query.into_inner();

    // early return if the image hasn't been loaded yet
    if let Some(background) = image.get(&sprite.image) {
        let size = background.size_f32();

        let scale_x = window.width() / size.x;
        let scale_y = window.height() / size.y;

        let scale = scale_x.max(scale_y);

        bg_transform.scale = Vec3::splat(scale);
    }
}

/// returns the appropriate radius for the planet size.
///
/// This scaling is based on the amount of present planets in a galaxy and follows an
/// exponential function that was derived empirically.
///
/// * `planet_num` = the number of planets present in the galaxy
pub(crate) fn scale_planet_size(planet_num: usize) -> f32 {
    let exponent = PLANET_SCALING_B * (planet_num as f32);
    PLANET_SCALING_A * f32::exp(exponent) + PLANET_SCALING_C
}

/// Offets the position of the explorer on the planet, given the "side" of the orientation
///
/// * `planet_size` = the radius of a planet, as computed by [scale_planet_size]
/// * `positive_offset` = whether the offset sign should be positive (top-right) or negative (bottom-right)
pub(crate) fn scale_offset_size(planet_size: f32, positive_offset: bool) -> (f32, f32) {
    if positive_offset {
        (planet_size / 2., planet_size / 2.)
    } else {
        (-planet_size / 2., -planet_size / 2.)
    }
}
