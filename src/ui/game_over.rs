use bevy::prelude::*;

use crate::ecs::markers::{GameOverButton, GameOverUI};
use crate::ui::button_bundle;

/// Screen to signal game over, appears when all planets die
pub(crate) fn spawn_game_over_splash(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.7)),
            GameOverUI,
        ))
        .with_children(|container| {
            container.spawn((
                Text::new("Game Over"),
                TextFont {
                    font_size: 48.,
                    ..Default::default()
                },
            ));

            container.spawn((button_bundle(Text::new("Quit"), 50.), GameOverButton));
        });
}

/// Handle the "Quit" button tha shuts the application down
pub(crate) fn game_over_actions(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<GameOverButton>)>,
    mut exit: MessageWriter<AppExit>,
) {
    for &interaction in &mut interaction_query {
        if interaction == Interaction::Pressed {
            exit.write(AppExit::Success);
        }
    }
}
