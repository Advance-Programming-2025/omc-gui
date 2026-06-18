use crate::ecs::components::ExpButtonActions;
use crate::ecs::components::{ButtonActions, ListType, RatioButton, UiExplorerText, UiPlanetText};
use crate::ecs::markers::{
    AliveExplorerButton, AlivePlanetActions, AlivePlanetButton, DropdownButton, DropdownLabel,
    DropdownRoot, ExpModeText, ExplorerOnlyButton, GameStateText, InGameUI, LogText,
    ManualExplorer, PlanetOnlyButton, RatioText,
};
use crate::ecs::resources::{GameState, StartupConfig};
use crate::ui::button_bundle;
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

    let list_factory = |(title, dropdown_el, width, max_h): (Text, ListType, f32, Option<Val>)| {
        (
            (
                Node {
                    width: Val::Px(width),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
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
                        overflow: Overflow::scroll_y(),
                        max_height: max_h.unwrap_or(Val::Auto),
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::new(0.15, 0.15, 0.15, 1.))),
                    dropdown_el
                )
            ],
        )
    };

    // root node
    commands.spawn((root, InGameUI)).with_children(|parent| {
        // side menu panel
        parent.spawn(side_menu_container).with_children(|parent| {
            // menu title
            parent.spawn(title_text);

            // list of planet info: id, status, energy cells, rocket available
            // list of explorer info: id, status, visited planet, bag
            parent.spawn(button_row.clone()).with_children(|parent| {
                parent.spawn((
                    Text::new("choose an entity to display its characteristics!"),
                    UiPlanetText::Name,
                ));
                parent
                    .spawn((
                        Node {
                            display: Display::None,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        PlanetOnlyButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            button_bundle(Text::new("<"), 20.),
                            ButtonActions::PrevPlanet,
                        ));
                        parent.spawn((
                            Text::new(""),
                            TextLayout {
                                justify: Justify::Center,
                                ..default()
                            },
                            Node {
                                width: Val::Percent(60.0),
                                ..default()
                            },
                            UiPlanetText::Id,
                        ));
                        parent.spawn((
                            button_bundle(Text::new(">"), 20.),
                            ButtonActions::NextPlanet,
                        ));
                    });
                parent.spawn((
                    Text::new(""),
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                    Node {
                        display: Display::None,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    PlanetOnlyButton,
                    UiPlanetText::Status,
                ));
                parent.spawn((
                    Text::new(""),
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                    Node {
                        display: Display::None,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    PlanetOnlyButton,
                    UiPlanetText::Energy,
                ));
                parent.spawn((
                    Text::new(""),
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                    Node {
                        display: Display::None,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    PlanetOnlyButton,
                    UiPlanetText::Rocket,
                ));
                parent
                    .spawn((
                        Node {
                            display: Display::None,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ExplorerOnlyButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            button_bundle(Text::new("<"), 20.),
                            ExpButtonActions::PrevExplorer,
                        ));
                        parent.spawn((
                            Text::new(""),
                            TextLayout {
                                justify: Justify::Center,
                                ..default()
                            },
                            Node {
                                width: Val::Percent(60.0),
                                ..default()
                            },
                            UiExplorerText::Id,
                        ));
                        parent.spawn((
                            button_bundle(Text::new(">"), 20.),
                            ExpButtonActions::NextExplorer,
                        ));
                    });
                parent.spawn((
                    Text::new(""),
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                    Node {
                        display: Display::None,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ExplorerOnlyButton,
                    UiExplorerText::Status,
                ));
                parent.spawn((
                    Text::new(""),
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                    Node {
                        display: Display::None,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ExplorerOnlyButton,
                    UiExplorerText::Visiting,
                ));
                parent.spawn((
                    Text::new(""),
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                    Node {
                        display: Display::None,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ExplorerOnlyButton,
                    UiExplorerText::ResourceBag,
                ));
            });

            // planet specific buttons: send sunray and asteroids
            // shown only if a planet has been selected
            parent
                .spawn((
                    Node {
                        display: Display::None,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..button_row.clone()
                    },
                    PlanetOnlyButton,
                    AlivePlanetActions,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        button_bundle(Text::new("Send asteroid"), 50.),
                        ButtonActions::ManualAsteroid,
                    ));
                    parent.spawn((
                        button_bundle(Text::new("Send sunray"), 50.),
                        ButtonActions::ManualSunray,
                    ));
                });

            //explorer menu
            parent
                .spawn((
                    Node {
                        display: Display::None,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..button_row.clone()
                    },
                    ExplorerOnlyButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        button_bundle(Text::new("Explorer mode: "), 50.),
                        ExpButtonActions::ExpModeChange,
                        ExpModeText,
                    ));
                    // show the following stuff only if the explorer is in manual mode
                    parent
                        .spawn((
                            Node {
                                width: percent(100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                display: Display::None,
                                ..default()
                            },
                            ManualExplorer,
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
                                        None,
                                    )));
                                    parent.spawn(list_factory((
                                        Text::new("make:"),
                                        ListType::ComplexList,
                                        110.,
                                        None,
                                    )));
                                });

                            parent.spawn(list_factory((
                                Text::new("select destination"),
                                ListType::MoveList,
                                220.,
                                Some(Val::Px(300.)),
                            )));
                        });
                });

            // random selection buttons
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        button_bundle(Text::new("Random Explorer"), 50.),
                        ButtonActions::RandomExplorer,
                        AliveExplorerButton,
                    ));
                    parent.spawn((
                        button_bundle(Text::new("Random Planet"), 50.),
                        ButtonActions::RandomPlanet,
                        AlivePlanetButton,
                    ));
                });
        });
    });
}

