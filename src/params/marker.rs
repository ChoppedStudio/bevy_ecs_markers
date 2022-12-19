use std::ops::Deref;
use std::{marker::PhantomData, ops::Index};

use bevy_ecs::prelude::Entity;
use bevy_ecs::system::{Res, SystemParam};

use crate::{entity_marker::EntityMarker, marker_data::MarkerData};

#[derive(SystemParam)]
pub struct Marker<'s, 'w, M: EntityMarker + 'static>
where
    [(); M::LENGTH]:,
{
    marker_data: Res<'w, MarkerData<M>>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + 'static> Index<M> for Marker<'s, 'w, M>
where
    [(); M::LENGTH]:,
{
    type Output = Entity;

    fn index(&self, index: M) -> &Self::Output {
        self.marker_data.value(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for Marker<'s, 'w, M>
where
    [(); M::LENGTH]:,
{
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.marker_data.get()
    }
}
