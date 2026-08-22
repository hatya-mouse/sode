//! # sode
//!
//! `sode` is a simple and small binary decoding / encoding crate.
//!
//! ## Example
//!
//! ```rust
//! use sode::{Decode, ValueDecoder};
//!
//! struct User {
//!     name: String,
//!     age: u32,
//!     id: u64,
//! }
//!
//! impl Decode for User {
//!     fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
//!         let name = String::decode(d)?;
//!         let age = u32::decode(d)?;
//!         let id = u64::decode(d)?;
//!         Ok(User { name, age, id })
//!     }
//! }
//! ```

/// The definition of `Decode` trait, `FieldDecoder` and `ValueDecoder` structs.
mod decode;
/// The definition of `Encode` trait and `Encoder` struct.
mod encode;
/// The implementetion of `Decode` and `Encode` traits for primitive types.
mod impls;
pub(crate) mod utils;

pub use decode::{Decode, DecodeError, FieldDecoder, ValueDecoder};
pub use encode::{Encode, EncodeError, Encoder};
