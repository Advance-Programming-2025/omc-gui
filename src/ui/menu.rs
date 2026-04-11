use crate::ecs::components::{
    ButtonActions, DropdownButton, DropdownLabel, DropdownRoot, ExplorerOnlyButton, ListType,
    LogText, PlanetOnlyButton, UiExplorerText, UiPlanetText,
};
use crate::ecs::components::{ExpButtonActions, GameStateText};
use crate::ecs::resources::GameState;
use bevy::prelude::*;

///Draws the menu that holds the list of all explorers and planets
pub(crate) fn draw_entity_info_menu(mut commands: Commands) {
    let root = Node {
        width: Val::Px(350.0),
        height: Val::Percent(100.0),
        // Left aligned
        justify_content: JustifyContent::FlexStart,
        ..default()
    };

    let side_menu_container = (
        BackgroundColor {
            0: Color::Srgba(Srgba {
                red: 0.12,
                green: 0.18,
                blue: 0.24,
                alpha: 0.7,
            }),
        },
        Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
    );

    let button_row = Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(20.0)),
        ..default()
    };

    let title_text = (
        Text::new("Selected Entity:"),
        TextFont {
            font_size: 32.,
            ..default()
        },
    );

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

    let list_factory = |(title, dropdown_el, width): (Text, ListType, f32)| {
        (
            (
                Node {
                    width: Val::Px(width),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ExplorerOnlyButton,
                DropdownRoot,
            ),
            children![
                (
                    Node {
                        height: Val::Px(32.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.07, 0.30, 0.53)),
                    DropdownButton,
                    children![(
                        title,
                        TextFont {
                            font_size: 16.0,
                            ..Default::default()
                        },
                        DropdownLabel,
                    )]
                ),
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::new(0.15, 0.15, 0.15, 1.))),
                    dropdown_el
                )
            ],
        )
    };

    // root node
    commands.spawn(root).with_children(|parent| {
        // side menu panel
        parent.spawn(side_menu_container).with_children(|parent| {
            // menu title
            parent.spawn(title_text);

            // list of planet info: id, status, energy cells, rocket available
            // list of explorer info: id, status, visited planet, bag
            parent.spawn(button_row.clone()).with_children(|parent| {
                parent.spawn((Text::new("choose a planet!"), UiPlanetText::Name));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    PlanetOnlyButton,
                    UiPlanetText::Id,
                ));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    PlanetOnlyButton,
                    UiPlanetText::Status,
                ));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    PlanetOnlyButton,
                    UiPlanetText::Energy,
                ));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    PlanetOnlyButton,
                    UiPlanetText::Rocket,
                ));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    ExplorerOnlyButton,
                    UiExplorerText::Id,
                ));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    ExplorerOnlyButton,
                    UiExplorerText::Status,
                ));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    ExplorerOnlyButton,
                    UiExplorerText::Visiting,
                ));
                parent.spawn((
                    Text::new(""),
                    Visibility::Hidden,
                    ExplorerOnlyButton,
                    UiExplorerText::ResourceBag,
                ));
            });

            // planet specific buttons: send sunray and asteroids
            // shown only if a planet has been selected
            parent
                .spawn((
                    button_row.clone(),
                    Visibility::Hidden, //only in the beginning
                    PlanetOnlyButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        button_factory(Text::new("Send asteroid")),
                        ButtonActions::ManualAsteroid,
                    ));
                    parent.spawn((
                        button_factory(Text::new("Send sunray")),
                        ButtonActions::ManualSunray,
                    ));
                });

            //explorer menu
            parent.spawn(button_row.clone()).with_children(|parent| {
                parent.spawn((
                    button_factory(Text::new("Explorer mode: TBD")),
                    ExpButtonActions::ExpModeChange,
                    ExplorerOnlyButton,
                ));
                // TODO show the following stuff only if the explorer is in manual mode
                parent
                    .spawn((
                        Node {
                            width: percent(100.0),
                            //debug
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Visibility::Hidden,
                        ExplorerOnlyButton,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(list_factory((
                                    Text::new("create:"),
                                    ListType::BasicList,
                                    110.,
                                )));
                                parent.spawn(list_factory((
                                    Text::new("make:"),
                                    ListType::ComplexList,
                                    110.,
                                )));
                            });

                        parent.spawn(list_factory((
                            Text::new("select destination"),
                            ListType::MoveList,
                            220.,
                        )));
                    });
            });
        });
    });
}

pub(crate) fn draw_game_options_menu(mut commands: Commands) {
    let root = Node {
        width: Val::Px(350.),
        height: Val::Percent(100.0),
        // Right aligned
        justify_content: JustifyContent::FlexEnd,
        margin: UiRect {
            left: Val::Auto,
            ..default()
        },
        ..default()
    };

    let side_menu_container = (
        BackgroundColor {
            0: Color::Srgba(Srgba {
                red: 0.12,
                green: 0.18,
                blue: 0.24,
                alpha: 0.7,
            }),
        },
        Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
    );

    let button_row = Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        ..default()
    };

    let log_square = (
        BackgroundColor(Color::Srgba(Srgba {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.6,
        })),
        Node {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            height: Val::Percent(50.),
            overflow: Overflow::scroll_y(),
            ..default()
        },
    );

    let title_text = Text::new("Galaxy Menu");

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

    // 1. Root node
    commands.spawn(root).with_children(|parent| {
        // 2. Side menu panel
        parent.spawn(side_menu_container).with_children(|parent| {
            // 3a. Menu title
            parent.spawn(title_text);
            parent.spawn((Text::new("Game state: Waiting start"), GameStateText));

            // 3b. Button Row
            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((button_factory(Text::new("Start")), ButtonActions::StartGame));

                //4b. button 2
                parent.spawn((button_factory(Text::new("Pause")), ButtonActions::StopGame));
            });

            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((
                    button_factory(Text::new("Restart")),
                    ButtonActions::StartGame,
                ));

                //4b. button 2
                parent.spawn((button_factory(Text::new("Blind")), ButtonActions::Blind));
            });

            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((button_factory(Text::new("Nuke")), ButtonActions::Nuke));

                //4b. button 2
                parent.spawn((
                    button_factory(Text::new("Explorer Messages")),
                    ButtonActions::StopGame,
                ));
            });
            parent.spawn(log_square).with_children(|parent| {
                parent.spawn((Text::new(""), LogText));
            });
        });
    });
}

pub(crate) fn update_game_state_text(
    game_state: Res<GameState>,
    state_text: Query<(&mut Text, &GameStateText)>,
) {
    // avoid computation if the state hasn't changed
    if !game_state.is_changed() {
        return;
    }

    debug!("updating game state text to {:?}", game_state);
    let inner_state = game_state.into_inner();

    for (mut text, _) in state_text {
        **text = format!("Game state: {:?}", inner_state);
    }
}
