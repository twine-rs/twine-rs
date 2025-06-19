// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Generics, Ident, Meta};

#[derive(Debug, FromMeta)]
struct TlvEntry {
    variant: Option<Ident>,
    tlv_type: u8,
    tlv_length: Option<usize>,
}

#[proc_macro_derive(Tlv, attributes(tlv))]
pub fn derive_tlv(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut entries = Vec::new();

    for attr in &input.attrs {
        if attr.path().is_ident("tlv") {
            let meta = attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )
                .expect("Invalid #[tlv(...)] attribute");
            let nested: Vec<NestedMeta> = meta.into_iter().map(NestedMeta::Meta).collect();
            let entry = TlvEntry::from_list(&nested).expect("Failed to parse tlv attribute");
            entries.push(entry);
        }
    }

    let original_ident = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut tokens = TokenStream2::new();

    let has_variants = entries.iter().any(|e| e.variant.is_some());

    if let Some(first) = entries.first() {
        tokens.extend(impl_all(
            original_ident,
            generics,
            original_ident,
            first.tlv_type,
            first.tlv_length,
            !has_variants,
        ));
        tokens.extend(impl_refs(original_ident, generics, first.tlv_length));
    }

    if has_variants {
        for entry in entries.iter() {
            let variant = entry
                .variant
                .clone()
                .expect("Each #[tlv(...)] must include `variant` if there are multiple");
            let wrapper_ident =
                Ident::new(&format!("{}{}", variant, original_ident), variant.span());

            tokens.extend(quote! {
                #[derive(Copy, Clone, Debug, Eq, PartialEq)]
                #vis struct #wrapper_ident #generics (#original_ident #generics);

                impl #generics From<#wrapper_ident #generics> for #original_ident #generics {
                    fn from(value: #wrapper_ident #generics) -> Self {
                        value.0
                    }
                }

                impl #generics From<#original_ident #generics> for #wrapper_ident #generics {
                    fn from(value: #original_ident #generics) -> Self {
                        #wrapper_ident(value)
                    }
                }

                impl #generics std::ops::Deref for #wrapper_ident #generics {
                    type Target = #original_ident #generics;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                impl #impl_generics TryEncodeTlvValue for #wrapper_ident #ty_generics #where_clause {
                    fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
                        self.0.try_encode_tlv_value(buffer)
                    }
                }

                impl #impl_generics DecodeTlvValueUnchecked for #wrapper_ident #ty_generics #where_clause {
                    fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
                        #wrapper_ident(<#original_ident #ty_generics>::decode_tlv_value_unchecked(buffer))
                    }
                }
            });

            tokens.extend(impl_all(
                &wrapper_ident,
                generics,
                original_ident,
                entry.tlv_type,
                entry.tlv_length,
                true,
            ));
            tokens.extend(impl_refs(&wrapper_ident, generics, entry.tlv_length));
        }
    }

    tokens.into()
}

fn impl_all(
    target_ident: &Ident,
    generics: &Generics,
    _inner_ident: &Ident,
    tlv_type: u8,
    tlv_length: Option<usize>,
    impl_encode_decode: bool,
) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let const_meta = if let Some(len) = tlv_length {
        quote! {
            impl #impl_generics TlvConstantMetadata for #target_ident #ty_generics #where_clause {
                const TLV_LEN: usize = #len;
            }
        }
    } else {
        quote! {}
    };

    let base = quote! {
        impl #impl_generics TlvType for #target_ident #ty_generics #where_clause {
            const TLV_TYPE: u8 = #tlv_type;
        }

        impl #impl_generics TlvMetadata for #target_ident #ty_generics #where_clause {}

        #const_meta
    };

    let extra = if impl_encode_decode {
        quote! {
            impl #impl_generics DecodeTlvUnchecked for #target_ident #ty_generics #where_clause {
                fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
                    let mut buffer = buffer.as_ref();
                    let _type_byte = buffer.get_u8();
                    let _len_byte = buffer.get_tlv_length();
                    Self::decode_tlv_value_unchecked(buffer)
                }
            }

            impl #impl_generics TryEncodeTlv for #target_ident #ty_generics #where_clause {
                fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
                    write_tlv(buffer, Self::TLV_TYPE, &*self)
                }
            }
        }
    } else {
        TokenStream2::new()
    };

    let tlv_len_impl = match tlv_length {
        Some(_) => quote! {
            impl #impl_generics TlvLength for #target_ident #ty_generics #where_clause {
                fn tlv_len(&self) -> usize {
                    Self::TLV_LEN
                }
                fn tlv_len_is_constant() -> bool {
                    true
                }
            }
        },
        None => quote! {},
    };

    quote! {
        #base
        #extra
        #tlv_len_impl
    }
}

fn impl_refs(target_ident: &Ident, generics: &Generics, tlv_length: Option<usize>) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let const_meta = if tlv_length.is_some() {
        quote! {
            #[allow(unused)]
            impl #impl_generics TlvConstantMetadata for &#target_ident #ty_generics #where_clause
            where
                #target_ident #ty_generics: TlvConstantMetadata
            {
                const TLV_LEN: usize = <#target_ident #ty_generics>::TLV_LEN;
            }
        }
    } else {
        quote! {}
    };

    let tlv_len_impl = match tlv_length {
        Some(_) => quote! {
            impl #impl_generics TlvLength for &#target_ident #ty_generics #where_clause {
                fn tlv_len(&self) -> usize {
                    (**self).tlv_len()
                }
                fn tlv_len_is_constant() -> bool {
                    <#target_ident #ty_generics>::tlv_len_is_constant()
                }
            }
        },
        None => quote! {
            impl #impl_generics TlvLength for &#target_ident #ty_generics #where_clause
            where
                #target_ident #ty_generics: TlvLength
            {
                fn tlv_len(&self) -> usize {
                    (**self).tlv_len()
                }
                fn tlv_len_is_constant() -> bool {
                    <#target_ident #ty_generics>::tlv_len_is_constant()
                }
            }
        },
    };

    quote! {
        #tlv_len_impl

        impl #impl_generics TlvType for &#target_ident #ty_generics #where_clause {
            const TLV_TYPE: u8 = <#target_ident #ty_generics>::TLV_TYPE;
        }

        impl #impl_generics TlvMetadata for &#target_ident #ty_generics #where_clause {}

        #const_meta
    }
}
