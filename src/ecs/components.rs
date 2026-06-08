use bevy::ecs::component::Component;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};

use crate::ecs::resources::ExpState;

// Galaxy-centric components
#[derive(Component)]
pub(crate) struct Planet {
    pub id: u32,
}

#[derive(Component)]
pub(crate) struct Explorer {
    pub id: u32,
    pub state: ExpState,
    pub current_planet: u32,
    pub position_offset: (f32, f32),
}

#[derive(Component)]
pub(crate) struct Edge {
    pub connects: (u32, u32),
}

/// Button associated actions
#[derive(Component)]
pub enum ButtonActions {
    StartGame,
    StopGame,
    ManualAsteroid,
    ManualSunray,
    Blind,
    Nuke,
}

/// Button associated actions
#[derive(Component)]
pub enum ExpButtonActions {
    ExpModeChange,
    CreateBasic(BasicResourceType),
    CreateComplex(ComplexResourceType),
}

/// Planet info marker component
#[derive(Component)]
pub enum UiPlanetText {
    Name,
    Id,
    Status,
    Energy,
    Rocket,
}

/// Explorer info marker component
#[derive(Component)]
pub enum UiExplorerText {
    Id,
    Visiting,
    Status,
    ResourceBag,
}


#[derive(Component)]
pub enum ListType {
    MoveList,
    BasicList,
    ComplexList,
}

#[derive(Component)]
pub struct DropdownItem {
    pub explorer_id: u32,
    pub planet_id: u32,
}

#[derive(Component)]
pub enum RatioButton {
    Increase,
    Decrease
}

#[derive(Component)]
pub enum StartMenuButton {
    ChooseFile,
    StartGame
}