use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    ecs::{
        components::StartMenuButton,
        markers::{CurrentPathText, StartMenuUI, StartPlanetText, StartRatioText},
        resources::{GameState, StartupConfig},
    },
    ui::button_bundle,
};

pub(crate) fn start_splash(mut commands: Commands) {
    let config_file = StartupConfig::default();
    let starter_file = config_file.topology_path.clone();
    let starter_ratio = config_file.ratio;
    let starter_planets = config_file.random_planets;
    commands.insert_resource(config_file);

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
            StartMenuUI,
        ))
        .with_children(|container| {
            container.spawn((
                Text::new("One Million Crabs: Galaxy Visualizer"),
                TextFont {
                    font_size: 36.,
                    ..Default::default()
                },
            ));
            container.spawn((
                button_bundle(Text::new("Choose file"), 50.),
                StartMenuButton::ChooseFile,
            ));

            container.spawn((
                Text::new(format!("Current path: {}", starter_file.unwrap_or(PathBuf::default()).display())),
                CurrentPathText,
            ));

            container
                .spawn(Node {
                    width: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|ratio_container| {
                    ratio_container.spawn((
                        button_bundle(Text::new("-"), 10.),
                        StartMenuButton::StartRatioLess,
                    ));

                    ratio_container.spawn((
                        Text::new(format!("Sunray/asteroid ratio: {}%", starter_ratio)),
                        StartRatioText,
                    ));

                    ratio_container.spawn((
                        button_bundle(Text::new("+"), 10.),
                        StartMenuButton::StartRatioMore,
                    ));
                });

            container
                .spawn(Node {
                    width: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|planet_container| {
                    planet_container.spawn((
                        button_bundle(Text::new("-"), 10.),
                        StartMenuButton::RandomPlanetLess,
                    ));

                    planet_container.spawn((
                        Text::new(format!("Random galaxy planet count: {}", starter_planets)),
                        StartPlanetText,
                    ));

                    planet_container.spawn((
                        button_bundle(Text::new("+"), 10.),
                        StartMenuButton::RandomPlanetMore,
                    ));
                });

            container.spawn((
                button_bundle(Text::new("Start from topology"), 50.),
                StartMenuButton::StartGame,
            ));

            container.spawn((
                button_bundle(Text::new("Start from random galaxy"), 50.),
                StartMenuButton::StartRandom,
            ));
        });
}

pub(crate) fn start_menu_actions(
    mut action_query: Query<(&Interaction, &StartMenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
    mut config_res: ResMut<StartupConfig>,
    mut text_query: Query<(&mut Text, &CurrentPathText)>,
    mut label_texts: ParamSet<(
        Query<(&mut Text, &StartRatioText), Without<CurrentPathText>>,
        Query<(&mut Text, &StartPlanetText), Without<CurrentPathText>>,
    )>,
) {
    for (&interaction, action_type) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action_type {
                StartMenuButton::ChooseFile => {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Galaxy", &["txt"])
                        .pick_file()
                    {
                        let to_display = path.clone();
                        config_res.topology_path = Some(path);
                        for (mut text, _) in &mut text_query {
                            **text =
                                format!("Current path: {}", to_display.display());
                        }
                    };
                }
                StartMenuButton::StartGame => {
                    state.set(GameState::Playing);
                }
                StartMenuButton::StartRatioLess => {
                    config_res.ratio = i32::max(0, config_res.ratio - 5);
                    for (mut text, _) in &mut label_texts.p0() {
                        **text = format!("Sunray/asteroid ratio: {}%", config_res.ratio);
                    }
                }
                StartMenuButton::StartRatioMore => {
                    config_res.ratio = i32::min(100, config_res.ratio + 5);
                    for (mut text, _) in &mut label_texts.p0() {
                        **text = format!("Sunray/asteroid ratio: {}%", config_res.ratio);
                    }
                }
                StartMenuButton::RandomPlanetLess => {
                    config_res.random_planets = config_res.random_planets.saturating_sub(1).max(2);
                    for (mut text, _) in &mut label_texts.p1() {
                        **text = format!("Random galaxy planet count: {}", config_res.random_planets);
                    }
                }
                StartMenuButton::RandomPlanetMore => {
                    config_res.random_planets = config_res.random_planets.saturating_add(1);
                    for (mut text, _) in &mut label_texts.p1() {
                        **text = format!("Random galaxy planet count: {}", config_res.random_planets);
                    }
                }
                StartMenuButton::StartRandom => {
                    config_res.topology_path = None;
                    state.set(GameState::Playing);
                },
            }
        }
    }
}

pub(crate) fn cleanup_start_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<StartMenuUI>>,
) {
    for ui_entity in menu_query {
        commands.entity(ui_entity).despawn();
    }
}
