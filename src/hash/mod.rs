#[cfg(doc)]
extern crate core as std;

mod macros;

use core::hash::{Hash, Hasher};
use crate::hash_trait_object;
pub use crate::__internal_hash_trait_object;

pub trait DynHash: sealed::Sealed {
    fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<T: Hash + ?Sized> DynHash for T {
    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        Hash::hash(self, &mut state);
    }
}

hash_trait_object!(DynHash);
#[doc(hidden)]
pub mod __private {
    pub use core::hash::{Hash, Hasher};
    pub use core::marker::{Send, Sync};
}

mod sealed {
    use core::hash::Hash;

    pub trait Sealed {}
    impl<T: Hash + ?Sized> Sealed for T {}
}
