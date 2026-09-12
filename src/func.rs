// The generated variants wrap boxed `FnOnce`s with up to 32 arguments, which
// trips `clippy::type_complexity`; this is inherent to the type, not a smell.
#![allow(clippy::type_complexity)]

use arg_picker_macros::internal_repeat;

use crate::{IntoPicker, Pickable, PickerArg};

/// Helper trait that powers [`PickerFunction::from`], [`Picker::invoke`] and
/// [`Picker::invoke_args`](crate::Picker::invoke_args).
///
/// It is implemented once per supported arity, with `Args` set to the tuple of
/// the callable's argument types. Because a 1-tuple, 2-tuple, … are distinct
/// type constructors, the impls stay disjoint for the compiler — which is what
/// allows a single `from`/`invoke` to accept closures of any arity.
///
/// The lifetime `'a` is the one used to parse the arguments as [`Pickable`]
/// values.
#[doc(hidden)]
pub trait IntoPickerFunction<'a, Args> {
    /// The [`PickerFunction`] produced by this callable.
    type Output;

    /// The value returned when the callable is invoked.
    type Ret;

    /// Converts this callable into its [`PickerFunction`] representation.
    fn into_picker_function(self) -> Self::Output;

    /// Parses `args` as positional arguments and calls this callable.
    fn invoke_with<PArgs>(self, args: PArgs) -> Self::Ret
    where
        PArgs: IntoPicker<'a>;
}

// `PickerFunction` is a single enum with one shared type-parameter list
// (`T0`..=`T31`, `R`) but 32 variants. `internal_repeat!` emits its body once
// per index, so we use the degenerate `32..=32` range to emit the body a single
// time; inside that body the `( …, +)` repetition groups expand once per arity,
// producing the type parameters, the variants, and each variant's argument list.
internal_repeat!(32..=32 => {
    /// A dynamically-dispatched callable that can be invoked with between 1 and 32 arguments.
    ///
    /// Each variant wraps a boxed [`FnOnce`] closure accepting exactly the number of
    /// arguments indicated by the variant's name, and returns a value of type `R`.
    /// This enum is typically used in "picker" APIs where the arity of the callback
    /// is not known at compile time, allowing implementations to dispatch to the
    /// correct arity and call the underlying function with the appropriate arguments.
    ///
    /// Type parameters that are not consumed by a given arity are set to `()`.
    ///
    /// # Type Parameters
    ///
    /// * `T0`..=`T31` - The argument types of the callable, used positionally.
    /// * `R` - The common return type produced by every variant.
    pub enum PickerFunction<
        (
            T$-,
        +)
        , R,
    > {
        (
            /// A callable taking the number of arguments indicated by the variant's
            /// name, and returning `R`.
            With$Arg(Box<dyn FnOnce(
                (
                    T$-,
                +)
            ) -> R>),
        +)
    }
});

internal_repeat!(32..=32 => {
    impl<
        (
            T$-,
        +)
        , R,
    > PickerFunction<
        (
            T$-,
        +)
        , R,
    > {
        /// Builds a [`PickerFunction`] from a callable.
        ///
        /// The argument types are inferred from the callable, and every type
        /// parameter beyond the callable's arity is set to `()`:
        ///
        /// ```
        /// use arg_picker::PickerFunction;
        ///
        /// let add = PickerFunction::from(|a: i32, b: i32| (a + b).to_string());
        /// match add {
        ///     PickerFunction::With2Arg(f) => assert_eq!(f(1, 2), "3"),
        ///     _ => unreachable!(),
        /// }
        /// ```
        // We cannot express this with the `From` trait: per-arity `From<F>`
        // impls collide in coherence, since `PickerFunction<T0, ()>` and
        // `PickerFunction<T0, T1>` unify when `T1 = ()`.
        #[allow(clippy::should_implement_trait)]
        pub fn from<'a, F, Marker>(func: F) -> Self
        where
            F: IntoPickerFunction<'a, Marker, Output = Self>,
        {
            func.into_picker_function()
        }
    }
});

internal_repeat!(1..=32 => {
    impl<
        'a,
        (
            T$-,
        +)
        , R,
        F,
    > IntoPickerFunction<'a, (
        (
            T$-,
        +)
        ,
    )> for F
    where
        F: FnOnce(
            (
                T$-,
            +)
        ) -> R + 'static,
        (
            T$-: Pickable<'a> + 'a,
        +)
    {
        type Output = PickerFunction<
            (
                T$-,
            +)
            (
                ,
                ()
            +)
            ^
            , R,
        >;

        type Ret = R;

        fn into_picker_function(self) -> Self::Output {
            PickerFunction::With$Arg(Box::new(self))
        }

        fn invoke_with<PArgs>(self, args: PArgs) -> Self::Ret
        where
            PArgs: IntoPicker<'a>,
        {
            let ((
                v$-,
            +)) = args
                (
                    .pick(&PickerArg::< T$- > {
                        full: &[],
                        short: None,
                        positional: true,
                        internal_type: ::std::marker::PhantomData,
                    })
                +)
                .unwrap();
            (self)((
                v$-,
            +))
        }
    }
});
