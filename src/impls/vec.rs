use crate::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

impl<T: Decode> Decode for Vec<T> {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        // Vector is stored in the following format:
        // [ count ][ data ]
        //             |
        //             v
        // [ len of element 0 ][ data ][ len of element 1 ][ data ]...

        // First get the count of elements in the vector
        let count = d.read_u64()?;
        let count_usize = count.try_into().map_err(|_| DecodeError::InvalidLength)?;
        let mut vec = Vec::new();

        // Then read the elements one by one
        for _ in 0..count_usize {
            let element_bytes = d.read_sized().map_err(|_| DecodeError::InvalidLength)?;
            let mut element_decoder = ValueDecoder::from_bytes(d.version(), element_bytes)?;
            let element = T::decode(&mut element_decoder)?;
            vec.push(element);
        }

        // Return an error if the length of the binary data is longer than the given count
        if !d.is_empty() {
            return Err(DecodeError::InvalidLength);
        }

        Ok(vec)
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        // First write the count of element to the vector
        let count: u64 = self
            .len()
            .try_into()
            .map_err(|_| EncodeError::InvalidLength)?;
        e.write_u64(count);

        // Write each element in the vector
        for element in self {
            // Encode the element to bytes
            let mut element_encoder = Encoder::new();
            element.encode(&mut element_encoder)?;
            e.write_sized(element_encoder.bytes())?;
        }

        Ok(())
    }
}
