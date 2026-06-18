use bevy::ecs::component::Component;

use crate::utils::traits::Visible;

/// Button visibility marker component; makes it so that the buttons tagged
/// with this component are rendered only when a planet is selected.
#[derive(Component)]
pub struct PlanetOnlyButton;

impl Visible for PlanetOnlyButton {
    fn is_selected(entity: &super::resources::EntityClickRes) -> bool {
        entity.planet.is_some()
    }
}

/// Button visibility marker component; makes it so that the buttons tagged
/// with this component are rendered only when an explorer is selected.
#[derive(Component)]
pub struct ExplorerOnlyButton;

impl Visible for ExplorerOnlyButton {
    fn is_selected(entity: &super::resources::EntityClickRes) -> bool {
        entity.explorer.is_some()
    }
}

/// Marker for the buttons specific to the explorer's manual mode
#[derive(Component)]
pub struct ManualExplorer;

/// Marker for the text that displays the game state in the top right
#[derive(Component)]
pub struct GameStateText;

/// Marker component for any loggable action
#[derive(Component)]
pub struct LogText;

/// Marker for the root UI container of all dropdown menus
#[derive(Component)]
pub struct DropdownRoot;

/// Marker for a button inside a dropdown menu
#[derive(Component)]
pub struct DropdownButton;

/// Marker for a label inside a dropdown menu
#[derive(Component)]
pub struct DropdownLabel;

/// Marker for the space background; needed so it can be scaled
#[derive(Component)]
pub struct Background;

/// Marker for the root UI container of the notification space
#[derive(Component)]
pub struct NotificationContainer;

/// Marker for the message to be displayed in the notification
#[derive(Component)]
pub struct NotificationText;

/// Marker for the sunray to asteroid ratio text
#[derive(Component)]
pub struct RatioText;

/// Marker for the root UI container of the start menu page
#[derive(Component)]
pub struct StartMenuUI;

/// Marker for the galaxy configuration path that is currently selected
#[derive(Component)]
pub struct CurrentPathText;

/// Marker for the text for the initial sunray to asteroid ratio
#[derive(Component)]
pub struct StartRatioText;

/// Marker for the text for the number of planets in a random galaxy
#[derive(Component)]
pub struct StartPlanetText;

/// Marker for the text of the button that signals the explorer AI mode: auto or manual
#[derive(Component)]
pub struct ExpModeText;

/// Marker for the button that selects a random alive explorer
#[derive(Component)]
pub struct AliveExplorerButton;

/// Marker for the button that selects a random alive planet
#[derive(Component)]
pub struct AlivePlanetButton;

/// Marker for planet action buttons that should be hidden when the planet is dead
#[derive(Component)]
pub struct AlivePlanetActions;
