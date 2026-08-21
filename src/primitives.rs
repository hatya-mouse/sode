use std::io::{self, Read};

/// Decodes an `u8` value from the given raw bytes.
pub(crate) fn decode_u8(bytes: &[u8]) -> Option<u8> {
    bytes.get(0).copied()
}

/// Decodes an `u16` value from the given raw bytes.
pub(crate) fn decode_u16(bytes: &mut &[u8]) -> io::Result<u16> {
    let mut value_bytes = [0u8; 2];
    bytes.read_exact(&mut value_bytes);
    Ok(u16::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `u32` value from the given raw bytes.
pub(crate) fn decode_u32(bytes: &mut &[u8]) -> io::Result<u32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes);
    Ok(u32::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `u64` value from the given raw bytes.
pub(crate) fn decode_u64(bytes: &mut &[u8]) -> io::Result<u64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes);
    Ok(u64::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `u128` value from the given raw bytes.
pub(crate) fn decode_u128(bytes: &mut &[u8]) -> io::Result<u128> {
    let mut value_bytes = [0u8; 16];
    bytes.read_exact(&mut value_bytes);
    Ok(u128::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i8` value from the given raw bytes.
pub(crate) fn decode_i8(bytes: &[u8]) -> Option<i8> {
    bytes.get(0).map(|byte| i8::from_le_bytes([*byte]))
}

/// Decodes an `i16` value from the given raw bytes.
pub(crate) fn decode_i16(bytes: &mut &[u8]) -> io::Result<i16> {
    let mut value_bytes = [0u8; 2];
    bytes.read_exact(&mut value_bytes);
    Ok(i16::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i32` value from the given raw bytes.
pub(crate) fn decode_i32(bytes: &mut &[u8]) -> io::Result<i32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes);
    Ok(i32::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i64` value from the given raw bytes.
pub(crate) fn decode_i64(bytes: &mut &[u8]) -> io::Result<i64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes);
    Ok(i64::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i128` value from the given raw bytes.
pub(crate) fn decode_i128(bytes: &mut &[u8]) -> io::Result<i128> {
    let mut value_bytes = [0u8; 16];
    bytes.read_exact(&mut value_bytes);
    Ok(i128::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `f32` value from the given raw bytes.
pub(crate) fn decode_f32(bytes: &mut &[u8]) -> io::Result<f32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes);
    Ok(f32::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `f64` value from the given raw bytes.
pub(crate) fn decode_f64(bytes: &mut &[u8]) -> io::Result<f64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes);
    Ok(f64::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `bool` value from the given raw bytes.
pub(crate) fn decode_bool(bytes: &[u8]) -> Option<bool> {
    bytes.get(0).map(|byte| *byte != 0)
}
