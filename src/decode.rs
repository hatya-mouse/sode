use std::{collections::HashMap, io::Read};

/// A trait for types that are decodable using a decoder.
pub trait Decode {
    /// Decodes the type from the given decoder.
    ///
    /// # Parameters
    /// - `d`: A mutable reference to the decoder to use for decoding.
    fn decode(d: &mut Decoder) -> Option<Self>
    where
        Self: Sized;
}

/// A struct used to decode data from a binary data.
pub struct Decoder<'a> {
    version: u64,
    fields: HashMap<u64, &'a [u8]>,
}

impl<'a> Decoder<'a> {
    /// Creates a new `Decoder` from the given bytes, storing the byte index to each fields.
    ///
    /// # Parameters
    /// - `bytes`: A slice of bytes to decode from.
    pub fn from_bytes(version: u64, bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let fields = Self::split_fields(bytes)?;
        Ok(Decoder { version, fields })
    }

    /// Splits the given bytes into the fields, and returns the field index and its corresponding bytes as a map of byte slices.
    fn split_fields(mut bytes: &[u8]) -> Result<HashMap<u64, &[u8]>, DecodeError> {
        let mut fields = HashMap::new();
        let mut index_bytes = [0u8; 8];
        let mut len_bytes = [0u8; 8];

        // Read the index of the field
        while let Ok(()) = bytes.read_exact(&mut index_bytes) {
            let index = u64::from_le_bytes(index_bytes);

            // Read the length of the field data
            let Ok(()) = bytes.read_exact(&mut len_bytes) else {
                return Err(DecodeError::LengthExceeded);
            };
            let len = u64::from_le_bytes(len_bytes);

            // Then read the field data and insert it to the fields map
            if bytes.len() < len as usize {
                return Err(DecodeError::LengthExceeded);
            }
            let field_data = &bytes[..len as usize];
            fields.insert(index, field_data);
        }

        Ok(fields)
    }
}

/// An error occured during decoding.
pub enum DecodeError {
    /// Decode has failed because `decode` function returned `None`.
    DecodeFailed,
    /// Decode has failed because the length of the bytes exceeded the available byte length.
    LengthExceeded,
}
