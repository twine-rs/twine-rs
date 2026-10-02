// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, ExprLit, Generics, Ident, Lit, Meta, Result};

/// A variant definition with its own name and TLV type byte.
struct VariantDef {
    name: String,
    tlv_type: Expr,
}

/// Parsed representation of a single `#[tlv(...)]` attribute.
///
/// Both `tlv_type` and `tlv_length` accept any expression that is valid in a
/// const context, so a named constant works as well as a literal.
///
/// Examples:
///   `#[tlv(tlv_type = 0x04, tlv_length = 4)]`
///   `#[tlv(tlv_type = 0x04, tlv_length = PSKC_MAX_SIZE)]`
///   `#[tlv(tlv_type = 0x04)]`
///   `#[tlv(variants = [("Active", tlv_type = 0x0e), ("Pending", tlv_type = 0x33)], tlv_length = 8)]`
struct TlvAttr {
    /// The TLV type byte for the base type, as a const expression.
    ///
    /// Taken from an explicit `tlv_type` when present, otherwise from the
    /// first variant's `tlv_type`.
    base_tlv_type: Expr,

    /// The constant TLV value length, as a const expression.
    ///
    /// When present, `TlvLength` and `TlvConstantMetadata` are generated automatically.
    /// When absent the type has variable length and the caller must implement `TlvLength` manually.
    tlv_length: Option<Expr>,

    /// Optional list of variant definitions.
    ///
    /// Each variant becomes a newtype wrapper around the base struct with its own TLV type byte.
    /// The first variant's `TLV_TYPE` is also used as the base type's `TLV_TYPE`.
    variants: Vec<VariantDef>,
}

/// Generate all trait impls for `#[derive(Tlv)]`.
pub(crate) fn expand(input: &DeriveInput) -> Result<TokenStream> {
    let attr = parse_tlv_attr(input)?;
    let ctx = DeriveCtx::new(input, &attr);

    let mut tokens = TokenStream::new();

    // Base type impls, always generated.
    tokens.extend(ctx.impl_tlv_type(ctx.ident, &attr.base_tlv_type));
    tokens.extend(ctx.impl_tlv_metadata(ctx.ident));
    tokens.extend(ctx.impl_constant_length(ctx.ident));
    tokens.extend(ctx.impl_decode_tlv_unchecked(ctx.ident));
    tokens.extend(ctx.impl_try_encode_tlv(ctx.ident));
    tokens.extend(ctx.impl_ref_impls(ctx.ident));

    // Variant wrapper types.
    for variant in &attr.variants {
        let variant_ident = Ident::new(&format!("{}{}", variant.name, ctx.ident), ctx.ident.span());

        tokens.extend(ctx.impl_variant_struct(&variant_ident));
        tokens.extend(ctx.impl_tlv_type(&variant_ident, &variant.tlv_type));
        tokens.extend(ctx.impl_tlv_metadata(&variant_ident));
        tokens.extend(ctx.impl_constant_length(&variant_ident));
        tokens.extend(ctx.impl_decode_tlv_unchecked(&variant_ident));
        tokens.extend(ctx.impl_try_encode_tlv(&variant_ident));
        tokens.extend(ctx.impl_ref_impls(&variant_ident));
        tokens.extend(ctx.impl_variant_encode_decode(&variant_ident));
    }

    Ok(tokens)
}

