use common_game::components::resource::{BasicResourceType, ComplexResourceType, ResourceType};

use crate::ecs::resources::EntityClickRes;

pub(crate) trait Visible {
    fn is_selected(entity: &EntityClickRes) -> bool;
}

pub(crate) trait Printable {
    fn to_print(&self) -> String;
}

impl Printable for BasicResourceType {
    fn to_print(&self) -> String {
        match self {
            BasicResourceType::Oxygen => format!("Oxygen"),
            BasicResourceType::Hydrogen =>format!("Hydrogen"),
            BasicResourceType::Carbon => format!("Carbon"),
            BasicResourceType::Silicon => format!("Silicon"),
        }
    }
}

impl Printable for ComplexResourceType {
    fn to_print(&self) -> String {
        match self {
            ComplexResourceType::AIPartner => format!("AI"),
            ComplexResourceType::Diamond => format!("Diamond"),
            ComplexResourceType::Dolphin =>format!("Dolphin"),
            ComplexResourceType::Water => format!("Water"),
            ComplexResourceType::Life => format!("Life"),
            ComplexResourceType::Robot => format!("Robot")
        }
    }
}

impl Printable for ResourceType {
    fn to_print(&self) -> String {
        match self {
            ResourceType::Basic(basic_resource_type) => basic_resource_type.to_print(),
            ResourceType::Complex(complex_resource_type) => complex_resource_type.to_print(),
        }
    }
}