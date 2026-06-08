use crate::ecs::resources::EntityClickRes;

pub(crate) trait Visible {
    fn is_selected(entity: &EntityClickRes) -> bool;
}