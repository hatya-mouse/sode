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
            return Err(EncodeError::InvalidLength);
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

    /// Write the given bytes to the encoder's byte vector, adding the length of the bytes as u64 before the data bytes.
    pub fn write_sized(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        let len = bytes
            .len()
            .try_into()
            .map_err(|_| EncodeError::InvalidLength)?;
        write_u64(&mut self.bytes, len);
        self.bytes.extend_from_slice(bytes);
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
}

/// An error occured during encoding.
pub enum EncodeError {
    /// Encoding has failed because the length value is invalid or overflown.
    InvalidLength,
}
