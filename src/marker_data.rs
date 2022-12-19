use bevy_ecs::{entity::Entity, system::Resource};

use crate::entity_marker::EntityMarker;

const PLACEHOLDER: Entity = Entity::from_raw(u32::MAX); // TODO: use Entity::PLACEHOLDER when released

#[derive(Resource)]
pub struct MarkerData<M: EntityMarker>
where
    [(); M::LENGTH]:,
{
    data: [Entity; M::LENGTH],
}

impl<M: EntityMarker> Default for MarkerData<M>
where
    [(); M::LENGTH]:,
{
    #[inline]
    fn default() -> Self {
        M::new_data()
    }
}

impl<M: EntityMarker> MarkerData<M>
where
    [(); M::LENGTH]:,
{
    #[inline]
    pub const fn new() -> Self {
        Self {
            data: [PLACEHOLDER; M::LENGTH],
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