/// Parse the `#[tlv(...)]` attribute
fn parse_tlv_attr(input: &DeriveInput) -> Result<TlvAttr> {
    let mut tlv_attrs = input.attrs.iter().filter(|a| a.path().is_ident("tlv"));

    let attr = tlv_attrs
        .next()
        .ok_or_else(|| syn::Error::new(input.ident.span(), "expected a #[tlv(...)] attribute"))?;

    if let Some(extra) = tlv_attrs.next() {
        return Err(syn::Error::new_spanned(
            extra,
            "expected exactly one #[tlv(...)] attribute",
        ));
    }

    let nested = attr
        .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)?;

    let mut tlv_type: Option<Expr> = None;
    let mut tlv_length: Option<Expr> = None;
    let mut variants: Vec<VariantDef> = Vec::new();

    for meta in nested {
        match &meta {
            // `key = value` pairs
            Meta::NameValue(nv) => {
                if nv.path.is_ident("tlv_type") {
                    tlv_type = Some(nv.value.clone());
                } else if nv.path.is_ident("tlv_length") {
                    tlv_length = Some(nv.value.clone());
                } else if nv.path.is_ident("variants") {
                    variants = parse_variant_array(&nv.value)?;
                } else {
                    return Err(syn::Error::new_spanned(
                        &nv.path,
                        "unknown #[tlv(...)] key, expected `tlv_type`, `tlv_length`, or `variants`",
                    ));
                }
            }
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "expected a `key = value` pair in #[tlv(...)]",
                ));
            }
        }
    }

    let base_tlv_type = tlv_type
        .or_else(|| variants.first().map(|v| v.tlv_type.clone()))
        .ok_or_else(|| {
            syn::Error::new_spanned(
                attr,
                "#[tlv(...)] requires `tlv_type` or at least one variant with a `tlv_type`",
            )
        })?;

    Ok(TlvAttr {
        base_tlv_type,
        tlv_length,
        variants,
    })
}

/// Parse `variants = [("Name", tlv_type = 0xNN), ...]`.
///
/// Each element is a tuple expression whose first element is a string literal
/// (the variant name) and whose remaining elements are `key = value` pairs.
fn parse_variant_array(expr: &Expr) -> Result<Vec<VariantDef>> {
    let Expr::Array(arr) = expr else {
        return Err(syn::Error::new_spanned(
            expr,
            "variants must be an array of tuples, \
             e.g. [(\"Active\", tlv_type = 0x0e), (\"Pending\", tlv_type = 0x33)]",
        ));
    };

    arr.elems
        .iter()
        .map(|e| {
            let Expr::Tuple(tuple) = e else {
                return Err(syn::Error::new_spanned(
                    e,
                    "each variant must be a tuple, e.g. (\"Name\", tlv_type = 0xNN)",
                ));
            };

            let mut iter = tuple.elems.iter();

            // First element: string literal name
            let name = match iter.next() {
                Some(Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                })) => s.value(),
                Some(other) => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "first element of a variant tuple must be a string literal",
                    ));
                }
                None => {
                    return Err(syn::Error::new_spanned(
                        tuple,
                        "variant tuple must start with a string literal name",
                    ));
                }
            };

            // Remaining elements: `key = value` assignments
            let mut variant_tlv_type: Option<Expr> = None;
            for elem in iter {
                let Expr::Assign(assign) = elem else {
                    return Err(syn::Error::new_spanned(
                        elem,
                        "variant tuple elements after the name must be \
                         `key = value` pairs, e.g. tlv_type = 0xNN",
                    ));
                };

                let Expr::Path(path) = &*assign.left else {
                    return Err(syn::Error::new_spanned(
                        &assign.left,
                        "variant key must be an identifier",
                    ));
                };

                if path.path.is_ident("tlv_type") {
                    variant_tlv_type = Some((*assign.right).clone());
                } else {
                    return Err(syn::Error::new_spanned(
                        path,
                        "unknown variant key, expected `tlv_type`",
                    ));
                }
            }

            let tlv_type = variant_tlv_type.ok_or_else(|| {
                syn::Error::new_spanned(tuple, "each variant requires `tlv_type`")
            })?;

            Ok(VariantDef { name, tlv_type })
        })
        .collect()
}

/// Context helper that carries common data through code generation.
struct DeriveCtx<'a> {
    ident: &'a Ident,
    vis: &'a syn::Visibility,
    generics: &'a Generics,
    tlv_length: Option<&'a Expr>,
}

