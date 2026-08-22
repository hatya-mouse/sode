/// A trait for types that are encodable using an encoder.
pub trait Encode {
    /// Encodes the payload using the given encoder.
    ///
    /// # Parameters
    /// - `e`: A mutable reference to the encoder to use for encoding.
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError>;
}

/// An encoder for encoding value to bytes.
/// Use `field` to encode a field with a given ID and value, or write_* functions to append raw bytes to the encoder's byte vector.
///
/// # Example
/// ```rust
/// use sode::{Encode, Encoder, EncodeError};
///
/// struct Vector3 {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// impl Encode for Vector3 {
///     fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
///         // Manually encode the Vector3 struct into bytes
///         e.write_f32(self.x);
///         e.write_f32(self.y);
///         e.write_f32(self.z);
///         Ok(())
///     }
/// }
///
/// struct Product {
///     name: String,
///     price: u32,
/// }
///
/// impl Encode for Product {
///     fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
///         // Use field() to encode the fields with stable IDs
///         e.field(0, &self.name)?;
///         e.field(1, &self.price)?;
///         Ok(())
///     }
/// }
#[derive(Default)]
pub struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    /// Creates a new `Encoder`.
    pub fn new() -> Self {
        Encoder::default()
    }

    /// Encodes the given value with the given field ID and appends it to the encoder's byte vector.
    ///
    /// # Parameters
    /// - `id`: The ID of the field to encode.
    /// - `value`: The value to encode, which must implement the `Encode` trait.
    pub fn field<T>(&mut self, id: u32, value: &T) -> Result<(), EncodeError>
    where
        T: Encode,
    {
        // Encode the given value using the trait
        let mut e = Encoder::new();
        value.encode(&mut e)?;

        // Write the id and the encoded data to the byte vector
        self.write_u32(id);
        self.write_sized(e.bytes())?;

        Ok(())
    }

    /// Appends the given byte to the encoder's byte vector.
    ///
    /// # Parameter
    /// - `byte`: A single byte to be added to the encoder's byte vector.
    pub fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// Appends the given bytes to the encoder's byte vector.
    ///
    /// # Parameter
    /// - `bytes`: A slice of bytes to be added to the encoder's byte vector.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// Write the given bytes to the encoder's byte vector, adding the length of the bytes as u64 before the data bytes.
    ///
    /// # Parameter
    /// - `bytes`: A slice of bytes to encode.
    pub fn write_sized(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        let len = bytes
            .len()
            .try_into()
            .map_err(|_| EncodeError::InvalidLength)?;
        self.write_u64(len);
        self.write_bytes(bytes);
        Ok(())
    }

    /// Returns a reference to the encoder's byte vector.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the current length of the encoder's byte vector.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the encoder's byte vector is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Moves the encoder's byte vector out of the encoder, leaving it empty.
    pub fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }
}

/// An error occured during encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    /// Encoding has failed because the length value is invalid or overflowed.
    InvalidLength,
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::InvalidLength => write!(f, "the length value is invalid or overflowed"),
        }
    }
}

impl std::error::Error for EncodeError {}
