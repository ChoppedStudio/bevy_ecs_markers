use std::ops::{Deref, DerefMut, IndexMut};
use std::{marker::PhantomData, ops::Index};

use bevy_ecs::prelude::Entity;
use bevy_ecs::system::{ResMut, SystemParam};

use crate::DynamicEntityMarker;
use crate::{entity_marker::EntityMarker, marker_data::MarkerData};

/// A System Param that can read and modify the data from a given [`EntityMarker`]
#[derive(SystemParam)]
pub struct MarkerMut<'s, 'w, M: EntityMarker + 'static> {
    marker_data: ResMut<'w, MarkerData<M>>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + DynamicEntityMarker + 'static> MarkerMut<'s, 'w, M> {
    #[inline(always)]
    pub fn add(&mut self, entity: Entity) {
        self.marker_data.add(entity);
    }
}

impl<'s, 'w, M: EntityMarker + 'static> Index<M> for MarkerMut<'s, 'w, M> {
    type Output = Entity;

    #[inline(always)]
    fn index(&self, index: M) -> &Self::Output {
        self.marker_data.value(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> IndexMut<M> for MarkerMut<'s, 'w, M> {
    #[inline(always)]
    fn index_mut(&mut self, index: M) -> &mut Self::Output {
        self.marker_data.value_mut(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for MarkerMut<'s, 'w, M> {
    type Target = Entity;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.marker_data.get()
    }
}

impl<'s, 'w, M: EntityMarker + 'static> DerefMut for MarkerMut<'s, 'w, M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.marker_data.get_mut()
    }
}
