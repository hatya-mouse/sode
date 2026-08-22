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
    version: u64,
    bytes: &'a [u8],
}

impl<'a> ValueDecoder<'a> {
    /// Creates a new `ValueDecoder` from the given bytes.
    ///
    /// # Parameters
    /// - `version`: The version of the data to decode.
    /// - `bytes`: A slice of bytes to decode from.
    pub fn from_bytes(version: u64, bytes: &'a [u8]) -> Result<Self, DecodeError> {
        Ok(ValueDecoder { version, bytes })
    }

    /// Creates a new `FieldDecoder` from this `ValueDecoder`.
    pub fn to_field_decoder(mut self) -> Result<FieldDecoder<'a>, DecodeError> {
        let fields = self.parse_fields()?;
        Ok(FieldDecoder::new(self.version, fields))
    }

    /// Parses the bytes into the fields while consuming the bytes, and returns the field index and its corresponding bytes as a map of byte slices.
    ///
    /// # Parameter
    /// - `bytes`: A slice of bytes to parse into fields.
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
pub struct FieldDecoder<'a> {
    version: u64,
    fields: Vec<Field<'a>>,
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
    /// - `version`: The version of the data to decode.
    /// - `fields`: The parsed fields.
    fn new(version: u64, fields: Vec<Field<'a>>) -> Self {
        FieldDecoder { version, fields }
    }

    /// Returns the version of the data we're currently decoding.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Returns the decoded data for the given field ID.
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
        let mut d = ValueDecoder::from_bytes(self.version, field.data)?;
        T::decode(&mut d).map(Some)
    }
}

/// An error occured during decoding.
pub enum DecodeError {
    /// Decoding has failed because the length value is invalid or overflown.
    InvalidLength,
    /// Decoding has failed because the field ID is duplicated.
    DuplicateField,
    /// Decoding has failed because the decoder reached the end of the file unexpectedly.
    UnexpectedEof,
}
