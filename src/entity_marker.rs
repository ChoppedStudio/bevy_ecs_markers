use bevy_ecs::{entity::Entity, system::Resource};

pub trait EntityMarker: Sync + Send {
    const PLACEHOLDER: Entity = Entity::from_raw(u32::MAX); // TODO: use Entity::PLACEHOLDER when released

    type MarkerData: Resource;

    fn new_data() -> Self::MarkerData
    where
        Self: Sized;

    fn unit_index(&self) -> usize;
}
