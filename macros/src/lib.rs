use bevy_macro_utils::BevyManifest;
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{self, spanned::Spanned};

#[proc_macro_derive(EntityMarker)]
pub fn entity_marker_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let span = input.span();

    let name = &input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let temp = format!("{}MarkerData", name.to_string());
    let marker_name = syn::Ident::new(&temp, name.span());
    let marker_data_link = quote! {
        type MarkerData = #marker_name;
    };

    // BEGIN Some useful declarations

    let mut entity_path = bevy_ecs_path();
    entity_path.segments.push(format_ident!("entity").into());
    entity_path.segments.push(format_ident!("Entity").into());

    let mut resource_path = bevy_ecs_path();
    resource_path.segments.push(format_ident!("system").into());
    resource_path
        .segments
        .push(format_ident!("Resource").into());

    let placeholder = quote! { #entity_path::from_raw(u32::MAX) }; // TODO: use Entity::PLACEHOLDER when released

    // END

    let marker_data = match input.data {
        syn::Data::Struct(_) => {
            quote! {
                #[derive(#resource_path)]
                struct #marker_name (#entity_path);

                impl bevy_ecs_markers::SingleMarkerData<#name> for #marker_name {
                    #[inline(always)]
                    fn get(&self) -> &#entity_path {
                        &self.0
                    }

                    #[inline(always)]
                    fn get_mut(&mut self) -> &mut #entity_path {
                        &mut self.0
                    }
                }

                impl Default for #marker_name {
                    #[inline(always)]
                    fn default() -> Self {
                        Self(#placeholder)
                    }
                }
            }
        }

        syn::Data::Enum(d) => {
            let capacity = d.variants.len();

            // BEGIN unit_index

            let mut arms = quote!();
            let mut index: usize = 0;

            for variant in d.variants {
                match variant.fields {
                    syn::Fields::Unit => {
                        let ident = variant.ident;
                        arms = quote! { #arms #name::#ident => #index, };
                        index += 1;
                    }
                    _ => {
                        return quote_spanned! {
                            span => compile_error!("All Fields should be Units!");
                        }
                        .into();
                    }
                };
            }

            // END

            quote! {
                #[derive(#resource_path)]
                struct #marker_name ([#entity_path; #capacity]);

                impl bevy_ecs_markers::ValueMarkerData<#name> for #marker_name {
                    #[inline(always)]
                    fn value(&self, key: #name) -> &#entity_path {
                        &self.0[#marker_name::unit_index(key)]
                    }

                    #[inline(always)]
                    fn value_mut(&mut self, key: #name) -> &mut #entity_path {
                        &mut self.0[#marker_name::unit_index(key)]
                    }

                    #[inline]
                    fn unit_index(key: #name) -> usize
                        where
                            Self: Sized
                    {
                        match key {
                            #arms
                        }
                    }
                }

                impl Default for #marker_name {
                    #[inline(always)]
                    fn default() -> Self {
                        Self([#placeholder; #capacity])
                    }
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
        #marker_data

        impl #impl_generics bevy_ecs_markers::EntityMarker for #name #ty_generics #where_clause {
            #marker_data_link
        }
    }
    .into()
}

pub(crate) fn bevy_ecs_path() -> syn::Path {
    BevyManifest::default().get_path("bevy_ecs")
}
