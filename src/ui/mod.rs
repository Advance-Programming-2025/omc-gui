use bevy::prelude::*;

pub mod buttons;
pub mod dropdown;
pub mod game_over;
pub mod menu;
pub mod notification;
pub mod scroll;
pub mod start;
pub mod visibility;

fn button_bundle(text: Text, width: f32) -> impl Bundle {
    (
        Button,
        BackgroundColor(Color::srgb(0.67, 0.30, 0.53)),
        Node {
            width: Val::Percent(width),
            height: Val::Px(40.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(15.)),
        children![(
            text,
            TextFont {
                font_size: 12.,
                ..default()
            },
            TextLayout {
                justify: Justify::Center,
                ..default()
            },
            TextColor(Color::srgb(0.97, 0.98, 0.96))
        )],
    )
}
