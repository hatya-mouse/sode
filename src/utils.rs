use crate::{DecodeError, Encoder, ValueDecoder};

// --- WRITE ---

impl Encoder {
    /// Writes the given `u8` value.
    pub fn write_u8(&mut self, value: u8) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `u16` value.
    pub fn write_u16(&mut self, value: u16) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `u32` value.
    pub fn write_u32(&mut self, value: u32) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `u64` value.
    pub fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `u128` value.
    pub fn write_u128(&mut self, value: u128) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `i8` value.
    pub fn write_i8(&mut self, value: i8) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `i16` value.
    pub fn write_i16(&mut self, value: i16) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `i32` value.
    pub fn write_i32(&mut self, value: i32) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `i64` value.
    pub fn write_i64(&mut self, value: i64) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `i128` value.
    pub fn write_i128(&mut self, value: i128) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `f32` value.
    pub fn write_f32(&mut self, value: f32) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `f64` value.
    pub fn write_f64(&mut self, value: f64) {
        self.write_bytes(&value.to_le_bytes());
    }

    /// Writes the given `bool` value.
    pub fn write_bool(&mut self, value: bool) {
        self.write_byte(if value { 1 } else { 0 });
    }
}

// --- READ ---

impl ValueDecoder<'_> {
    /// Decodes an `u8` value, consuming the bytes in the decoder.
    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        let mut value_byte = [0u8; 1];
        self.read_exact(&mut value_byte)?;
        Ok(u8::from_le_bytes(value_byte))
    }

    /// Decodes an `u16` value, consuming the bytes in the decoder.
    pub fn read_u16(&mut self) -> Result<u16, DecodeError> {
        let mut value_bytes = [0u8; 2];
        self.read_exact(&mut value_bytes)?;
        Ok(u16::from_le_bytes(value_bytes))
    }

    /// Decodes an `u32` value, consuming the bytes in the decoder.
    pub fn read_u32(&mut self) -> Result<u32, DecodeError> {
        let mut value_bytes = [0u8; 4];
        self.read_exact(&mut value_bytes)?;
        Ok(u32::from_le_bytes(value_bytes))
    }

    /// Decodes an `u64` value, consuming the bytes in the decoder.
    pub fn read_u64(&mut self) -> Result<u64, DecodeError> {
        let mut value_bytes = [0u8; 8];
        self.read_exact(&mut value_bytes)?;
        Ok(u64::from_le_bytes(value_bytes))
    }

    /// Decodes an `u128` value, consuming the bytes in the decoder.
    pub fn read_u128(&mut self) -> Result<u128, DecodeError> {
        let mut value_bytes = [0u8; 16];
        self.read_exact(&mut value_bytes)?;
        Ok(u128::from_le_bytes(value_bytes))
    }

    /// Decodes an `i8` value, consuming the bytes in the decoder.
    pub fn read_i8(&mut self) -> Result<i8, DecodeError> {
        let mut value_byte = [0u8; 1];
        self.read_exact(&mut value_byte)?;
        Ok(i8::from_le_bytes(value_byte))
    }

    /// Decodes an `i16` value, consuming the bytes in the decoder.
    pub fn read_i16(&mut self) -> Result<i16, DecodeError> {
        let mut value_bytes = [0u8; 2];
        self.read_exact(&mut value_bytes)?;
        Ok(i16::from_le_bytes(value_bytes))
    }

    /// Decodes an `i32` value, consuming the bytes in the decoder.
    pub fn read_i32(&mut self) -> Result<i32, DecodeError> {
        let mut value_bytes = [0u8; 4];
        self.read_exact(&mut value_bytes)?;
        Ok(i32::from_le_bytes(value_bytes))
    }

    /// Decodes an `i64` value, consuming the bytes in the decoder.
    pub fn read_i64(&mut self) -> Result<i64, DecodeError> {
        let mut value_bytes = [0u8; 8];
        self.read_exact(&mut value_bytes)?;
        Ok(i64::from_le_bytes(value_bytes))
    }

    /// Decodes an `i128` value, consuming the bytes in the decoder.
    pub fn read_i128(&mut self) -> Result<i128, DecodeError> {
        let mut value_bytes = [0u8; 16];
        self.read_exact(&mut value_bytes)?;
        Ok(i128::from_le_bytes(value_bytes))
    }

    /// Decodes an `f32` value, consuming the bytes in the decoder.
    pub fn read_f32(&mut self) -> Result<f32, DecodeError> {
        let mut value_bytes = [0u8; 4];
        self.read_exact(&mut value_bytes)?;
        Ok(f32::from_le_bytes(value_bytes))
    }

    /// Decodes an `f64` value, consuming the bytes in the decoder.
    pub fn read_f64(&mut self) -> Result<f64, DecodeError> {
        let mut value_bytes = [0u8; 8];
        self.read_exact(&mut value_bytes)?;
        Ok(f64::from_le_bytes(value_bytes))
    }

    /// Decodes an `bool` value, consuming the bytes in the decoder.
    pub fn read_bool(&mut self) -> Result<bool, DecodeError> {
        let mut value_byte = [0u8; 1];
        self.read_exact(&mut value_byte)?;
        match value_byte[0] {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
}
