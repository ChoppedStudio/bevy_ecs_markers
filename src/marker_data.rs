use bevy_ecs::{entity::Entity, system::Resource};

use crate::entity_marker::EntityMarker;

#[derive(Resource)]
pub struct MarkerData<M: EntityMarker> {
    data: M::Storage,
}

impl<M: EntityMarker> Default for MarkerData<M> {
    #[inline]
    fn default() -> Self {
        M::new_data()
    }
}

impl<M: EntityMarker> MarkerData<M> {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: M::create_storage(),
        }
    }

    pub fn value(&self, key: M) -> &Entity {
        &self.data[key.unit_index()]
    }

    pub fn value_mut(&mut self, key: M) -> &mut Entity {
        &mut self.data[key.unit_index()]
    }

    #[inline]
    pub fn get(&self) -> &Entity {
        &self.data[0]
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut Entity {
        &mut self.data[0]
    }
}
