#[macro_export]
macro_rules! eq_trait_object {
	($($path:tt)+) => {
		$crate::eq::__internal_eq_trait_object!(begin $($path)+);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_eq_trait_object {
	(begin < $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(generics () () $($rest)*);
	};

	(begin $first:tt $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(path () ($first) $($rest)*);
	};

	(generics ($($generics:tt)*) () > $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(path ($($generics)*) () $($rest)*);
	};

	(generics ($($generics:tt)*) ($($brackets:tt)*) < $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(generics ($($generics)* <) ($($brackets)* <) $($rest)*);
	};

	(generics ($($generics:tt)*) (< $($brackets:tt)*) > $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(generics ($($generics)* >) ($($brackets)*) $($rest)*);
	};

	(generics ($($generics:tt)*) ($($brackets:tt)*) $first:tt $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(generics ($($generics)* $first) ($($brackets)*) $($rest)*);
	};

	// End with `where` clause.
	(path ($($generics:tt)*) ($($path:tt)*) where $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(impl ($($generics)*) ($($path)*) ($($rest)*));
	};

	// End without `where` clause.
	(path ($($generics:tt)*) ($($path:tt)*)) => {
		$crate::eq::__internal_eq_trait_object!(impl ($($generics)*) ($($path)*) ());
	};

	// Token inside of path.
	(path ($($generics:tt)*) ($($path:tt)*) $first:tt $($rest:tt)*) => {
		$crate::eq::__internal_eq_trait_object!(path ($($generics)*) ($($path)* $first) $($rest)*);
	};

	// The impl.
	(impl ($($generics:tt)*) ($($path:tt)*) ($($bound:tt)*)) => {
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + 'eq) where $($bound)* {
			fn eq(&self, other: &Self) -> bool {
				self.dyn_eq(DynEq::as_any(other))
			}
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + ::core::marker::Send + 'eq) where $($bound)* {
			fn eq(&self, other: &Self) -> bool {
				self.dyn_eq(DynEq::as_any(other))
			}
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + ::core::marker::Sync + 'eq) where $($bound)* {
			fn eq(&self, other: &Self) -> bool {
				self.dyn_eq(DynEq::as_any(other))
			}
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + ::core::marker::Send + ::core::marker::Sync + 'eq) where $($bound)* {
			fn eq(&self, other: &Self) -> bool {
				self.dyn_eq(DynEq::as_any(other))
			}
		}

		$crate::eq::__internal_eq_trait_object_alloc!(($($generics)*) ($($path)*) ($($bound)*));

		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + 'eq) where $($bound)* {}
		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + ::core::marker::Send + 'eq) where $($bound)* {}
		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + ::core::marker::Sync + 'eq) where $($bound)* {}
		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + ::core::marker::Send + ::core::marker::Sync + 'eq) where $($bound)* {}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_eq_trait_object_alloc {
	(($($generics:tt)*) ($($path:tt)*) ($($bound:tt)*)) => {
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::eq::Box<dyn $($path)* + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::eq::Box<dyn $($path)* + ::core::marker::Send + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::eq::Box<dyn $($path)* + ::core::marker::Sync + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::eq::Box<dyn $($path)* + ::core::marker::Send + ::core::marker::Sync + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}
	}
}