impl<'a> DeriveCtx<'a> {
    fn new(input: &'a DeriveInput, attr: &'a TlvAttr) -> Self {
        Self {
            ident: &input.ident,
            vis: &input.vis,
            generics: &input.generics,
            tlv_length: attr.tlv_length.as_ref(),
        }
    }

    /// Split generics into the three parts needed for impls: `impl<...>`, `Type<...>`, and `where ...`.
    fn split_generics(&self) -> (TokenStream, TokenStream, TokenStream) {
        let (ig, tg, wc) = self.generics.split_for_impl();
        (quote!(#ig), quote!(#tg), quote!(#wc))
    }

    /// Build a `where` clause holding the type's own predicates plus an extra bound.
    fn where_with_bound(&self, bound: TokenStream) -> TokenStream {
        let preds = self
            .generics
            .where_clause
            .iter()
            .flat_map(|w| w.predicates.iter());
        quote! { where #(#preds,)* #bound }
    }

    /// Implement `TlvType` for the given target type with the specified TLV type byte.
    fn impl_tlv_type(&self, target: &Ident, tlv_type: &Expr) -> TokenStream {
        let (ig, tg, wc) = self.split_generics();
        quote! {
            impl #ig ::twine_tlv::TlvType for #target #tg #wc {
                const TLV_TYPE: u8 = #tlv_type;
            }
        }
    }

    /// Implement `TlvMetadata` for the given target type
    fn impl_tlv_metadata(&self, target: &Ident) -> TokenStream {
        let (ig, tg, wc) = self.split_generics();
        quote! {
            impl #ig ::twine_tlv::TlvMetadata for #target #tg #wc {}
        }
    }

    /// Implement `TlvConstantMetadata` and `TlvLength` for the given target type when a constant length is specified.
    fn impl_constant_length(&self, target: &Ident) -> TokenStream {
        let (ig, tg, wc) = self.split_generics();
        match self.tlv_length {
            Some(len) => quote! {
                impl #ig ::twine_tlv::TlvConstantMetadata for #target #tg #wc {
                    const TLV_LEN: usize = #len;
                }

                impl #ig ::twine_tlv::TlvLength for #target #tg #wc {
                    fn tlv_len(&self) -> usize {
                        <Self as ::twine_tlv::TlvConstantMetadata>::TLV_LEN
                    }

                    fn tlv_len_is_constant() -> bool {
                        true
                    }
                }
            },
            None => TokenStream::new(),
        }
    }

