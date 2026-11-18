//! Procedural macros for `esp32p4`.
//!
//! The runtime's `_start` calls a symbol named `main` with the C calling
//! convention. [`macro@main`] turns an ordinary Rust `fn main() -> !` into that
//! symbol and runs the core-level start-up code first.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType, Type, parse_macro_input, parse_quote};

/// Marks the application entry point.
///
/// ```ignore
/// #[esp32p4::main]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
///
/// Expands to a `#[unsafe(no_mangle)] extern "C" fn main()` that first calls
/// [`esp32p4::startup::init`] (watchdogs, core 1, branch predictor) and then
/// runs the function body.
///
/// The function must take no arguments and return `!`.
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if !function.sig.inputs.is_empty() {
        return syn::Error::new_spanned(
            &function.sig.inputs,
            "#[main] function must take no arguments",
        )
        .to_compile_error()
        .into();
    }

    match &function.sig.output {
        ReturnType::Type(_, ty) if matches!(**ty, Type::Never(_)) => {}
        output => {
            return syn::Error::new_spanned(output, "#[main] function must return `!`")
                .to_compile_error()
                .into();
        }
    }

    function.vis = parse_quote!(pub);
    function.sig.ident = syn::Ident::new("main", function.sig.ident.span());
    function.sig.abi = Some(parse_quote!(extern "C"));
    function.attrs.push(parse_quote!(#[unsafe(no_mangle)]));
    function
        .block
        .stmts
        .insert(0, parse_quote!(::esp32p4::startup::init();));

    quote!(#function).into()
}
