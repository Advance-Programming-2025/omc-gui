use bevy::ecs::component::Component;

#[derive(Component)]
pub(crate) struct Planet {
    pub id: u32,
}

#[derive(Component)]
pub(crate) struct Explorer {
    pub id: u32,
    pub current_planet: u32,
}

#[derive(Component)]
pub(crate) struct Edge {
    pub connects: (u32, u32),
}

#[derive(Component)]
pub enum ButtonActions {
    StartGame,
    StopGame,
    ManualAsteroid,
    ManualSunray,
    Blind,
    Nuke,
}

#[derive(Component)]
pub enum UiPlanetText {
    Name,
    Id,
    Energy,
    Rocket,
    // ResourceList,
    // ExplorerList,
}

#[derive(Component)]
pub enum UiExplorerText {
    Id,
    Visiting,
    Status,
    ResourceBag,
}

#[derive(Component)]
pub struct LogText;
