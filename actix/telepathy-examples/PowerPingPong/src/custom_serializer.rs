use serde::{Deserialize, Serialize};
use bincode;
use actix_telepathy::{CustomSerialization, CustomSerializationError};

pub struct BinCodeSerializer {}

impl CustomSerialization for BinCodeSerializer {
    fn serialize<T>(&self, value: &T) -> Result<Vec<u8>, CustomSerializationError>
    where
        T: ?Sized + Serialize,
    {
        match bincode::serialize(value) {
            Ok(vec) => Ok(vec),
            Err(_) => Err(CustomSerializationError)
        }
    }

    fn deserialize<'a, T>(&self, s: &'a [u8]) -> Result<T, CustomSerializationError>
    where
        T: ?Sized + Deserialize<'a>,
    {
        match bincode::deserialize(s) {
            Ok(val) => Ok(val),
            Err(_) => Err(CustomSerializationError)
        }
    }
}
