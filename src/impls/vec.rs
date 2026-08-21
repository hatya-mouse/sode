use crate::{Decode, DecodeError, ValueDecoder, utils::read_u64};

impl<T> Decode for Vec<T>
where
    T: Decode + Sized,
{
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        // Vector is stored in the following format:
        // [ count ][ data ]
        //             |
        //             v
        // [ byte length for element 0 ][ data ][ byte length for element 1 ][ data ]...
        let mut bytes = d.bytes();

        // First get the count of elements in the vector
        let count = read_u64(&mut bytes).map_err(|_| DecodeError::LengthExceeded)?;
        let count_usize = count
            .try_into()
            .map_err(|_| DecodeError::UnsupportedLength)?;
        let mut vec = Vec::with_capacity(count_usize);

        // Then read the elements one by one
        while !bytes.is_empty() {
            // Get the byte length of the next element
            let len = read_u64(&mut bytes).map_err(|_| DecodeError::LengthExceeded)?;
            let len_usize = len.try_into().map_err(|_| DecodeError::UnsupportedLength)?;

            // Read the element data and decode it
            let element_bytes = bytes.get(..len_usize).ok_or(DecodeError::LengthExceeded)?;
            let mut element_decoder = ValueDecoder::from_bytes(d.version(), element_bytes)?;
            let element = T::decode(&mut element_decoder)?;
            vec.push(element);
        }

        Ok(vec)
    }
}
