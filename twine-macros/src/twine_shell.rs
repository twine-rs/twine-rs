// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, ExprLit, Lit, Meta, Result};

/// Generate the `impl TwineCtl for T` block for a shell-backed type.
///
/// Each trait method delegates to the corresponding `shell_*` method
/// provided by `TwineCtlShell`.
pub(crate) fn expand(input: &DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let ctl_path = parse_crate_path(input)?;

    Ok(quote! {
        #[::async_trait::async_trait]
        impl #ctl_path::TwineCtl for #ident {
            async fn new_random_network(&mut self) -> Result<(), #ctl_path::TwineCtlError> {
                self.shell_new_random_network().await
            }

            async fn active_dataset(&mut self) -> Result<::twine_codec::OperationalDataset, #ctl_path::TwineCtlError> {
                self.shell_active_dataset().await
            }

            async fn attach_with_dataset(
                &mut self,
                dataset: &::twine_codec::OperationalDataset,
            ) -> Result<(), #ctl_path::TwineCtlError> {
                self.shell_attach_with_dataset(dataset).await
            }

            async fn pending_dataset(&mut self) -> Result<::twine_codec::OperationalDataset, #ctl_path::TwineCtlError> {
                self.shell_pending_dataset().await
            }

            async fn channel(&mut self) -> Result<::twine_codec::Channel, #ctl_path::TwineCtlError> {
                self.shell_channel().await
            }

            async fn preferred_channel_mask(&mut self) -> Result<::twine_codec::ChannelMask, #ctl_path::TwineCtlError> {
                self.shell_preferred_channel_mask().await
            }

            async fn supported_channel_mask(&mut self) -> Result<::twine_codec::ChannelMask, #ctl_path::TwineCtlError> {
                self.shell_supported_channel_mask().await
            }

            async fn factory_reset(&mut self) -> Result<(), #ctl_path::TwineCtlError> {
                self.shell_factory_reset().await
            }

            async fn network_name(&mut self) -> Result<::twine_codec::NetworkName, #ctl_path::TwineCtlError> {
                self.shell_network_name().await
            }

            async fn pan_id(&mut self) -> Result<::twine_codec::PanId, #ctl_path::TwineCtlError> {
                self.shell_pan_id().await
            }

            async fn reset(&mut self) -> Result<(), #ctl_path::TwineCtlError> {
                self.shell_reset().await
            }

            async fn rloc16(&mut self) -> Result<::twine_codec::Rloc16, #ctl_path::TwineCtlError> {
                self.shell_rloc16().await
            }

            async fn role(&mut self) -> Result<::twine_codec::NetworkRole, #ctl_path::TwineCtlError> {
                self.shell_role().await
            }

            async fn version(&mut self) -> Result<String, #ctl_path::TwineCtlError> {
                self.shell_version().await
            }

            async fn uptime(&mut self) -> Result<String, #ctl_path::TwineCtlError> {
                self.shell_uptime().await
            }
        }
    })
}

/// Parse the optional `#[twine_shell(crate_path = "...")]` attribute.
///
/// Returns the token stream for the crate path, defaulting to `::twine_ctl`.
fn parse_crate_path(input: &DeriveInput) -> Result<TokenStream> {
    let mut crate_path: Option<TokenStream> = None;

    for attr in &input.attrs {
        if !attr.path().is_ident("twine_shell") {
            continue;
        }

        let nested = attr.parse_args_with(
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
        )?;

        for meta in nested {
            match &meta {
                Meta::NameValue(nv) if nv.path.is_ident("crate_path") => {
                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = &nv.value
                    else {
                        return Err(syn::Error::new_spanned(
                            &nv.value,
                            "crate_path value must be a string literal",
                        ));
                    };

                    let path: syn::Path = s.parse()?;
                    crate_path = Some(quote!(#path));
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "unknown #[twine_shell(...)] key, expected `crate_path`",
                    ));
                }
            }
        }
    }

    // Default: absolute path to the twine_ctl crate
    Ok(crate_path.unwrap_or_else(|| quote!(::twine_ctl)))
}
