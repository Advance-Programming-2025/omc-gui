use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct PlanetDespawn {
    pub planet_id: u32,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum CelestialBody {
    Sunray,
    Asteroid,
}

#[derive(Event, Component)]
pub(crate) struct Celestial {
    pub kind: CelestialBody,
    pub planet_id: u32,
}

/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub(crate) struct Scroll {
    pub entity: Entity,
    /// Scroll delta in logical coordinates.
    pub delta: Vec2,
}
