use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, system::Resource};
use hashbrown::HashMap;

use crate::entity_marker::EntityMarker;

const PLACEHOLDER: Entity = Entity::from_raw(u32::MAX); // TODO: use Entity::PLACEHOLDER when released

pub enum MarkerDataType<M: EntityMarker> {
    Multiple(HashMap<M, Entity>),
    Single(Entity),
}

#[derive(Resource)]
pub struct MarkerData<M: EntityMarker> {
    data: MarkerDataType<M>,
    phantom: PhantomData<M>,
}

impl<M: EntityMarker> Default for MarkerData<M> {
    #[inline]
    fn default() -> Self {
        M::new_data()
    }
}

impl<M: EntityMarker> MarkerData<M> {
    #[inline]
    pub const fn new(data: MarkerDataType<M>) -> Self {
        Self {
            data,
            phantom: PhantomData,
        }
    }

    pub fn value(&self, key: M) -> &Entity {
        match &self.data {
            MarkerDataType::Multiple(map) => map.get(&key).unwrap_or(&PLACEHOLDER),
            MarkerDataType::Single(entity) => entity,
        }
    }

    pub fn value_mut(&mut self, key: M) -> &mut Entity {
        match &mut self.data {
            MarkerDataType::Multiple(map) => map.entry(key).or_insert(PLACEHOLDER),
            MarkerDataType::Single(entity) => entity,
        }
    }

    pub fn get(&self) -> &Entity {
        match &self.data {
            MarkerDataType::Multiple(map) => map.values().next().unwrap(), // TODO: make this a better error (get expects `MarkerDataType::Single`)
            MarkerDataType::Single(entity) => entity,
        }
    }

    pub fn get_mut(&mut self) -> &mut Entity {
        match &mut self.data {
            MarkerDataType::Multiple(map) => map.values_mut().next().unwrap(), // TODO: make this a better error (get expects `MarkerDataType::Single`)
            MarkerDataType::Single(entity) => entity,
        }
    }
}
