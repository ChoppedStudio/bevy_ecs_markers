use std::ops::Deref;
use std::{marker::PhantomData, ops::Index};

use bevy_ecs::prelude::Entity;
use bevy_ecs::system::{Res, SystemParam};

use crate::entity_marker::EntityMarker;
use crate::{SingleMarkerData, ValueMarkerData};

/// A System Param that can read the data from a given [`EntityMarker`]
#[derive(SystemParam)]
pub struct Marker<'s, 'w, M: EntityMarker + 'static> {
    marker_data: Res<'w, M::MarkerData>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + 'static> Index<M> for Marker<'s, 'w, M>
where
    M::MarkerData: ValueMarkerData<M>,
{
    type Output = Entity;

    #[inline(always)]
    fn index(&self, index: M) -> &Self::Output {
        self.marker_data.value(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for Marker<'s, 'w, M>
where
    M::MarkerData: SingleMarkerData<M>,
{
    type Target = Entity;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.marker_data.get()
    }
}