pub(crate) fn draw_game_options_menu(mut commands: Commands, ratio: Res<StartupConfig>) {
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
            height: Val::Percent(75.),
            overflow: Overflow::scroll_y(),
            ..default()
        },
    );

    let title_text = Text::new("Galaxy Menu");

    // 1. Root node
    commands.spawn((root, InGameUI)).with_children(|parent| {
        // 2. Side menu panel
        parent.spawn(side_menu_container).with_children(|parent| {
            // 3a. Menu title
            parent.spawn(title_text);
            parent.spawn((Text::new("Game state: Waiting start"), GameStateText));

            // 3b. Button Row
            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((
                    button_bundle(Text::new("Start"), 50.),
                    ButtonActions::StartGame,
                ));

                //4b. button 2
                parent.spawn((
                    button_bundle(Text::new("Pause"), 50.),
                    ButtonActions::StopGame,
                ));
            });

            parent.spawn(button_row.clone()).with_children(|parent| {
                parent.spawn((button_bundle(Text::new("Nuke"), 50.), ButtonActions::Nuke));

                //4b. button 2
                parent.spawn((button_bundle(Text::new("Blind"), 50.), ButtonActions::Blind));
            });

            parent.spawn((
                Text::new(format!(
                    "Sunray to asteroid ratio: {}%",
                    ratio.into_inner().ratio
                )),
                RatioText,
            ));

            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((button_bundle(Text::new("Less"), 50.), RatioButton::Decrease));

                //4b. button 2
                parent.spawn((button_bundle(Text::new("More"), 50.), RatioButton::Increase));
            });

            parent.spawn(log_square).with_children(|parent| {
                parent.spawn((Text::new(""), LogText));
            });
        });
    });
}

pub(crate) fn update_game_state_text(
    game_state: Res<State<GameState>>,
    state_text: Query<(&mut Text, &GameStateText)>,
) {
    // avoid computation if the state hasn't changed
    if !game_state.is_changed() {
        return;
    }

    debug!("updating game state text to {:?}", game_state);
    let inner_state = game_state.into_inner();

    for (mut text, _) in state_text {
        **text = format!("Game state: {}", inner_state.get());
    }
}
