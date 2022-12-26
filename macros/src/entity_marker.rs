use bevy_macro_utils::{get_lit_str, Symbol};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens, __private::Span};
use syn::{self, spanned::Spanned, DataEnum, DeriveInput, Error, Result};

use crate::paths;

const ENTITY_MARKER: Symbol = Symbol("entity_marker");
const DATA_NAME: Symbol = Symbol("data_name");

pub fn parse_entity_marker_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let span = input.span();

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Attribute parsing
    let attrs = match parse_marker_attr(
        &input,
        Attrs {
            data_name: format!("{}MarkerData", name.to_string()),
        },
    ) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error().into(),
    };

    // Marker Data Name Ident
    let marker_name = syn::Ident::new(&attrs.data_name, name.span());
    let marker_data_link = quote! {
        type MarkerData = #marker_name;
    };

    // Marker Data Implementation
    let marker_data = match input.data {
        syn::Data::Struct(_) => default_single_marker_impl(name, marker_name),

        syn::Data::Enum(d) => default_value_marker_impl(span, d, name, marker_name),

        syn::Data::Union(_) => quote_spanned! {
            span => compile_error!("Unions cannot be used as Markers.");
        },
    };

    quote! {
        #marker_data

        impl #impl_generics bevy_ecs_markers::EntityMarker for #name #ty_generics #where_clause {
            #marker_data_link
        }
    }
    .into()
}

struct Attrs {
    data_name: String,
}

fn parse_marker_attr(ast: &DeriveInput, mut attrs: Attrs) -> Result<Attrs> {
    let meta_items = bevy_macro_utils::parse_attrs(ast, ENTITY_MARKER)?;

    for meta in meta_items {
        use syn::{
            Meta::NameValue,
            NestedMeta::{Lit, Meta},
        };
        match meta {
            Meta(NameValue(m)) if m.path == DATA_NAME => {
                attrs.data_name = get_lit_str(DATA_NAME, &m.lit)?.value();
            }
            Meta(meta_item) => {
                return Err(Error::new_spanned(
                    meta_item.path(),
                    format!(
                        "unknown marker attribute `{}`",
                        meta_item.path().into_token_stream()
                    ),
                ));
            }
            Lit(lit) => {
                return Err(Error::new_spanned(
                    lit,
                    "unexpected literal in marker attribute",
                ))
            }
        }
    }

    Ok(attrs)
}

fn default_single_marker_impl(
    name: impl ToTokens,
    marker_name: impl ToTokens,
) -> quote::__private::TokenStream {
    let entity_path = paths::entity_path();
    let resource_path = paths::resource_path();
    let placeholder = quote! { #entity_path::from_raw(u32::MAX) }; // TODO: use Entity::PLACEHOLDER when released

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

        impl std::ops::Deref for #marker_name {
            type Target = #entity_path;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                use bevy_ecs_markers::SingleMarkerData;
                self.get()
            }
        }

        impl std::ops::DerefMut for #marker_name {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                use bevy_ecs_markers::SingleMarkerData;
                self.get_mut()
            }
        }

        impl Default for #marker_name {
            #[inline(always)]
            fn default() -> Self {
                Self(#placeholder)
            }
        }
    }
    .into()
}

fn default_value_marker_impl(
    span: Span,
    d: DataEnum,
    name: impl ToTokens,
    marker_name: impl ToTokens,
) -> quote::__private::TokenStream {
    let entity_path = paths::entity_path();
    let resource_path = paths::resource_path();
    let placeholder = quote! { #entity_path::from_raw(u32::MAX) }; // TODO: use Entity::PLACEHOLDER when released

    let capacity = d.variants.len();

    // unit_index impl
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

        impl std::ops::Index<#name> for #marker_name {
            type Output = #entity_path;

            #[inline(always)]
            fn index(&self, index: #name) -> &Self::Output {
                use bevy_ecs_markers::ValueMarkerData;
                self.value(index)
            }
        }

        impl std::ops::IndexMut<#name> for #marker_name {
            #[inline(always)]
            fn index_mut(&mut self, index: #name) -> &mut Self::Output {
                use bevy_ecs_markers::ValueMarkerData;
                self.value_mut(index)
            }
        }

        impl Default for #marker_name {
            #[inline(always)]
            fn default() -> Self {
                Self([#placeholder; #capacity])
            }
        }
    }
    .into()
}
