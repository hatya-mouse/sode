use crate::utils::{write_u32, write_u64};

/// A trait for types that are encodable using an encoder.
pub trait Encode {
    /// Encodes the payload using the given encoder.
    ///
    /// # Parameters
    /// - `e`: A mutable reference to the encoder to use for encoding.
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError>;
}

/// An encoder.
/// Use `field` to encode a field with a given ID and value, or `write_bytes` to append raw bytes to the encoder's byte vector.
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

        // Write the encoded data to the byte vector
        let Some(len) = e.bytes.len().try_into().ok() else {
            return Err(EncodeError::UnsupportedLength);
        };
        write_u32(&mut self.bytes, id);
        write_u64(&mut self.bytes, len);
        self.bytes.extend_from_slice(&e.bytes);

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
}

/// An error occured during encoding.
pub enum EncodeError {
    /// Encoding has failed because the length of the bytes exceeded the supported length.
    UnsupportedLength,
}
