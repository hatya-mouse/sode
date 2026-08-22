use std::io::Read;

/// A trait for types that are decodable using a decoder.
pub trait Decode: Sized {
    /// Decodes the bytes using the given decoder.
    ///
    /// # Parameters
    /// - `d`: A mutable reference to the decoder to use for decoding.
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError>;
}

/// A decoder for a raw bytes representin one value.
pub struct ValueDecoder<'a> {
    bytes: &'a [u8],
    version: u64,
}

impl<'a> ValueDecoder<'a> {
    /// Creates a new `ValueDecoder` from the given bytes.
    ///
    /// # Parameters
    /// - `bytes`: A slice of bytes to decode from.
    /// - `version`: The version of the data to decode.
    pub fn from_bytes(bytes: &'a [u8], version: u64) -> Result<Self, DecodeError> {
        Ok(ValueDecoder { version, bytes })
    }

    /// Creates a new `FieldDecoder` from this `ValueDecoder`.
    pub fn to_field_decoder(&mut self) -> Result<FieldDecoder<'a>, DecodeError> {
        let fields = self.parse_fields()?;
        Ok(FieldDecoder::new(fields, self.version))
    }

    /// Parses the bytes into the fields while consuming the bytes, and returns the field index and its corresponding bytes as a vector of fields.
    fn parse_fields(&mut self) -> Result<Vec<Field<'a>>, DecodeError> {
        let mut fields = Vec::new();

        while !self.is_empty() {
            // Read the id of the field
            let id = self.read_u32()?;

            // Check for duplicate field IDs
            if fields.iter().any(|field: &Field<'a>| field.id == id) {
                return Err(DecodeError::DuplicateField);
            }

            // Read the data of the field
            let data = self.read_sized()?;
            fields.push(Field { id, data });
        }

        Ok(fields)
    }

    /// Returns the version of the data we're currently decoding.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Reads the exact number of bytes required to fill buf.
    ///
    /// See [Read::read_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact) for more details.
    ///
    /// # Parameter
    /// - `buf`: The slice to fill the obtained bytes into.
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), DecodeError> {
        self.bytes
            .read_exact(buf)
            .map_err(|_| DecodeError::UnexpectedEof)
    }

    /// Reads the data bytes prefixed by its length in u64, returning the data bytes.
    pub fn read_sized(&mut self) -> Result<&'a [u8], DecodeError> {
        // Read the length of the data bytes
        let len = self.read_u64().map_err(|_| DecodeError::UnexpectedEof)?;
        let len_usize = len.try_into().map_err(|_| DecodeError::InvalidLength)?;
        // Perform data length checking
        if self.len() < len_usize {
            return Err(DecodeError::UnexpectedEof);
        }

        // Then read the data bytes
        let (data, rest) = self.bytes.split_at(len_usize);
        self.bytes = rest;
        Ok(data)
    }

    /// Reads the data until its end, returning the data bytes.
    pub fn read_to_end(&mut self) -> &'a [u8] {
        let bytes = self.bytes;
        self.bytes = &[];
        bytes
    }

    /// Returns the remaining bytes in the decoder.
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the length of the remaining bytes in the decoder.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns true if there are no remaining bytes in the decoder.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// A decoder for binary data composed of multiple fields with unique IDs.
///
/// This can only be created by calling `to_field_decoder()` on the given `ValueDecoder`.
pub struct FieldDecoder<'a> {
    fields: Vec<Field<'a>>,
    version: u64,
}

/// A struct representing a field in the field decoder.
struct Field<'a> {
    id: u32,
    data: &'a [u8],
}

impl<'a> FieldDecoder<'a> {
    /// Creates a new `Decoder` from the given version and the fields.
    ///
    /// # Parameters
    /// - `fields`: The parsed fields.
    /// - `version`: The version of the data to decode.
    fn new(fields: Vec<Field<'a>>, version: u64) -> Self {
        FieldDecoder { version, fields }
    }

    /// Returns the version of the data we're currently decoding.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Returns the decoded data for the given field ID.
    /// If the field does not exist, `Ok(None)` is returned.
    /// If the field exists but decoding fails, `Err(DecodeError)` is returned.
    ///
    /// # Parameter
    /// - `id`: The ID of the field to decode.
    pub fn field<T>(&self, id: u32) -> Result<Option<T>, DecodeError>
    where
        T: Decode,
    {
        let Some(field) = self.fields.iter().find(|field| field.id == id) else {
            return Ok(None);
        };
        let mut d = ValueDecoder::from_bytes(field.data, self.version)?;
        T::decode(&mut d).map(Some)
    }

    /// Returns all the field IDs in the decoder.
    pub fn all_field_ids(&self) -> Vec<u32> {
        self.fields.iter().map(|field| field.id).collect()
    }
}

/// An error occured during decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Decoding has failed because the length value is invalid or overflowed.
    InvalidLength,
    /// Decoding has failed because the field ID is duplicated.
    DuplicateField,
    /// Decoding has failed because the decoder reached the end of the file unexpectedly.
    UnexpectedEof,
    /// Decoding has falied because the data is invalid.
    /// Use this for cases where the decoder can read the bytes, but the value is semantically invalid.
    InvalidData,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::InvalidLength => write!(f, "the length value is invalid or overflowed"),
            DecodeError::DuplicateField => write!(f, "the field ID is duplicated"),
            DecodeError::UnexpectedEof => write!(f, "the decoder has unexpectedly reached the end"),
            DecodeError::InvalidData => write!(f, "the data is invalid"),
        }
    }
}

impl std::error::Error for DecodeError {}
