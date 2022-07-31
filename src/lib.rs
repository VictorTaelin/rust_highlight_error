#![cfg_attr(not(feature = "std"), no_std)]
//! Features:
//! - `std`: enabled by default. Enables std support. When it is disabled, the crate is `no_std`.
//! It will still need `alloc` though.

extern crate alloc;

mod highlight_error;

pub use highlight_error::{*};
