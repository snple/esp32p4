//! Implementation helpers for the `#[ramfunc]` and `#[tcmfunc]` attributes.
//!
//! Both attributes only change where a function is *placed*: the code still
//! lives in the flash image and the bootloader copies it into RAM before
//! `_start`, so the addresses inside the image and at run time differ.

use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Attribute, ItemFn, ItemStatic, parse_macro_input};

/// Implementation of `#[ramfunc]`: place the function in `.ramfunc`, which the
/// linker script puts in the SRAM region.
pub fn ramfunc_impl(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    place(".ramfunc", "`#[ramfunc]`", args, input)
}

/// Implementation of `#[tcmfunc]`: place the function in `.tcm.text`, which the
/// linker script puts in the 8 KB HP scratchpad at `0x3010_0000`.
pub fn tcmfunc_impl(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    place(".tcm.text", "`#[tcmfunc]`", args, input)
}

/// Implementation of `#[tcmdata]`: place a static in `.tcm.data`, which the
/// linker script puts in the 8 KB HP scratchpad at `0x3010_0000`.
pub fn tcmdata_impl(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(input as ItemStatic);

    if !args.is_empty() {
        return syn::Error::new(Span::call_site(), "`#[tcmdata]` takes no arguments")
            .to_compile_error()
            .into();
    }

    if let Some(attr) = item.attrs.iter().find(|a| has_link_section(a)) {
        return syn::Error::new_spanned(
            attr,
            "`#[tcmdata]` already places this item in `.tcm.data`; remove the explicit \
             `link_section`",
        )
        .to_compile_error()
        .into();
    }

    let section_attr: Attribute = syn::parse_quote!(#[unsafe(link_section = ".tcm.data")]);
    item.attrs.insert(0, section_attr);

    quote!(#item).into()
}

fn place(
    section: &str,
    attribute: &str,
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    if !args.is_empty() {
        return syn::Error::new(Span::call_site(), format!("{attribute} takes no arguments"))
            .to_compile_error()
            .into();
    }

    // An `async fn` only returns a future; the body — the code that actually
    // runs — ends up in the generated `poll`, which stays wherever the compiler
    // puts it. Placing the wrapper would look like it worked.
    if let Some(asyncness) = &f.sig.asyncness {
        return syn::Error::new_spanned(
            asyncness,
            format!(
                "{attribute} cannot be applied to an `async fn`: the body runs in the generated \
                 future's `poll`, which is not placed in `{section}`"
            ),
        )
        .to_compile_error()
        .into();
    }

    // An inlined copy lives in the caller's section, so a hint would make the
    // code run from flash without any diagnostic.
    if let Some(attr) = f.attrs.iter().find(|a| inline_hint(a) == Some(false)) {
        return syn::Error::new_spanned(
            attr,
            format!(
                "{attribute} implies `#[inline(never)]`: an inlined copy lives in the caller's \
                 section and would run from flash. Call this function from another placed \
                 function, or drop {attribute} if it does not need to run from RAM"
            ),
        )
        .to_compile_error()
        .into();
    }

    if let Some(attr) = f.attrs.iter().find(|a| has_link_section(a)) {
        return syn::Error::new_spanned(
            attr,
            format!(
                "{attribute} already places this function in `{section}`; remove the explicit \
                 `link_section`"
            ),
        )
        .to_compile_error()
        .into();
    }

    // In Rust 2024 edition, link_section must use `unsafe(link_section = ...)`.
    let section_attr: Attribute = syn::parse_quote!(#[unsafe(link_section = #section)]);

    // Insert at the beginning so the user's own attributes stay visible.
    f.attrs.insert(0, section_attr);

    // Small functions are otherwise inlined into their callers at `opt-level 3`
    // (`release` also enables fat LTO), which would silently leave the body in
    // flash.
    if !f.attrs.iter().any(|a| inline_hint(a) == Some(true)) {
        let no_inline_attr: Attribute = syn::parse_quote!(#[inline(never)]);
        f.attrs.insert(0, no_inline_attr);
    }

    quote!(#f).into()
}

/// Classifies `#[inline]`-family attributes.
///
/// `Some(true)` for `#[inline(never)]`, `Some(false)` for `#[inline]` and
/// `#[inline(always)]`, `None` for anything else.
fn inline_hint(attr: &Attribute) -> Option<bool> {
    if !attr.path().is_ident("inline") {
        return None;
    }
    Some(attr.meta.to_token_stream().to_string().contains("never"))
}

/// Matches `#[link_section = "..."]` and its edition-2024 form,
/// `#[unsafe(link_section = "...")]`.
fn has_link_section(attr: &Attribute) -> bool {
    if attr.path().is_ident("link_section") {
        return true;
    }
    if !attr.path().is_ident("unsafe") {
        return false;
    }
    // `#[unsafe(...)]` keeps the attribute it wraps in the token stream, so
    // `meta` reads as `unsafe(link_section = "...")`.
    attr.meta
        .to_token_stream()
        .to_string()
        .contains("link_section")
}
