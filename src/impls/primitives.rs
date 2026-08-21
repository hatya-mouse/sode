use crate::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder, utils::*};

// --- ENCODING ---

impl Encode for u8 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_byte(*self);
        Ok(())
    }
}

impl Encode for u16 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for u32 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for u64 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for u128 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for i8 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for i16 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for i32 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for i64 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for f32 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for f64 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_bytes(&Self::to_le_bytes(*self));
        Ok(())
    }
}

impl Encode for bool {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_byte(if *self { 1 } else { 0 });
        Ok(())
    }
}

// --- DECODING ---

impl Decode for u8 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_u8(d.bytes()).ok_or(DecodeError::LengthExceeded)
    }
}

impl Decode for u16 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_u16(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for u32 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_u32(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for u64 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_u64(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for u128 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_u128(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for i8 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_i8(d.bytes()).ok_or(DecodeError::LengthExceeded)
    }
}

impl Decode for i16 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_i16(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for i32 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_i32(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for i64 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_i64(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for i128 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_i128(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for f32 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_f32(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for f64 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_f64(&mut d.bytes()).map_err(|_| DecodeError::LengthExceeded)
    }
}

impl Decode for bool {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        read_bool(d.bytes()).ok_or(DecodeError::LengthExceeded)
    }
}
