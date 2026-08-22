use crate::{Decode, DecodeError, ValueDecoder};
use std::collections::HashMap;

impl<K, V> Decode for HashMap<K, V>
where
    K: Decode,
    V: Decode,
{
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        // HashMap is stored in the following format:
        // [ count ][ data ]
        //             |
        //             v
        // [ len of key 0 ][ key ][ len of value 0 ][ value ][ len of key 1 ][ key ][ len of value 1 ][ value ]...

        // First get the count of elements in the vector
    }
}
