use bevy::prelude::*;

use crate::ecs::{components::LogText, resources::LogTextRes};

pub(crate) fn update_logs(log_text: &mut ResMut<LogTextRes>, event_to_push: String) {
    log_text.text.push_front(event_to_push);
}

// TODO find a more performant approach.
// Fine now because time is of the essence
// and the average game isn't that long, but this
// allocates a new string everytime a log event
// happens. That's a lot of memory gone for nothing!

// alternative approach: spawn the log text
// directly, using commands.spawn()
pub(crate) fn log_text(logs: ResMut<LogTextRes>, mut log_node: Single<&mut Text, With<LogText>>) {
    if !logs.is_changed() {
        return;
    };
    let mut bruh = String::new();

    for log_event in logs.text.iter() {
        bruh += log_event;
    }

    log_node.0 = bruh;
}
