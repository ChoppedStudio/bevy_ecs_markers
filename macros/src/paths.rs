use bevy_macro_utils::BevyManifest;
use quote::format_ident;

#[inline]
pub(crate) fn bevy_ecs_path() -> syn::Path {
    BevyManifest::default().get_path("bevy_ecs")
}

#[inline]
pub(crate) fn entity_path() -> syn::Path {
    let mut entity_path = bevy_ecs_path();
    entity_path.segments.push(format_ident!("entity").into());
    entity_path.segments.push(format_ident!("Entity").into());
    entity_path
}

#[inline]
pub(crate) fn resource_path() -> syn::Path {
    let mut resource_path = bevy_ecs_path();
    resource_path.segments.push(format_ident!("system").into());
    resource_path
        .segments
        .push(format_ident!("Resource").into());
    resource_path
}
