use bevy::{ecs::system::Res, log::info, state::state::State};

use crate::ecs::resources::{GameState, OrchestratorResource};

pub(crate) fn log_state_entry(state: Res<State<GameState>>, orchestrator: Option<Res<OrchestratorResource>>) {
    if let Some(res) = orchestrator.as_ref() {
        let mut debug_str = String::new();
        for message in res.orchestrator.gui_messages.iter().clone() {
            let msg = format!("{:?}", message);
            debug_str.push_str(&msg);
        }
        info!(
                "→ EVENT QUEUE: entered state {:?}, pending orchestrator events: {}",
                state.get(),
                debug_str
            );
    }
}