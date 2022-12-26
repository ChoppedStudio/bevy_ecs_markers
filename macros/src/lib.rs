use proc_macro::TokenStream;

mod entity_marker;
mod paths;

#[proc_macro_derive(EntityMarker, attributes(entity_marker))]
pub fn entity_marker_derive(input: TokenStream) -> TokenStream {
    entity_marker::parse_entity_marker_derive(input)
}
