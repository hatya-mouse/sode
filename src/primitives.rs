use std::io::{self, Read};

// --- WRITE ---

/// Writes the given `u8` value to the given byte vector.
pub(crate) fn write_u8(bytes: &mut Vec<u8>, value: u8) {
    bytes.push(value);
}

/// Writes the given `u16` value to the given byte vector.
pub(crate) fn write_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `u32` value to the given byte vector.
pub(crate) fn write_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `u64` value to the given byte vector.
pub(crate) fn write_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `u128` value to the given byte vector.
pub(crate) fn write_u128(bytes: &mut Vec<u8>, value: u128) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `u8` value to the given byte vector.
pub(crate) fn write_i8(bytes: &mut Vec<u8>, value: i8) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `i16` value to the given byte vector.
pub(crate) fn write_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `i32` value to the given byte vector.
pub(crate) fn write_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `u64` value to the given byte vector.
pub(crate) fn write_i64(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `u128` value to the given byte vector.
pub(crate) fn write_i128(bytes: &mut Vec<u8>, value: i128) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `f32` value to the given byte vector.
pub(crate) fn write_f32(bytes: &mut Vec<u8>, value: f32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `f64` value to the given byte vector.
pub(crate) fn write_f64(bytes: &mut Vec<u8>, value: f64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `bool` value to the given byte vector.
pub(crate) fn write_bool(bytes: &mut Vec<u8>, value: bool) {
    bytes.push(if value { 1 } else { 0 });
}

// --- DECODE ---

/// Decodes an `u8` value from the given raw bytes.
pub(crate) fn read_u8(bytes: &[u8]) -> Option<u8> {
    bytes.get(0).copied()
}

/// Decodes an `u16` value from the given raw bytes.
pub(crate) fn read_u16(bytes: &mut &[u8]) -> io::Result<u16> {
    let mut value_bytes = [0u8; 2];
    bytes.read_exact(&mut value_bytes);
    Ok(u16::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `u32` value from the given raw bytes.
pub(crate) fn read_u32(bytes: &mut &[u8]) -> io::Result<u32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes);
    Ok(u32::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `u64` value from the given raw bytes.
pub(crate) fn read_u64(bytes: &mut &[u8]) -> io::Result<u64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes);
    Ok(u64::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `u128` value from the given raw bytes.
pub(crate) fn read_u128(bytes: &mut &[u8]) -> io::Result<u128> {
    let mut value_bytes = [0u8; 16];
    bytes.read_exact(&mut value_bytes);
    Ok(u128::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i8` value from the given raw bytes.
pub(crate) fn read_i8(bytes: &[u8]) -> Option<i8> {
    bytes.get(0).map(|byte| i8::from_le_bytes([*byte]))
}

/// Decodes an `i16` value from the given raw bytes.
pub(crate) fn read_i16(bytes: &mut &[u8]) -> io::Result<i16> {
    let mut value_bytes = [0u8; 2];
    bytes.read_exact(&mut value_bytes);
    Ok(i16::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i32` value from the given raw bytes.
pub(crate) fn read_i32(bytes: &mut &[u8]) -> io::Result<i32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes);
    Ok(i32::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i64` value from the given raw bytes.
pub(crate) fn read_i64(bytes: &mut &[u8]) -> io::Result<i64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes);
    Ok(i64::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `i128` value from the given raw bytes.
pub(crate) fn read_i128(bytes: &mut &[u8]) -> io::Result<i128> {
    let mut value_bytes = [0u8; 16];
    bytes.read_exact(&mut value_bytes);
    Ok(i128::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `f32` value from the given raw bytes.
pub(crate) fn read_f32(bytes: &mut &[u8]) -> io::Result<f32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes);
    Ok(f32::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `f64` value from the given raw bytes.
pub(crate) fn read_f64(bytes: &mut &[u8]) -> io::Result<f64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes);
    Ok(f64::from_le_bytes(value_bytes.try_into().unwrap()))
}

/// Decodes an `bool` value from the given raw bytes.
pub(crate) fn read_bool(bytes: &[u8]) -> Option<bool> {
    bytes.get(0).map(|byte| *byte != 0)
}
