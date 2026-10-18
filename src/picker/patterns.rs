use arg_picker_macros::internal_repeat;

use crate::{Pickable, PickerArg, PickerArgResult, PickerArgs};

internal_repeat!(1..=32 => {
    #[doc(hidden)]
    pub struct PickerPattern$<'a, (T$,+), Route>
    where (T$: Pickable<'a>,+)
    {
        pub args: PickerArgs<'a>,
        pub error_route: Option<Route>,
        (
            pub(crate) arg_$: PickerArg<'a, T$>,
            pub(crate) result_$: PickerArgResult<T$>,
            pub(crate) default_$: Option<Box<dyn FnOnce() -> T$>>,
            pub(crate) route_$: Option<Box<dyn FnOnce() -> Route>>,
            pub(crate) post_$: Option<Box<dyn FnOnce(T$) -> T$>>,
        +)
    }
});

/// A closure used as a fallback value or route.
///
/// Implemented for every `FnMut() -> R` where `R: Into<T>`, so a fallback may return any type
/// that converts into the expected one: `|| "aaa"` works where `String` is expected.
pub trait Fallback<T> {
    /// Produces the fallback value.
    fn fallback(&mut self) -> T;
}

impl<T, R, F> Fallback<T> for F
where
    F: FnMut() -> R,
    R: Into<T>,
{
    fn fallback(&mut self) -> T {
        self().into()
    }
}

internal_repeat!(1..=32 => {
    impl<'a, (T$,+), Route> PickerPattern$<'a, (T$,+), Route>
    where (T$: Pickable<'a>,+)
    {
        /// Sets a default value provider for this arg.
        ///
        /// If the arg is not provided by the user at runtime, the given closure will be
        /// called to produce a default value. The closure may return any type that converts
        /// into `T$`, so `|| "aaa"` works where `String` is expected.
        ///
        /// # Example
        ///
        #[allow(clippy::type_complexity)]
        pub fn or(mut self, mut func: impl Fallback<T$> + 'static) -> Self {
            self.default_$ = Some(Box::new(move || func.fallback()));
            self
        }

        /// Uses the default value for this arg's type if the arg is not provided.
        ///
        /// If the arg is not provided by the user at runtime, the default value for `T$`
        /// (as defined by the `Default` trait) will be used.
        ///
        /// # Example
        ///
        #[allow(clippy::type_complexity)]
        pub fn or_default(mut self) -> Self
        where
            T$: Default,
        {
            self.default_$ = Some(Box::new(|| T$::default()));
            self
        }

        /// Sets a route for when the arg is not provided.
        ///
        /// If the arg is not provided by the user at runtime, the given closure will be
        /// called to produce a route value that will be returned early. The closure may return
        /// any type that converts into `Route`.
        ///
        /// # Example
        ///
        pub fn or_route(mut self, mut func: impl Fallback<Route> + 'static) -> Self {
            self.route_$ = Some(Box::new(move || func.fallback()));
            self
        }


        /// Resets the route for this picker pattern, allowing a different route type.
        ///
        /// This method converts the current `PickerPattern` into a new one with a different
        /// route type `NewRoute`. All existing arg configurations, defaults, and post-
        /// processing functions are preserved, but the `error_route` and individual
        /// `route_$` fields are cleared (set to `None`).
        ///
        /// This is useful when you want to change the error/redirect route type mid-chain,
        /// for example when composing patterns from different contexts that use different
        /// route enums.
        #[allow(clippy::type_complexity)]
        pub fn with_route<NewRoute>(self) -> PickerPattern$<'a, (T$,+), NewRoute>
        {
            PickerPattern$ {
                args: self.args,
                error_route: None,
                (
                    arg_$: self.arg_$,
                    result_$: self.result_$,
                    default_$: self.default_$,
                    route_$: None,
                    post_$: self.post_$,
                +)
            }
        }

        /// Attaches a post-processing function to this arg.
        ///
        /// After the arg's value is parsed (or defaulted), the given closure will be
        /// invoked with the parsed value and its return value will be used as the final
        /// result. This allows transforming or validating the parsed value.
        ///
        /// # Example
        ///
        #[allow(clippy::type_complexity)]
        pub fn post<F>(mut self, func: F) -> Self
        where
            F: 'static + FnMut(T$) -> T$,
        {
            self.post_$ = Some(Box::new(func));
            self
        }
    }
});

internal_repeat!(1..32 => {
   impl<'a, (T$,+), Route> PickerPattern$<'a, (T$,+), Route>
   where (T$: Pickable<'a>,+)
   {
       #[allow(clippy::type_complexity)]
       /// Adds a new arg to the picking chain, returning a new `PickerPattern` with one more type parameter.
       ///
       /// This method extends the current picking pattern by appending an additional arg.
       /// The previous args and their results are preserved as part of the new pattern.
       /// The new arg's result is initially `Unparsed`.
       pub fn pick<N>(self, arg: impl Into<PickerArg<'a, N>>) -> PickerPattern$+<'a, (T$,+), N, Route>
       where
           N: Pickable<'a>,
       {
           PickerPattern$+ {
               // Args
               args: self.args,
               error_route: self.error_route,

               // Current
               arg_$+: arg.into(),
               result_$+: PickerArgResult::Unparsed,
               default_$+: None,
               route_$+: None,
               post_$+: None,

               // Prev
               (
                   arg_$: self.arg_$,
                   result_$: self.result_$,
                   default_$: self.default_$,
                   route_$: self.route_$,
                   post_$: self.post_$,
                +)
           }
       }

       /// Picks a new arg with a default value provider in a single call.
       ///
       /// This is a shorthand for calling `.pick(arg).or(func)`.
       ///
       /// # Example
       ///
       #[allow(clippy::type_complexity)]
       pub fn pick_or<N>(
           self,
           arg: impl Into<PickerArg<'a, N>>,
           func: impl Fallback<N> + 'static,
       ) -> PickerPattern$+<'a, (T$,+), N, Route>
       where
           N: Pickable<'a>,
       {
           self.pick(arg).or(func)
       }

       /// Picks a new arg with a default value in a single call.
       ///
       /// This is a shorthand for calling `.pick(arg).or_default()`.
       ///
       /// # Example
       ///
       #[allow(clippy::type_complexity)]
       pub fn pick_or_default<N>(self, arg: impl Into<PickerArg<'a, N>>) -> PickerPattern$+<'a, (T$,+), N, Route>
       where
           N: Pickable<'a> + Default,
       {
           self.pick(arg).or_default()
       }

       /// Picks a new arg with a route in a single call.
       ///
       /// This is a shorthand for calling `.pick(arg).or_route(func)`.
       ///
       /// # Example
       ///
       #[allow(clippy::type_complexity)]
       pub fn pick_or_route<N>(
           self,
           arg: impl Into<PickerArg<'a, N>>,
           func: impl Fallback<Route> + 'static,
       ) -> PickerPattern$+<'a, (T$,+), N, Route>
       where
           N: Pickable<'a>,
       {
           self.pick(arg).or_route(func)
       }
   }
});
