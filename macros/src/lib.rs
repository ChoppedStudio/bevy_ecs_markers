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

    let length = match &input.data {
        syn::Data::Struct(_) => {
            quote! {
                const LENGTH: usize = 1;
            }
        }

        syn::Data::Enum(d) => {
            let capacity = d.variants.len();
            quote! {
                const LENGTH: usize = #capacity;
            }
        }

        syn::Data::Union(_) => {
            quote_spanned! {
                span => compile_error!("Unions cannot be used as Markers.");
            }
        }
    };

    let new_data = quote! {
        fn new_data() -> bevy_ecs_markers::MarkerData<Self>
            where
                [(); Self::LENGTH]:,
                Self: Sized
        {
            bevy_ecs_markers::MarkerData::new()
        }
    };

    let unit_index = match input.data {
        syn::Data::Struct(_) => {
            quote! {
                #[inline(always)]
                fn unit_index(&self) -> usize {
                    0
                }
            }
        }

        syn::Data::Enum(d) => {
            let mut arms = quote!();
            let mut index: usize = 0;
            for variant in d.variants {
                match variant.fields {
                    syn::Fields::Unit => {
                        let ident = variant.ident;
                        arms = quote! { #arms Self::#ident => #index, };
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
                #[inline(always)]
                fn unit_index(&self) -> usize {
                    match self {
                        #arms
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
        impl #impl_generics bevy_ecs_markers::EntityMarker for #name #ty_generics #where_clause {
            #length

            #new_data

            #unit_index
        }
    }
    .into()
}
