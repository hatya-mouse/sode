use crate::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- ENCODING ---

impl Encode for u8 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u8(*self);
        Ok(())
    }
}

impl Encode for u16 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u16(*self);
        Ok(())
    }
}

impl Encode for u32 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u32(*self);
        Ok(())
    }
}

impl Encode for u64 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u64(*self);
        Ok(())
    }
}

impl Encode for u128 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u128(*self);
        Ok(())
    }
}

impl Encode for i8 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_i8(*self);
        Ok(())
    }
}

impl Encode for i16 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_i16(*self);
        Ok(())
    }
}

impl Encode for i32 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_i32(*self);
        Ok(())
    }
}

impl Encode for i64 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_i64(*self);
        Ok(())
    }
}

impl Encode for f32 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_f32(*self);
        Ok(())
    }
}

impl Encode for f64 {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_f64(*self);
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
        d.read_u8()
    }
}

impl Decode for u16 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_u16()
    }
}

impl Decode for u32 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_u32()
    }
}

impl Decode for u64 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_u64()
    }
}

impl Decode for u128 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_u128()
    }
}

impl Decode for i8 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_i8()
    }
}

impl Decode for i16 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_i16()
    }
}

impl Decode for i32 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_i32()
    }
}

impl Decode for i64 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_i64()
    }
}

impl Decode for i128 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_i128()
    }
}

impl Decode for f32 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_f32()
    }
}

impl Decode for f64 {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_f64()
    }
}

impl Decode for bool {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        d.read_bool()
    }
}
