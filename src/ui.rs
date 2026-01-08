use bevy::prelude::*;

pub fn setup_ui(commands: Commands) {
    draw_game_options_menu(commands);
}

fn draw_game_options_menu(
    mut commands: Commands
) {
    let root = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        // Right aligned
        justify_content: JustifyContent::FlexEnd, 
        ..default()
    };

    let side_menu_container = (
        BackgroundColor{
            0: Color::Srgba(Srgba { red: 0.12, green: 0.18, blue: 0.18, alpha: 0.8 })
        },
        Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
    });

    let title_text = Text::new("Galaxy Menu");

    let test_button = (Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)));

    let button_text = Text::new("Launch Asteroid");
    
    // 1. Root node
    commands.spawn(root)
        .with_children(|parent| {
        // 2. Side menu panel
        parent.spawn(side_menu_container)
        .with_children(|parent| {
            // 3a. Menu title
            parent.spawn(title_text);
            
            // 3b. Button
            parent.spawn(test_button)
            .with_children(|parent| {
                // 4. Button text
                parent.spawn(button_text);
            });
        });
    });
}

pub fn button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (&interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                println!("Button Pressed!");
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}