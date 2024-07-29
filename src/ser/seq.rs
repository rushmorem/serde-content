use crate::ser::Value;
use crate::Error;
use crate::Serializer;
use alloc::vec::Vec;
use serde::ser;

pub struct Seq {
    vec: Vec<Value>,
    human_readable: bool,
}

impl Seq {
    pub(super) const fn new(vec: Vec<Value>, human_readable: bool) -> Self {
        Self {
            vec,
            human_readable,
        }
    }
}

impl ser::SerializeSeq for Seq {
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
        Ok(Value::Seq(self.vec))
    }
}
