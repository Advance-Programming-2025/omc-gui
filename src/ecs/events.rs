use bevy::prelude::*;

/// Event triggered when a planet dies after getting destroyed by an asteroid
#[derive(Event)]
pub(crate) struct PlanetDespawn {
    pub planet_id: u32,
}

/// Struct to differentiate between the two possible celestial bodies
#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum CelestialBody {
    Sunray,
    Asteroid,
}

/// Event triggered when a celestial body is spawned and has yet to reach a planet
#[derive(Event, Component)]
pub(crate) struct Celestial {
    pub kind: CelestialBody,
    pub planet_id: u32,
}

/// Event triggered when an explorer needs to move, either automatically or by the user's request
#[derive(Event)]
pub(crate) struct MoveExplorerEvent {
    pub id: u32,
    pub destination: u32,
}

/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub(crate) struct Scroll {
    pub entity: Entity,
    pub scroll_delta: Vec2,
}

#[derive(Event)]
pub struct Notification {
    pub message: String,
}
