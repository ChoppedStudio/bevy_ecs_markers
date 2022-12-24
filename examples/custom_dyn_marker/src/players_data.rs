use std::ops::Deref;

use bevy::prelude::{Entity, Resource};
use bevy_ecs_markers::SingleMarkerData;

use crate::Players;

#[derive(Resource, Default)]
pub struct PlayersMarkerData(Vec<Entity>, usize);

impl PlayersMarkerData {
    pub fn add(&mut self, entity: Entity) {
        self.0.push(entity)
    }
}

impl SingleMarkerData<Players> for PlayersMarkerData {
    fn get(&self) -> &Entity {
        &self.0[self.1]
    }

    fn get_mut(&mut self) -> &mut Entity {
        &mut self.0[self.1]
    }
}

impl Deref for PlayersMarkerData {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
