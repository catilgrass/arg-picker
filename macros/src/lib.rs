#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use proc_macro::TokenStream;

mod arg;
mod internal_repeat;
#[cfg(feature = "derive")]
mod pickialize;

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
    pickialize::derive_pickable(input)
}
