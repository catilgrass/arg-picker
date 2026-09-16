#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use proc_macro::TokenStream;

mod arg;
#[cfg(feature = "derive")]
mod derive;
mod internal_repeat;

/// Root path the generated code reaches `arg-picker` through.
///
/// With `mingling_support`, that is mingling's re-export of this crate, so a user who
/// depends on mingling alone can use the macros; without it, it is `arg-picker`
/// itself. Every generated path — in [`arg`] and in the `Pickable` derive — is built
/// from this, so the two cannot disagree about which crate they mean.
pub(crate) fn picker_root() -> proc_macro2::TokenStream {
    #[cfg(feature = "mingling_support")]
    {
        quote::quote! { ::mingling::picker }
    }

    #[cfg(not(feature = "mingling_support"))]
    {
        quote::quote! { ::arg_picker }
    }
}

/// Core proc-macro: repeats a template body `count` times.
///
/// Internal call signature: `internal_repeat!(count => { template })`
#[proc_macro]
pub fn internal_repeat(input: TokenStream) -> TokenStream {
    internal_repeat::internal_repeat(input)
}

/// Quick builder for `PickerArg`.
///
/// # Syntax
///
/// ```ignore
/// use arg_picker_macros::flag;
///
/// let basic = arg![name: String];
/// let with_short_name = arg![name: String, 'n'];
/// let with_short_alias = arg![name: String, 'n', "alias"];
/// let positional = arg![String];
/// let positional_with_name = arg![String, 'n', "alias"];
/// ```
#[proc_macro]
pub fn arg(input: TokenStream) -> TokenStream {
    arg::arg(input)
}

/// Derives `Pickable` for structs and `SinglePickable` for unit-only enums.
///
/// Only available when the `derive` feature is enabled.
#[cfg(feature = "derive")]
#[proc_macro_derive(Pickable, attributes(arg))]
pub fn derive_pickable(input: TokenStream) -> TokenStream {
    derive::derive_pickable(input)
}
