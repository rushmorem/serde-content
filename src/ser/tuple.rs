use crate::ser::Value;
use crate::Error;
use crate::Serializer;
use alloc::vec::Vec;
use serde::ser;

pub struct Tuple {
    vec: Vec<Value>,
    human_readable: bool,
}

impl Tuple {
    pub(super) const fn new(vec: Vec<Value>, human_readable: bool) -> Self {
        Self {
            vec,
            human_readable,
        }
    }
}

impl ser::SerializeTuple for Tuple {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + ser::Serialize,
    {
        let value = value.serialize(Serializer {
            human_readable: self.human_readable,
        })?;
        self.vec.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(Value::Tuple(self.vec))
    }
}
