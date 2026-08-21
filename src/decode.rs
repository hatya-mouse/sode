use crate::primitives::{decode_u32, decode_u64};

/// A trait for types that are decodable using a decoder.
pub trait Decode: Sized {
    /// Decodes the type from the given decoder.
    ///
    /// # Parameters
    /// - `d`: A mutable reference to the decoder to use for decoding.
    fn decode(d: &mut Decoder) -> Result<Self, DecodeError>;
}

/// A struct used to decode data from a binary data.
pub struct Decoder<'a> {
    version: u64,
    fields: Vec<Field<'a>>,
}

struct Field<'a> {
    id: u32,
    data: &'a [u8],
}

impl<'a> Decoder<'a> {
    /// Creates a new `Decoder` from the given bytes, storing the byte index to each fields.
    ///
    /// # Parameters
    /// - `bytes`: A slice of bytes to decode from.
    pub fn from_bytes(version: u64, bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let fields = Self::parse_fields(bytes)?;

        Ok(Decoder { version, fields })
    }

    /// Parses the given bytes into the fields, and returns the field index and its corresponding bytes as a map of byte slices.
    fn parse_fields(mut bytes: &'a [u8]) -> Result<Vec<Field>, DecodeError> {
        let mut fields = Vec::new();

        while !bytes.is_empty() {
            // Read the id of the field
            let Ok(id) = decode_u32(&mut bytes) else {
                return Err(DecodeError::LengthExceeded);
            };

            // Check for duplicate field IDs
            if fields.iter().any(|field: &Field<'a>| field.id == id) {
                return Err(DecodeError::DuplicateField);
            }

            // Read the length of the field data
            let Ok(len) = decode_u64(&mut bytes) else {
                return Err(DecodeError::LengthExceeded);
            };

            // Convert the length to usize safely
            let len_usize = len.try_into().map_err(|_| DecodeError::LengthExceeded)?;

            // Then read the field data and insert it to the fields map
            if bytes.len() < len_usize {
                return Err(DecodeError::LengthExceeded);
            }
            let (data, rest) = bytes.split_at(len_usize);
            bytes = rest;
            fields.push(Field { id, data });
        }

        Ok(fields)
    }

    /// Returns the decoded data for the given field ID.
    pub fn field<T>(&self, id: u32) -> Result<Option<T>, DecodeError>
    where
        T: Decode,
    {
        let Some(field) = self.fields.iter().find(|field| field.id == id) else {
            return Ok(None);
        };
        let mut d = Decoder::from_bytes(self.version, field.data)?;
        T::decode(&mut d).map(Some)
    }
}

/// An error occured during decoding.
pub enum DecodeError {
    /// Decode has failed because the field ID is duplicated.
    DuplicateField,
    /// Decode has failed because `decode` function returned `None`.
    DecodeFailed,
    /// Decode has failed because the length of the bytes exceeded the available byte length.
    LengthExceeded,
}
