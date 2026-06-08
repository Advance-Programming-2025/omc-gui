use crate::ecs::markers::Background;
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
