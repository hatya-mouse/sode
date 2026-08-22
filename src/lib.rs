//! # sode
//!
//! `sode` is a simple and small binary decoding / encoding crate.
//!
//! ## Example
//!
//! ```rust
//! use sode::{Encode, Encoder, EncodeError, Decode, ValueDecoder, DecodeError};
//!
//! #[derive(Debug, PartialEq)]
//! struct User {
//!     name: String,
//!     age: u32,
//!     id: u64,
//! }
//!
//! impl Encode for User {
//!     fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
//!         e.field(0, &self.name)?;
//!         e.field(1, &self.age)?;
//!         e.field(2, &self.id)?;
//!         Ok(())
//!     }
//! }
//!
//! impl Decode for User {
//!     fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
//!         let d = d.to_field_decoder()?;
//!
//!         Ok(User {
//!             name: d.field(0)?.unwrap_or_default(),
//!             age: d.field(1)?.unwrap_or_default(),
//!             id: d.field(2)?.unwrap_or_default(),
//!         })
//!     }
//! }
//!
//! let user = User {
//!     name: "Alice".to_string(),
//!     age: 30,
//!     id: 256,
//! };
//!
//! // Encode the user struct into bytes
//! let bytes = sode::encode(&user).unwrap();
//!
//! // Decode the bytes back into a user struct
//! let decoded_user = sode::decode::<User>(&bytes, 1).unwrap();
//!
//! assert_eq!(user, decoded_user);
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

/// Encodes the given value into a binary representation.
///
/// # Parameter
/// - `value`: The value to be encoded.
pub fn encode<T: Encode>(value: &T) -> Result<Vec<u8>, EncodeError> {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.take_bytes())
}

/// Decodes the given bytes into a value of type `T`.
///
/// # Parameters
/// - `bytes`: The bytes to be decoded.
/// - `version`: The version of the encoding format.
pub fn decode<T: Decode>(bytes: &[u8], version: u64) -> Result<T, DecodeError> {
    let mut decoder = ValueDecoder::from_bytes(bytes, version)?;
    T::decode(&mut decoder)
}
