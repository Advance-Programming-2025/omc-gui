use bevy::prelude::Event;

#[derive(Event)]
pub(crate) struct PlanetDespawn{
    pub planet_id: u32
}