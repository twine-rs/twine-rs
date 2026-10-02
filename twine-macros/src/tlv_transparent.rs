// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

/// Generate transparent TLV value codec impls for `#[derive(TlvTransparent)]`.
///
/// Implements `TryEncodeTlvValue` and `DecodeTlvValueUnchecked` for a
/// single-field tuple struct by delegating to the inner field, mirroring
/// `#[repr(transparent)]`: the type's TLV value encoding is its field's
/// encoding.
pub(crate) fn expand(input: &DeriveInput) -> Result<TokenStream> {
    let inner_ty = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed.first().unwrap().ty
            }
            _ => {
                return Err(syn::Error::new(
                    input.ident.span(),
                    "`TlvTransparent` requires a single-field tuple struct",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "`TlvTransparent` can only be derived for structs",
            ));
        }
    };

    let ident = &input.ident;
    let (ig, tg, _) = input.generics.split_for_impl();

    // Each impl delegates to the inner type, so merge a bound on it into the
    // type's own where clause (a second `where` would not parse).
    let preds: Vec<&syn::WherePredicate> = input
        .generics
        .where_clause
        .iter()
        .flat_map(|w| w.predicates.iter())
        .collect();
    let encode_wc = quote! { where #(#preds,)* #inner_ty: ::twine_tlv::TryEncodeTlvValue };
    let decode_wc = quote! { where #(#preds,)* #inner_ty: ::twine_tlv::DecodeTlvValueUnchecked };

    Ok(quote! {
        impl #ig ::twine_tlv::TryEncodeTlvValue for #ident #tg #encode_wc {
            fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, ::twine_tlv::TwineTlvError> {
                ::twine_tlv::TryEncodeTlvValue::try_encode_tlv_value(&self.0, buffer)
            }
        }

        impl #ig ::twine_tlv::DecodeTlvValueUnchecked for #ident #tg #decode_wc {
            fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
                let inner: #inner_ty = ::twine_tlv::DecodeTlvValueUnchecked::decode_tlv_value_unchecked(buffer);
                #ident(inner)
            }
        }
    })
}
