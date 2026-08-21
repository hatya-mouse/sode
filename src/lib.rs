/// The definition of `Decode` trait, `FieldDecoder` and `ValueDecoder` structs.
mod decode;
/// The definition of `Encode` trait and `Encoder` struct.
mod encode;
/// The implementetion of `Decode` and `Encode` traits for primitive types.
mod impls;
pub(crate) mod utils;

pub use decode::{Decode, DecodeError, FieldDecoder, ValueDecoder};
pub use encode::{Encode, EncodeError, Encoder};
