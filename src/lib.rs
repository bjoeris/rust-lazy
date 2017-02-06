#![deny(missing_docs, warnings)]
#![feature(optin_builtin_traits)]
// #![feature(core, std_misc, unsafe_destructor, optin_builtin_traits)]

//! Lazy evaluation for Rust.

#[macro_use(debug_unreachable)]
extern crate debug_unreachable;

extern crate oncemutex;

mod invoke;

/// A Thunk safe for single-threaded access.
pub mod single;

/// A Thunk safe for multi-threaded use.
// pub mod sync;

mod lazy {
    pub use super::*;
}

#[macro_export]
macro_rules! lazy {
    ($e:expr) => {
        $crate::single::Thunk::new(move || { $e })
    }
}

// #[macro_export]
// macro_rules! sync_lazy {
//     ($e:expr) => {
//         $crate::sync::Thunk::new(move || { $e })
//     }
// }

