use crate::ser::Value;
use crate::Error;
use crate::Serializer;
use alloc::vec::Vec;
use serde::ser;

pub struct Map {
    vec: Vec<(Value, Value)>,
    human_readable: bool,
}

impl Map {
    pub(super) const fn new(vec: Vec<(Value, Value)>, human_readable: bool) -> Self {
        Self {
            vec,
            human_readable,
        }
    }
}

impl ser::SerializeMap for Map {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + ser::Serialize,
    {
        let key = key.serialize(Serializer {
            human_readable: self.human_readable,
        })?;
        self.vec.push((key, Value::Unit));
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if let Some(last) = self.vec.last_mut() {
            last.1 = value.serialize(Serializer {
                human_readable: self.human_readable,
            })?;
        }
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        let serializer = Serializer {
            human_readable: self.human_readable,
        };
        let key = key.serialize(serializer)?;
        let value = value.serialize(serializer)?;
        self.vec.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(Value::Map(self.vec))
    }
}
