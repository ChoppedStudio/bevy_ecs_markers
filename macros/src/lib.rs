use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self, spanned::Spanned};

#[proc_macro_derive(EntityMarker)]
pub fn item_data_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let span = input.span();

    let name = &input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let new_data = match input.data {
        syn::Data::Struct(_) => {
            quote! {
                fn new_data() -> bevy_ecs_markers::MarkerData<Self>
                    where
                        Self: Sized
                {
                    // TODO: use Entity::PLACEHOLDER when released
                    // TODO: is there a better path to `bevy_ecs::entity::Entity`?
                    bevy_ecs_markers::MarkerData::new(bevy_ecs_markers::MarkerDataType::Single(bevy_ecs::entity::Entity::from_raw(u32::MAX)))
                }
            }
        }

        syn::Data::Enum(d) => {
            let capacity = d.variants.len();
            quote! {
                fn new_data() -> bevy_ecs_markers::MarkerData<Self>
                    where
                        Self: Sized
                {
                    bevy_ecs_markers::MarkerData::new(bevy_ecs_markers::MarkerDataType::Multiple(hashbrown::HashMap::with_capacity(#capacity)))
                }
            }
        }

        syn::Data::Union(_) => {
            quote_spanned! {
                span => compile_error!("Unions cannot be used as Markers.");
            }
        }
    };

    quote! {
        impl #impl_generics bevy_ecs_markers::EntityMarker for #name #ty_generics #where_clause {
            #new_data
        }
    }
    .into()
}
