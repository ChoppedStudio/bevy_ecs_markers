use bevy_ecs::world::World;

use crate::EntityMarker;

pub trait InitMarkerData {
    fn init_markerdata<M: EntityMarker>(&mut self) -> &mut Self;
}

impl InitMarkerData for World {
    fn init_markerdata<M: EntityMarker>(&mut self) -> &mut Self {
        self.init_resource::<M::MarkerData>();
        self
    }
}

#[cfg(feature = "full_bevy")]
impl InitMarkerData for bevy::prelude::App {
    fn init_markerdata<M: EntityMarker>(&mut self) -> &mut Self {
        self.init_resource::<M::MarkerData>();
        self
    }
}
