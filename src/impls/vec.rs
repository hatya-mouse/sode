use crate::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder, utils::read_u64};

impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        // Vector is stored in the following format:
        // [ count ][ data ]
        //             |
        //             v
        // [ byte length for element 0 ][ data ][ byte length for element 1 ][ data ]...
        let mut bytes = d.bytes();

        // First get the count of elements in the vector
        let count = read_u64(&mut bytes).map_err(|_| DecodeError::UnexpectedEof)?;
        let count_usize = count.try_into().map_err(|_| DecodeError::InvalidLength)?;
        let mut vec = Vec::with_capacity(count_usize);

        // Then read the elements one by one
        while !bytes.is_empty() {
            // Get the byte length of the next element
            let len = read_u64(&mut bytes).map_err(|_| DecodeError::UnexpectedEof)?;
            let len_usize = len.try_into().map_err(|_| DecodeError::InvalidLength)?;

            // Read the element data and decode it
            let element_bytes = bytes.get(..len_usize).ok_or(DecodeError::UnexpectedEof)?;
            let mut element_decoder = ValueDecoder::from_bytes(d.version(), element_bytes)?;
            let element = T::decode(&mut element_decoder)?;
            vec.push(element);
        }

        Ok(vec)
    }
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        // First write the count of element in the vector
        let count: u64 = self
            .len()
            .try_into()
            .map_err(|_| EncodeError::InvalidLength)?;
        e.write_bytes(&count.to_le_bytes());

        // Write each element in the vector
        for element in self {
            // Encode the element to bytes
            let mut element_encoder = Encoder::new();
            element.encode(&mut element_encoder)?;

            // Write the length of the element into the encoder
            let element_len: u64 = element_encoder
                .len()
                .try_into()
                .map_err(|_| EncodeError::InvalidLength)?;
            e.write_bytes(&element_len.to_le_bytes());

            // Then write the element bytes into the encoder
            e.write_bytes(element_encoder.bytes());
        }

        Ok(())
    }
}
