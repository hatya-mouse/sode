# sode

![Crates.io License](https://img.shields.io/crates/l/sode?style=for-the-badge)
![Crates.io Version](https://img.shields.io/crates/v/sode?style=for-the-badge)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/sode?style=for-the-badge)

`sode` is a simple and small binary decoding / encoding crate.

## Example

```rust
use sode::{Encode, Encoder, EncodeError, Decode, ValueDecoder, DecodeError};

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    id: u64,
}

impl Encode for User {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.name)?;
        e.field(1, &self.age)?;
        e.field(2, &self.id)?;
        Ok(())
    }
}

impl Decode for User {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;

        Ok(User {
            name: d.field(0)?.unwrap_or_default(),
            age: d.field(1)?.unwrap_or_default(),
            id: d.field(2)?.unwrap_or_default(),
        })
    }
}

let user = User {
    name: "Alice".to_string(),
    age: 256,
    id: 30,
};

// Encode the user struct into bytes
let bytes = sode::encode(&user).unwrap();

// Decode the bytes back into a user struct
let decoded_user = sode::decode::<User>(&bytes, 1).unwrap();

assert_eq!(user, decoded_user);
```
