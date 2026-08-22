use crate::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};
use std::{collections::HashMap, hash::Hash};

impl<K, V> Encode for HashMap<K, V>
where
    K: Encode,
    V: Encode,
{
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        // First write the count of the values to the vector
        let count: u64 = self
            .len()
            .try_into()
            .map_err(|_| EncodeError::InvalidLength)?;
        e.write_u64(count);

        for (key, value) in self {
            // Write the key
            let mut key_encoder = Encoder::new();
            key.encode(&mut key_encoder)?;
            e.write_sized(key_encoder.bytes())?;

            // Then write the value
            let mut value_encoder = Encoder::new();
            value.encode(&mut value_encoder)?;
            e.write_sized(value_encoder.bytes())?;
        }

        Ok(())
    }
}

impl<K, V> Decode for HashMap<K, V>
where
    K: Decode + Eq + Hash,
    V: Decode,
{
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        // HashMap is stored in the following format:
        // [ count ][ data ]
        //             |
        //             v
        // [ len of key 0 ][ key ][ len of value 0 ][ value ][ len of key 1 ][ key ][ len of value 1 ][ value ]...

        // First get the count of elements in the vector
        let count = d.read_u64()?;
        let count_usize = count.try_into().map_err(|_| DecodeError::InvalidLength)?;

        // If the remaining length of the binary data is less then the minimum length possible, throw an error
        if count_usize > d.len() / 16 {
            return Err(DecodeError::InvalidLength);
        }

        let mut hash_map = HashMap::new();

        // Then read the elements one by one
        for _ in 0..count_usize {
            // Read the key
            let key_bytes = d.read_sized().map_err(|_| DecodeError::InvalidLength)?;
            let mut key_decoder = ValueDecoder::from_bytes(d.version(), key_bytes)?;
            let key = K::decode(&mut key_decoder)?;

            // Then read the value
            let value_bytes = d.read_sized().map_err(|_| DecodeError::InvalidLength)?;
            let mut value_decoder = ValueDecoder::from_bytes(d.version(), value_bytes)?;
            let value = V::decode(&mut value_decoder)?;

            // If the key already exists in the hash map, return an error
            if hash_map.insert(key, value).is_some() {
                return Err(DecodeError::InvalidData);
            }
        }

        // Return an error if the length of the binary data is longer than the given count
        if !d.is_empty() {
            return Err(DecodeError::InvalidLength);
        }

        Ok(hash_map)
    }
}
