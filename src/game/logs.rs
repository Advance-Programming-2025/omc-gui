use bevy::prelude::*;

use crate::ecs::{markers::LogText, resources::LogTextRes};

/// Pushes a loggable action to the log timeline
pub(crate) fn update_logs(log_text: &mut ResMut<LogTextRes>, event_to_push: String) {
    log_text.text.push_front(event_to_push);
}

/// Updates the log region in the right-hand menu
pub(crate) fn log_text(logs: ResMut<LogTextRes>, mut log_node: Single<&mut Text, With<LogText>>) {
    if !logs.is_changed() {
        return;
    };

    log_node.0.clear();

    for event in &logs.text {
        log_node.0.push_str(event);
    }
}
