use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use bevy_ecs::system::{ResMut, SystemParam};

use crate::entity_marker::EntityMarker;

/// A System Param that can read and modify the data from a given [`EntityMarker`]
#[derive(SystemParam)]
pub struct MarkerMut<'s, 'w, M: EntityMarker + 'static> {
    marker_data: ResMut<'w, M::MarkerData>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for MarkerMut<'s, 'w, M> {
    type Target = M::MarkerData;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &*self.marker_data
    }
}

impl<'s, 'w, M: EntityMarker + 'static> DerefMut for MarkerMut<'s, 'w, M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.marker_data
    }
}
