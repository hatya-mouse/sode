use crate::primitives::decode_u64;

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
    fields: Vec<Field<'a>>,
}

struct Field<'a> {
    id: u64,
    data: &'a [u8],
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
    fn split_fields(mut bytes: &[u8]) -> Result<Vec<Field>, DecodeError> {
        let mut fields = Vec::new();

        while !bytes.is_empty() {
            // Read the id and length of the field
            let Ok(id) = decode_u64(&mut bytes) else {
                return Err(DecodeError::LengthExceeded);
            };
            let Ok(len) = decode_u64(&mut bytes) else {
                return Err(DecodeError::LengthExceeded);
            };

            // Then read the field data and insert it to the fields map
            if bytes.len() < len as usize {
                return Err(DecodeError::LengthExceeded);
            }
            let (data, rest) = &bytes.split_at(len as usize);
            bytes = rest;
            fields.push(Field { id, data });
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
