use crate::{Encode, EncodeError, Encoder};

impl<T: Encode> Encode for &[T] {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        // First write the count of element to the vector
        let count: u64 = self
            .len()
            .try_into()
            .map_err(|_| EncodeError::InvalidLength)?;
        e.write_u64(count);

        // Write each element in the vector
        for element in self.iter() {
            // Encode the element to bytes
            let mut element_encoder = Encoder::new();
            element.encode(&mut element_encoder)?;
            e.write_sized(element_encoder.bytes())?;
        }

        Ok(())
    }
}
