// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod tlv;
mod twine_shell;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive macro for implementing TLV encoding/decoding traits.
///
/// See the [`tlv`] module for the full attribute grammar.
#[proc_macro_derive(Tlv, attributes(tlv))]
pub fn derive_tlv(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    tlv::expand(&input).into()
}

/// Derive macro for implementing `TwineCtl` on shell-based interfaces.
///
/// Generates an `impl TwineCtl for T` block that delegates each trait method
/// to the corresponding `shell_*` method provided by `TwineCtlShell`.
///
/// # Attributes
///
/// - `#[twine_shell(crate_path = "...")]` — Override the path used to reference the
///   `twine_ctl` crate. Defaults to `::twine_ctl`. Set to `crate` when deriving from
///   within the `twine-ctl` crate itself.
///
/// # Example
///
/// ```ignore
/// // From an external crate (default):
/// #[derive(TwineShell)]
/// pub struct MyShell { /* ... */ }
///
/// // From within twine-ctl:
/// #[derive(TwineShell)]
/// #[twine_shell(crate_path = "crate")]
/// pub struct TwineCtlSerialShell { /* ... */ }
/// ```
#[proc_macro_derive(TwineShell, attributes(twine_shell))]
pub fn derive_twine_shell(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    twine_shell::expand(&input).into()
}
