use std::ops::{Index, IndexMut};

use bevy_ecs::entity::Entity;

use crate::MarkerData;

pub trait EntityMarker: Sync + Send {
    const PLACEHOLDER: Entity = Entity::from_raw(u32::MAX); // TODO: use Entity::PLACEHOLDER when released

    type Storage: Index<usize, Output = Entity> + IndexMut<usize> + Send + Sync;

    fn create_storage() -> Self::Storage
    where
        Self: Sized;

    fn new_data() -> MarkerData<Self>
    where
        Self: Sized;

    fn unit_index(&self) -> usize;
}

pub trait DynamicEntityMarker: EntityMarker {
    fn add(storage: &mut Self::Storage, entity: Entity)
    where
        Self: Sized;
}
