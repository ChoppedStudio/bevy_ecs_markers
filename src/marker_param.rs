use std::ops::{Deref, DerefMut, IndexMut};
use std::{marker::PhantomData, ops::Index};

use bevy_ecs::prelude::Entity;
use bevy_ecs::system::{Res, ResMut, SystemParam};

use crate::{entity_marker::EntityMarker, marker_data::MarkerData};

#[derive(SystemParam)]
pub struct Marker<'s, 'w, M: EntityMarker + 'static> {
    marker_data: Res<'w, MarkerData<M>>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + 'static> Index<M> for Marker<'s, 'w, M> {
    type Output = Entity;

    fn index(&self, index: M) -> &Self::Output {
        self.marker_data.value(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for Marker<'s, 'w, M> {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.marker_data.get()
    }
}

#[derive(SystemParam)]
pub struct MarkerMut<'s, 'w, M: EntityMarker + 'static> {
    marker_data: ResMut<'w, MarkerData<M>>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + 'static> Index<M> for MarkerMut<'s, 'w, M> {
    type Output = Entity;

    fn index(&self, index: M) -> &Self::Output {
        self.marker_data.value(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> IndexMut<M> for MarkerMut<'s, 'w, M> {
    fn index_mut(&mut self, index: M) -> &mut Self::Output {
        self.marker_data.value_mut(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for MarkerMut<'s, 'w, M> {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.marker_data.get()
    }
}

impl<'s, 'w, M: EntityMarker + 'static> DerefMut for MarkerMut<'s, 'w, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.marker_data.get_mut()
    }
}
