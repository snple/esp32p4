//! Procedural macros for `esp32p4`.
//!
//! The runtime's `_start` calls a symbol named `main` with the C calling
//! convention. [`macro@main`] turns an ordinary Rust `fn main() -> !` into that
//! symbol and runs the core-level start-up code first.
//!
//! [`macro@ramfunc`], [`macro@tcmfunc`] and [`macro@tcmdata`] only change where
//! a function or static is linked: SRAM instead of flash, or the 8 KB HP
//! scratchpad ("TCM") at `0x3010_0000`.

mod ramfunc;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType, Type, parse_macro_input, parse_quote};

/// Marks the application entry point.
///
/// ```ignore
/// #[esp32p4_hal::main]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
///
/// Expands to a `#[unsafe(no_mangle)] extern "C" fn main()` that first calls
/// [`esp32p4_hal::startup::init`] (watchdogs, core 1, branch predictor) and then
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
        .insert(0, parse_quote!(::esp32p4_hal::startup::init();));

    quote!(#function).into()
}

/// Places a function in SRAM instead of flash.
///
/// ```ignore
/// #[esp32p4_hal::ramfunc]
/// fn spin(micros: u32) {
///     // Runs with the flash cache suspended.
/// }
/// ```
///
/// The code still lives in the flash image; the 2nd-stage bootloader copies it
/// to the `.ramfunc` section of the RAM region before `_start`. Call it from
/// code that runs while the flash cache is off — an MSPI re-tune, a CPU
/// frequency switch, or a flash operation.
///
/// The attribute adds `#[inline(never)]`, because an inlined copy would sit in
/// the caller's section and silently run from flash. Anything the function calls
/// stays in flash unless it is marked too; only the marked function itself is
/// copied.
#[proc_macro_attribute]
pub fn ramfunc(attr: TokenStream, item: TokenStream) -> TokenStream {
    ramfunc::ramfunc_impl(attr, item)
}

/// Places a function in the 8 KB HP scratchpad, i.e. the TCM.
///
/// ```ignore
/// #[esp32p4_hal::tcmfunc]
/// fn sample() -> u32 {
///     // Runs from 0x3010_0000, two cycles per access, no cache involved.
/// }
/// ```
///
/// The scratchpad is the only RAM the CPU reaches over its own instruction and
/// data buses rather than through a cache, so this is where a latency-critical
/// handler belongs. It is small — 8 KB in total — and the linker fails when the
/// section does not fit.
///
/// Like [`macro@ramfunc`], the attribute adds `#[inline(never)]` and only places
/// the function itself, not what it calls.
#[proc_macro_attribute]
pub fn tcmfunc(attr: TokenStream, item: TokenStream) -> TokenStream {
    ramfunc::tcmfunc_impl(attr, item)
}

/// Places a static in the 8 KB HP scratchpad, i.e. the TCM.
///
/// ```ignore
/// #[esp32p4_hal::tcmdata]
/// static SAMPLES: [u32; 64] = [0; 64];
/// ```
///
/// The same section (`0x3010_0000`, two cycles per access, no cache) as
/// [`macro@tcmfunc`], so both share the 8 KB. The initial value lives in flash
/// in the image and the bootloader copies it in before `_start`; unlike `.bss`
/// there is no zero-fill, so a `static` with an initialiser is the useful case.
#[proc_macro_attribute]
pub fn tcmdata(attr: TokenStream, item: TokenStream) -> TokenStream {
    ramfunc::tcmdata_impl(attr, item)
}
