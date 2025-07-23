use crate::clone::DynClone;

#[macro_export]
macro_rules! clone_trait_object {
    ($($path:tt)+) => {
        $crate::clone::__internal_clone_trait_object!(begin $($path)+);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_clone_trait_object {
    // Invocation started with `<`, parse generics.
    (begin < $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(generics () () $($rest)*);
    };

    // Invocation did not start with `<`.
    (begin $first:tt $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(path () ($first) $($rest)*);
    };

    // End of generics.
    (generics ($($generics:tt)*) () > $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(path ($($generics)*) () $($rest)*);
    };

    // Generics open bracket.
    (generics ($($generics:tt)*) ($($brackets:tt)*) < $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(generics ($($generics)* <) ($($brackets)* <) $($rest)*);
    };

    // Generics close bracket.
    (generics ($($generics:tt)*) (< $($brackets:tt)*) > $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(generics ($($generics)* >) ($($brackets)*) $($rest)*);
    };

    // Token inside of generics.
    (generics ($($generics:tt)*) ($($brackets:tt)*) $first:tt $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(generics ($($generics)* $first) ($($brackets)*) $($rest)*);
    };

    // End with `where` clause.
    (path ($($generics:tt)*) ($($path:tt)*) where $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(impl ($($generics)*) ($($path)*) ($($rest)*));
    };

    // End without `where` clause.
    (path ($($generics:tt)*) ($($path:tt)*)) => {
        $crate::clone::__internal_clone_trait_object!(impl ($($generics)*) ($($path)*) ());
    };

    // Token inside of path.
    (path ($($generics:tt)*) ($($path:tt)*) $first:tt $($rest:tt)*) => {
        $crate::clone::__internal_clone_trait_object!(path ($($generics)*) ($($path)* $first) $($rest)*);
    };

    // The impl.
    (impl ($($generics:tt)*) ($($path:tt)*) ($($bound:tt)*)) => {
        #[allow(unknown_lints, non_local_definitions)] // false positive: https://github.com/rust-lang/rust/issues/121621
        impl<'clone, $($generics)*> $crate::clone::__private::Clone for $crate::clone::__private::Box<dyn $($path)* + 'clone> where $($bound)* {
            fn clone(&self) -> Self {
                $crate::clone::clone_box(&**self)
            }
        }

        #[allow(unknown_lints, non_local_definitions)]
        impl<'clone, $($generics)*> $crate::clone::__private::Clone for $crate::clone::__private::Box<dyn $($path)* + $crate::clone::__private::Send + 'clone> where $($bound)* {
            fn clone(&self) -> Self {
                $crate::clone::clone_box(&**self)
            }
        }

        #[allow(unknown_lints, non_local_definitions)]
        impl<'clone, $($generics)*> $crate::clone::__private::Clone for $crate::clone::__private::Box<dyn $($path)* + $crate::clone::__private::Sync + 'clone> where $($bound)* {
            fn clone(&self) -> Self {
                $crate::clone::clone_box(&**self)
            }
        }

        #[allow(unknown_lints, non_local_definitions)]
        impl<'clone, $($generics)*> $crate::clone::__private::Clone for $crate::clone::__private::Box<dyn $($path)* + $crate::clone::__private::Send + $crate::clone::__private::Sync + 'clone> where $($bound)* {
            fn clone(&self) -> Self {
                $crate::clone::clone_box(&**self)
            }
        }
    };
}

clone_trait_object!(DynClone);
