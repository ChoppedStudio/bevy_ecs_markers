use bevy_ecs::{entity::Entity, system::Resource};

use crate::entity_marker::EntityMarker;

/// This is the container which stores all IDs from [`Entity`] in it
#[derive(Resource)]
pub struct MarkerData<M: EntityMarker> {
    data: M::Storage,
}

impl<M: EntityMarker> Default for MarkerData<M> {
    #[inline(always)]
    fn default() -> Self {
        M::new_data()
    }
}

impl<M: EntityMarker> MarkerData<M> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            data: M::create_storage(),
        }
    }

    #[inline(always)]
    pub fn value(&self, key: M) -> &Entity {
        &self.data[key.unit_index()]
    }

    #[inline(always)]
    pub fn value_mut(&mut self, key: M) -> &mut Entity {
        &mut self.data[key.unit_index()]
    }

    #[inline(always)]
    pub fn get(&self) -> &Entity {
        &self.data[0]
    }

    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut Entity {
        &mut self.data[0]
    }
}
