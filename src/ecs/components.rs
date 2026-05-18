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

/// Button visibility marker component;
/// makes it so that the buttons tagged
/// with this component are rendered only
/// when a planet is selected.
#[derive(Component)]
pub struct PlanetOnlyButton;

#[derive(Component)]
pub struct ExplorerOnlyButton;

#[derive(Component)]
pub struct ManualExplorer;

#[derive(Component)]
pub struct GameStateText;

/// Explorer info marker component
#[derive(Component)]
pub enum UiExplorerText {
    Id,
    Visiting,
    Status,
    ResourceBag,
}

/// Marker component for any loggable action
#[derive(Component)]
pub struct LogText;

#[derive(Component)]
pub struct DropdownRoot;

#[derive(Component)]
pub struct DropdownButton;

#[derive(Component)]
pub struct DropdownLabel;

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
pub struct Background;

#[derive(Component)]
pub struct NotificationContainer;

#[derive(Component)]
pub struct NotificationText;