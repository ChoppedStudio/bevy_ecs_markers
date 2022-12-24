use bevy_ecs::entity::Entity;

use crate::entity_marker::EntityMarker;

pub trait SingleMarkerData<M: EntityMarker> {
    fn get(&self) -> &Entity;
    fn get_mut(&mut self) -> &mut Entity;
}

pub trait ValueMarkerData<M: EntityMarker> {
    fn value(&self, key: M) -> &Entity;
    fn value_mut(&mut self, key: M) -> &mut Entity;

    fn unit_index(key: M) -> usize
    where
        Self: Sized;
}

pub trait DynMarkerData<M: EntityMarker> {
    fn add(&mut self, entity: Entity);
}
