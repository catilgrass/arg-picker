//! Shortcut entry points for parsing positional arguments and invoking a
//! callable with the parsed values.

use crate::{IntoPicker, IntoPickerFunction, Picker};

impl<'a> Picker<'a> {
    /// Parses `args` as positional arguments and calls `func` with the parsed
    /// values, returning `func`'s result.
    ///
    /// The argument types — and therefore the expected number of positional
    /// arguments — are inferred from `func`; every remaining type parameter is
    /// `()`.
    ///
    /// # Positional only
    ///
    /// This is a convenience for small scripts, so it only understands
    /// **positional** arguments, consumed strictly in order. Flags, named
    /// arguments, defaults and routes are not supported — use a `pick(..)`
    /// chain for those.
    ///
    /// # Panics
    ///
    /// Panics if the positional arguments are missing or fail to parse.
    ///
    /// # Examples
    ///
    /// ```
    /// use arg_picker::Picker;
    ///
    /// let sum = Picker::invoke(vec!["1", "2", "3"], |a: i32, b: i32, c: i32| a + b + c);
    /// assert_eq!(sum, 6);
    /// ```
    ///
    /// > ⚠️ Warning:
    /// >
    /// > Experimentation: `Picker::invoke` and the other shortcut
    /// > entry points here are experimental, so their names, signatures and
    /// > behavior may change in future releases.
    pub fn invoke<Arg, F, Marker>(args: Arg, func: F) -> F::Ret
    where
        Arg: IntoPicker<'a>,
        F: IntoPickerFunction<'a, Marker>,
    {
        func.invoke_with(args)
    }

    /// Like [`Picker::invoke`], but takes the arguments from
    /// [`std::env::args`] (skipping the program name).
    ///
    /// This is the “one-liner” form for small scripts:
    ///
    /// ```no_run
    /// use arg_picker::Picker;
    ///
    /// // Reads `std::env::args()`; panics on missing/malformed arguments.
    /// let doubled = Picker::invoke_args(|n: i32| n * 2);
    /// ```
    ///
    /// See [`Picker::invoke`] for the positional-only and panic caveats.
    ///
    /// > ⚠️ Warning:
    /// >
    /// > Experimentation: `Picker::invoke_args` and the other
    /// > shortcut entry points here are experimental, so their names,
    /// > signatures and behavior may change in future releases.
    pub fn invoke_args<F, Marker>(func: F) -> F::Ret
    where
        F: IntoPickerFunction<'a, Marker>,
    {
        func.invoke_with(Picker::from_args().into_args())
    }
}
