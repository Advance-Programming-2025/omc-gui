use bevy::prelude::*;

use crate::ecs::{components::{CurrentPathText, StartMenuButton, StartMenuUI}, resources::{GameState, StartupConfig}};

pub(crate) fn start_splash (mut commands: Commands) {

    let config_file = StartupConfig::default();
    let starter_file = config_file.topology_path.clone();
    commands.insert_resource(config_file);

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
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    }, StartMenuUI)).with_children(|container| {
        container.spawn(Text::new("One Million Crabs: Galaxy Visualizer"));
        container.spawn((
            button_factory(Text::new("Choose file")),
            StartMenuButton::ChooseFile
        ));
        container.spawn((
            Text::new(format!("Current path: {}", starter_file.display())),
            CurrentPathText
        ));

        container.spawn((
            button_factory(Text::new("Start game")),
            StartMenuButton::StartGame
        ));
    });
}

pub(crate) fn start_menu_actions (
    mut action_query: Query<(&Interaction, &StartMenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
    mut config_res: ResMut<StartupConfig>,
    mut text_query: Query<(&mut Text, &CurrentPathText)>,
) {
    for (&interaction, action_type) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action_type {
                StartMenuButton::ChooseFile => {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Galaxy", &["txt"])
                        .pick_file() {
                        config_res.topology_path = path;
                        for (mut text, _) in &mut text_query {
                            **text = format!(
                                "Current path: {}",
                                config_res.topology_path.display()
                            );
                        }
                    };
                },
                StartMenuButton::StartGame => {
                    state.set(GameState::Playing);
                },
            }
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
