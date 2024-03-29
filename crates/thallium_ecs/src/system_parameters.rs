use crate::system::{Borrow, SystemRunState};

/// The trait for parameters to [`SystemFunction`](crate::SystemFunction)s
pub trait SystemParameter: Send + Sync {
    /// The type that this trait is implemented on, but with a different lifetime
    type This<'this>;
    /// The lock returned by [`SystemParameter::lock`]
    type Lock<'state>;

    /// Locks any state required for this [`SystemParameter`]
    fn lock<'state>(state: &SystemRunState<'state>) -> Self::Lock<'state>;
    /// Constructs the [`SystemParameter`] from a lock
    fn construct<'this>(state: &'this mut Self::Lock<'_>, last_run_tick: u64) -> Self::This<'this>;
    /// Returns an iterator over all [`Resource`](crate::Resource) types that this system parameter will lock
    fn get_resource_types() -> impl Iterator<Item = Borrow>;
    /// Returns an iterator over all [`Component`](crate::Component) types that this system parameter will lock
    fn get_component_types() -> impl Iterator<Item = Borrow>;
}

macro_rules! system_parameter_tuple {
    ($($param:ident),*) => {
        impl<$($param),*> SystemParameter for ($($param,)*)
        where
            $($param: SystemParameter,)*
        {
            type This<'this> = ($($param::This<'this>,)*);
            type Lock<'state> = ($($param::Lock<'state>,)*);

            #[allow(clippy::unused_unit)]
            fn lock<'state>(state: &SystemRunState<'state>) -> Self::Lock<'state> {
                _ = state;
                ($($param::lock(state),)*)
            }

            #[allow(clippy::unused_unit)]
            fn construct<'this>(state: &'this mut Self::Lock<'_>, last_run_tick: u64) -> Self::This<'this> {
                _ = last_run_tick;
                #[allow(non_snake_case)]
                let ($($param,)*) = state;
                ($($param::construct($param, last_run_tick),)*)
            }

            fn get_resource_types() -> impl Iterator<Item = Borrow> {
                std::iter::empty()
                    $(
                        .chain($param::get_resource_types())
                    )*
            }

            fn get_component_types() -> impl Iterator<Item = Borrow> {
                std::iter::empty()
                    $(
                        .chain($param::get_component_types())
                    )*
            }
        }
    };
}

system_parameter_tuple!();
system_parameter_tuple!(A);
system_parameter_tuple!(A, B);
system_parameter_tuple!(A, B, C);
system_parameter_tuple!(A, B, C, D);
system_parameter_tuple!(A, B, C, D, E);
system_parameter_tuple!(A, B, C, D, E, F);
system_parameter_tuple!(A, B, C, D, E, F, G);
system_parameter_tuple!(A, B, C, D, E, F, G, H);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I, J);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I, J, K);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
system_parameter_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
