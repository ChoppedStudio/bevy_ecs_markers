use std::ops::{Deref, DerefMut, IndexMut};
use std::{marker::PhantomData, ops::Index};

use bevy_ecs::prelude::Entity;
use bevy_ecs::system::{ResMut, SystemParam};

use crate::{entity_marker::EntityMarker, marker_data::MarkerData};

#[derive(SystemParam)]
pub struct MarkerMut<'s, 'w, M: EntityMarker + 'static>
where
    [(); M::LENGTH]:,
{
    marker_data: ResMut<'w, MarkerData<M>>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'s, 'w, M: EntityMarker + 'static> Index<M> for MarkerMut<'s, 'w, M>
where
    [(); M::LENGTH]:,
{
    type Output = Entity;

    fn index(&self, index: M) -> &Self::Output {
        self.marker_data.value(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> IndexMut<M> for MarkerMut<'s, 'w, M>
where
    [(); M::LENGTH]:,
{
    fn index_mut(&mut self, index: M) -> &mut Self::Output {
        self.marker_data.value_mut(index)
    }
}

impl<'s, 'w, M: EntityMarker + 'static> Deref for MarkerMut<'s, 'w, M>
where
    [(); M::LENGTH]:,
{
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.marker_data.get()
    }
}

impl<'s, 'w, M: EntityMarker + 'static> DerefMut for MarkerMut<'s, 'w, M>
where
    [(); M::LENGTH]:,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.marker_data.get_mut()
    }
}
