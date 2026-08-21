use std::io::{self, Read};

// --- WRITE ---

/// Writes the given `u32` value to the given byte vector.
pub(crate) fn write_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

/// Writes the given `u64` value to the given byte vector.
pub(crate) fn write_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

// --- READ ---

/// Decodes an `u8` value from the given raw bytes.
pub(crate) fn read_u8(bytes: &[u8]) -> Option<u8> {
    bytes.first().copied()
}

/// Decodes an `u16` value from the given raw bytes.
pub(crate) fn read_u16(bytes: &mut &[u8]) -> io::Result<u16> {
    let mut value_bytes = [0u8; 2];
    bytes.read_exact(&mut value_bytes)?;
    Ok(u16::from_le_bytes(value_bytes))
}

/// Decodes an `u32` value from the given raw bytes.
pub(crate) fn read_u32(bytes: &mut &[u8]) -> io::Result<u32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes)?;
    Ok(u32::from_le_bytes(value_bytes))
}

/// Decodes an `u64` value from the given raw bytes.
pub(crate) fn read_u64(bytes: &mut &[u8]) -> io::Result<u64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes)?;
    Ok(u64::from_le_bytes(value_bytes))
}

/// Decodes an `u128` value from the given raw bytes.
pub(crate) fn read_u128(bytes: &mut &[u8]) -> io::Result<u128> {
    let mut value_bytes = [0u8; 16];
    bytes.read_exact(&mut value_bytes)?;
    Ok(u128::from_le_bytes(value_bytes))
}

/// Decodes an `i8` value from the given raw bytes.
pub(crate) fn read_i8(bytes: &[u8]) -> Option<i8> {
    bytes.first().map(|byte| i8::from_le_bytes([*byte]))
}

/// Decodes an `i16` value from the given raw bytes.
pub(crate) fn read_i16(bytes: &mut &[u8]) -> io::Result<i16> {
    let mut value_bytes = [0u8; 2];
    bytes.read_exact(&mut value_bytes)?;
    Ok(i16::from_le_bytes(value_bytes))
}

/// Decodes an `i32` value from the given raw bytes.
pub(crate) fn read_i32(bytes: &mut &[u8]) -> io::Result<i32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes)?;
    Ok(i32::from_le_bytes(value_bytes))
}

/// Decodes an `i64` value from the given raw bytes.
pub(crate) fn read_i64(bytes: &mut &[u8]) -> io::Result<i64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes)?;
    Ok(i64::from_le_bytes(value_bytes))
}

/// Decodes an `i128` value from the given raw bytes.
pub(crate) fn read_i128(bytes: &mut &[u8]) -> io::Result<i128> {
    let mut value_bytes = [0u8; 16];
    bytes.read_exact(&mut value_bytes)?;
    Ok(i128::from_le_bytes(value_bytes))
}

/// Decodes an `f32` value from the given raw bytes.
pub(crate) fn read_f32(bytes: &mut &[u8]) -> io::Result<f32> {
    let mut value_bytes = [0u8; 4];
    bytes.read_exact(&mut value_bytes)?;
    Ok(f32::from_le_bytes(value_bytes))
}

/// Decodes an `f64` value from the given raw bytes.
pub(crate) fn read_f64(bytes: &mut &[u8]) -> io::Result<f64> {
    let mut value_bytes = [0u8; 8];
    bytes.read_exact(&mut value_bytes)?;
    Ok(f64::from_le_bytes(value_bytes))
}

/// Decodes an `bool` value from the given raw bytes.
pub(crate) fn read_bool(bytes: &[u8]) -> Option<bool> {
    bytes.first().map(|byte| *byte != 0)
}
