// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, Default, FromDeriveInput)]
#[darling(attributes(tlv), forward_attrs(allow, doc, cfg))]
struct ConstantTlvMacroArgs {
    tlv_type: u8,
    tlv_length: usize,
}

#[proc_macro_derive(ConstantTlv, attributes(tlv))]
pub fn derive_constant_tlv(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let args = ConstantTlvMacroArgs::from_derive_input(&input).expect("Invalid arguments");
    let DeriveInput { ident, .. } = input;

    let tlv_type = args.tlv_type;
    let tlv_length = args.tlv_length;

    let output = quote! {
        impl TlvType for #ident {
            const TLV_TYPE: u8 = #tlv_type;
        }

        impl TlvLength for #ident {
            fn tlv_len(&self) -> usize {
                Self::TLV_LEN
            }

            fn tlv_len_is_constant() -> bool {
                true
            }
        }

        impl TlvMetadata for #ident {}

        impl TlvConstantMetadata for #ident {
            const TLV_LEN: usize = #tlv_length;
        }

        impl DecodeTlvUnchecked for #ident {
            fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
                let mut buffer = buffer.as_ref();
                let _type_byte = buffer.get_u8();
                let _len_byte = buffer.get_tlv_length();
                Self::decode_tlv_value_unchecked(buffer)
            }
        }

        impl TryEncodeTlv for #ident {
            fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
                let num_bytes = write_tlv(buffer, Self::TLV_TYPE, *self)?;
                Ok(num_bytes)
            }
        }
    };

    output.into()
}

#[derive(Debug, Default, FromDeriveInput)]
#[darling(attributes(tlv), forward_attrs(allow, doc, cfg))]
struct TlvMacroArgs {
    tlv_type: u8,
    clone: Option<darling::util::Flag>,
}

#[proc_macro_derive(Tlv, attributes(tlv))]
pub fn derive_tlv(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let args = TlvMacroArgs::from_derive_input(&input).expect("Invalid arguments");
    let DeriveInput { ident, .. } = input;

    let tlv_type = args.tlv_type;

    let self_expr = if args.clone.is_some() {
        quote! { (*self).clone() }
    } else {
        quote! { *self }
    };

    let output = quote! {
        impl TlvMetadata for #ident {}

        impl TlvType for #ident {
            const TLV_TYPE: u8 = #tlv_type;
        }

        impl DecodeTlvUnchecked for #ident {
            fn decode_tlv_unchecked(buffer: impl AsRef<[u8]>) -> Self {
                let mut buffer = buffer.as_ref();
                let _type_byte = buffer.get_u8();
                let _len_byte = buffer.get_tlv_length();
                Self::decode_tlv_value_unchecked(buffer)
            }
        }

        impl TryEncodeTlv for #ident {
            fn try_encode_tlv(&self, buffer: &mut [u8]) -> Result<usize, TwineTlvError> {
                let num_bytes = write_tlv(buffer, Self::TLV_TYPE, #self_expr)?;
                Ok(num_bytes)
            }
        }
    };

    output.into()
}
