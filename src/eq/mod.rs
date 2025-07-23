extern crate alloc;

#[doc(hidden)]
pub use alloc::boxed::Box;
use core::any::Any;
pub use crate::{
    __internal_eq_trait_object,
    __internal_eq_trait_object_alloc
};

mod macros;

pub trait DynEq: Any + private::Sealed {
    #[doc(hidden)]
    fn as_any(&self) -> &dyn Any;

    #[doc(hidden)]
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}

impl<T: Eq + 'static> DynEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref().map_or(false, |other| self == other)
    }
}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for T where T: PartialEq {}
}
