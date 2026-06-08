use bevy::prelude::*;
use crate::ecs::{markers::{NotificationContainer, NotificationText}, events::Notification, resources::ActiveNotification};

pub fn draw_notifications(
    mut commands: Commands
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute, // must get displayed above everything else
            left: Val::Percent(50.0),
            margin: UiRect {
                left: Val::Px(-150.0),
                ..default()
            },
            bottom: Val::Px(20.0),
            padding: UiRect::all(Val::Px(12.0)),
            display: Display::None, // hidden at startup
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
        BorderRadius::all(Val::Px(8.0)),
        NotificationContainer,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new(""),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            NotificationText,
        ));
    });
}

pub fn set_notification(event: On<Notification>, mut active_msg: ResMut<ActiveNotification>) {
    active_msg.message = Some(event.message.clone());
    active_msg.active_time = Timer::from_seconds(1.0, TimerMode::Once);
}

pub fn update_notification(
    time: Res<Time>,
    mut notif: ResMut<ActiveNotification>,
    mut root_query: Query<&mut Node, With<NotificationContainer>>,
    mut text_query: Query<&mut Text, With<NotificationText>>,
) {
    notif.active_time.tick(time.delta());

    // delete the active text if the timer is done
    if notif.active_time.is_finished() {
        notif.message = None;
    }

    let visible = notif.message.is_some();

    // set the notification to active if the message is present
    for mut node in &mut root_query {
        node.display = if visible {
            Display::Flex
        } else {
            Display::None
        };
    }

    // update text
    for mut text in &mut text_query {
        text.0 = notif.message.clone().unwrap_or_default();
    }
}