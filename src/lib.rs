#![feature(const_type_name)]
#![feature(const_fn)]
#![feature(raw)]
#[doc(hidden)]
pub mod macros;
pub mod service;
pub mod argument;
pub mod error;
pub mod async_di;