    /// Implement `DecodeTlvUnchecked` for the given target type.
    ///
    /// Delegates to `DecodeTlvValueUnchecked` for TLV value parsing.
    fn impl_decode_tlv_unchecked(&self, target: &Ident) -> TokenStream {
        let (ig, tg, wc) = self.split_generics();
        quote! {
            impl #ig ::twine_tlv::DecodeTlvUnchecked for #target #tg #wc {
                fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
                    use ::twine_tlv::GetTlvLength as _;
                    use ::twine_tlv::__private::bytes::Buf as _;
                    let mut buffer = buffer.as_ref();
                    let _type_byte = buffer.get_u8();
                    let _len_byte = buffer.get_tlv_length();
                    ::twine_tlv::DecodeTlvValueUnchecked::decode_tlv_value_unchecked(buffer)
                }
            }
        }
    }

    /// Implement `TryEncodeTlv` for the given target type, delegating to `write_tlv`.
    fn impl_try_encode_tlv(&self, target: &Ident) -> TokenStream {
        let (ig, tg, wc) = self.split_generics();
        quote! {
            impl #ig ::twine_tlv::TryEncodeTlv for #target #tg #wc {
                fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, ::twine_tlv::TwineTlvError> {
                    ::twine_tlv::write_tlv(buffer, <Self as ::twine_tlv::TlvType>::TLV_TYPE, self)
                }
            }
        }
    }

    /// Implement the reference impls for `&T` by delegating to `T`.
    ///
    /// Covers `TlvType`, `TlvMetadata`, `TlvLength`, and optionally
    /// `TlvConstantMetadata`.
    fn impl_ref_impls(&self, target: &Ident) -> TokenStream {
        let (ig, tg, wc) = self.split_generics();

        let tlv_len_ref = match self.tlv_length {
            Some(_) => quote! {
                impl #ig ::twine_tlv::TlvLength for &#target #tg #wc {
                    fn tlv_len(&self) -> usize {
                        ::twine_tlv::TlvLength::tlv_len(*self)
                    }

                    fn tlv_len_is_constant() -> bool {
                        <#target #tg as ::twine_tlv::TlvLength>::tlv_len_is_constant()
                    }
                }
            },
            None => {
                let wc = self.where_with_bound(quote!(#target #tg: ::twine_tlv::TlvLength));
                quote! {
                    impl #ig ::twine_tlv::TlvLength for &#target #tg #wc {
                        fn tlv_len(&self) -> usize {
                            ::twine_tlv::TlvLength::tlv_len(*self)
                        }

                        fn tlv_len_is_constant() -> bool {
                            <#target #tg as ::twine_tlv::TlvLength>::tlv_len_is_constant()
                        }
                    }
                }
            }
        };

        let const_meta_ref = if self.tlv_length.is_some() {
            let wc = self.where_with_bound(quote!(#target #tg: ::twine_tlv::TlvConstantMetadata));
            quote! {
                #[allow(unused)]
                impl #ig ::twine_tlv::TlvConstantMetadata for &#target #tg #wc {
                    const TLV_LEN: usize = <#target #tg as ::twine_tlv::TlvConstantMetadata>::TLV_LEN;
                }
            }
        } else {
            TokenStream::new()
        };

        quote! {
            #tlv_len_ref

            impl #ig ::twine_tlv::TlvType for &#target #tg #wc {
                const TLV_TYPE: u8 = <#target #tg as ::twine_tlv::TlvType>::TLV_TYPE;
            }

            impl #ig ::twine_tlv::TlvMetadata for &#target #tg #wc {}

            #const_meta_ref
        }
    }

    /// Implement the variant wrapper struct and `From` conversions to/from the base type.
    fn impl_variant_struct(&self, variant_ident: &Ident) -> TokenStream {
        let base = self.ident;
        let vis = self.vis;
        let generics = self.generics;
        let (ig, tg, wc) = self.split_generics();

        quote! {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            #vis struct #variant_ident #generics (#base #tg) #wc;

            impl #ig From<#variant_ident #tg> for #base #tg #wc {
                fn from(value: #variant_ident #tg) -> Self {
                    value.0
                }
            }

            impl #ig From<#base #tg> for #variant_ident #tg #wc {
                fn from(value: #base #tg) -> Self {
                    #variant_ident(value)
                }
            }

            impl #ig ::core::ops::Deref for #variant_ident #tg #wc {
                type Target = #base #tg;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }

    /// Implement the value encode and decode traits for a variant wrapper.
    ///
    /// Implements `TryEncodeTlvValue` and `DecodeTlvValueUnchecked` by
    /// delegating to the inner base type.
    fn impl_variant_encode_decode(&self, variant_ident: &Ident) -> TokenStream {
        let base = self.ident;
        let (ig, tg, wc) = self.split_generics();

        quote! {
            impl #ig ::twine_tlv::TryEncodeTlvValue for #variant_ident #tg #wc {
                fn try_encode_tlv_value(&self, buffer: &mut [u8]) -> Result<usize, ::twine_tlv::TwineTlvError> {
                    self.0.try_encode_tlv_value(buffer)
                }
            }

            impl #ig ::twine_tlv::DecodeTlvValueUnchecked for #variant_ident #tg #wc {
                fn decode_tlv_value_unchecked(buffer: impl AsRef<[u8]>) -> Self {
                    #variant_ident(<#base #tg>::decode_tlv_value_unchecked(buffer))
                }
            }
        }
    }
}
