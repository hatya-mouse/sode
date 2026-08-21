use crate::{Decode, DecodeError, ValueDecoder};

impl Decode for u8 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let mut buf = [0u8; 1];
        d.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}
