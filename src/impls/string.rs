use crate::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

impl Encode for String {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        // Write the string to the encoder
        e.write_bytes(self.as_bytes());
        Ok(())
    }
}

impl Encode for &str {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        // Write the string to the encoder
        e.write_bytes(self.as_bytes());
        Ok(())
    }
}

impl Decode for String {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        // Read the string from the decoder
        let bytes = d.read_to_end();
        String::from_utf8(bytes.to_vec()).map_err(|_| DecodeError::InvalidData)
    }
}
