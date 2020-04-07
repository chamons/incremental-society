#![allow(clippy::unknown_clippy_lints)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::iter_nth_zero)]

pub mod console_ui;
pub mod engine;
pub mod state;

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_is_enum_variant;
