// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod tlv;

use proc_macro::TokenStream;

use syn::parse_macro_input;

/// Derive macro for implementing TLV encoding/decoding traits.
///
/// See the [`tlv`] module for the full attribute grammar.
#[proc_macro_derive(Tlv, attributes(tlv))]
pub fn derive_tlv(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    tlv::expand(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
