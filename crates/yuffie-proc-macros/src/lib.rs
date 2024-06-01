// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Rust procedural macros for Yuffie.

use proc_macro::TokenStream;
use syn::spanned::Spanned;

/// Attribute to declare the entry point of a UEFI image.
///
/// The entry point should be declared as:
///
/// ```rust
/// use yuffie::prelude::*;
///
/// #[entry]
/// fn main(handle: Handle, st: &mut SystemTable) -> Status {
///     // ...
/// }
/// ```
///
/// The entry point name can be anything. `main` is used as a convention.
/// The exported symbol for the function will be `efi_main`, which the
/// `*-unknown-uefi` targets look for as the entry point for binaries.
///
/// ## References
///
/// - [UEFI Specification, Version 2.10][UEFI Spec]
///   - 4.1 UEFI Image Entry Point
/// - [UEFI Platform Integration Specification, version 1.8][UEFI PI]
///   - II-4.2 UEFI Image Entry Point Examples
///
/// [UEFI Spec]: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_10_Aug29.pdf
/// [UEFI PI]: https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf
#[proc_macro_attribute]
pub fn entry(args: TokenStream, stream: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(stream as syn::ItemFn);

    if !args.is_empty() {
        return syn::Error::new(input.span(), "attribute does not take any arguments")
            .to_compile_error()
            .into();
    }

    let attrs = &input.attrs;
    let sig = &input.sig;
    let ident = &sig.ident;
    let block = &input.block;

    // Validate the signature
    if sig.constness.is_some() {
        return syn::Error::new(sig.constness.span(), "entry point must not be `const`")
            .to_compile_error()
            .into();
    }
    if sig.asyncness.is_some() {
        return syn::Error::new(sig.asyncness.span(), "entry point must not be `async`")
            .to_compile_error()
            .into();
    }
    if sig.unsafety.is_some() {
        return syn::Error::new(sig.unsafety.span(), "entry point must not be `unsafe`")
            .to_compile_error()
            .into();
    }
    if sig.abi.is_some() {
        return syn::Error::new(sig.abi.span(), "entry point must not declare an ABI")
            .to_compile_error()
            .into();
    }
    if !sig.generics.params.is_empty() {
        return syn::Error::new(sig.span(), "entry point must not use generics")
            .to_compile_error()
            .into();
    }
    if sig.variadic.is_some() {
        return syn::Error::new(sig.span(), "entry point must not be variadic")
            .to_compile_error()
            .into();
    }

    #[rustfmt::skip]
    let tok = TokenStream::from(quote::quote! {
        #(#attrs)*
        #[export_name = "efi_main"]
        extern "efiapi" #sig
        #block

        // Check inputs and return type
        const _: ::yuffie::ImageEntryFn = #ident;
    });

    tok
}
