use crate::{Decode, DecodeError, ValueDecoder, primitives::*};

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
