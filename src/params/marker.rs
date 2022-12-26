use std::marker::PhantomData;
use std::ops::Deref;

use bevy_ecs::system::{Res, SystemParam};

use crate::entity_marker::EntityMarker;

/// A System Param that can read the data from a given [`EntityMarker`]
#[derive(SystemParam)]
pub struct Marker<'s, 'w, M: EntityMarker + 'static> {
    marker_data: Res<'w, <M as EntityMarker>::MarkerData>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for Marker<'s, 'w, M> {
    type Target = M::MarkerData;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &*self.marker_data
    }
}
