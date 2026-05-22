use bevy::prelude::*;

use crate::ecs::{components::{StartMenuButton, StartMenuUI}, resources::GameState};

pub(crate) fn start_splash (mut commands: Commands) {

    // TODO should probably just turn this into a function instead of copy pasting a closure...
    let button_factory = |text: Text| {
        (
            Button,
            BackgroundColor(Color::srgb(0.67, 0.30, 0.53)),
            Node {
                width: Val::Percent(50.),
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
    };

    commands.spawn((Node {
        width: Val::Percent(50.),
        height: Val::Percent(50.),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    }, StartMenuUI)).with_children(|container| {
        container.spawn(Text::new("Start menu: work in progress"));
        container.spawn((
            button_factory(Text::new("Start game")),
            StartMenuButton
        ));
    });
}

pub(crate) fn start_menu_actions (
    mut action_query: Query<(&Interaction, &StartMenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>
) {
    for (&interaction, _) in &mut action_query {
        if interaction == Interaction::Pressed {
            state.set(GameState::Playing);
        }
    }
}

pub(crate) fn cleanup_start_menu (
    mut commands: Commands,
    menu_query: Query<Entity, With<StartMenuUI>>,
) {
    for ui_entity in menu_query {
        commands.entity(ui_entity).despawn();
    }
}