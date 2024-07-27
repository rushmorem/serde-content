use crate::ser::Content;
use crate::Data;
use crate::Error;
use crate::Serializer;
use alloc::boxed::Box;
use serde::ser;
use serde::ser::SerializeStruct;
use serde::ser::SerializeTupleStruct;

pub struct Struct {
    r#struct: crate::Struct<'static>,
    human_readable: bool,
}

impl Struct {
    pub(super) const fn new(r#struct: crate::Struct<'static>, human_readable: bool) -> Self {
        Self {
            r#struct,
            human_readable,
        }
    }
}

impl ser::Serialize for crate::Struct<'static> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match &self.data {
            Data::Unit => serializer.serialize_unit_struct(self.name),
            Data::NewType { value } => serializer.serialize_newtype_struct(self.name, value),
            Data::Tuple { values } => {
                let mut tup = serializer.serialize_tuple_struct(self.name, values.len())?;
                for value in values {
                    tup.serialize_field(value)?;
                }
                tup.end()
            }
            Data::Struct { fields } => {
                let mut map = serializer.serialize_struct(self.name, fields.len())?;
                for (key, value) in fields {
                    map.serialize_field(key, value)?;
                }
                map.end()
            }
        }
    }
}

impl ser::SerializeStruct for Struct {
    type Ok = Content;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if let Data::Struct { fields } = &mut self.r#struct.data {
            let value = value.serialize(Serializer {
                human_readable: self.human_readable,
            })?;
            fields.push((key, value));
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(Content::Struct(Box::new(self.r#struct)))
    }
}

impl ser::SerializeTupleStruct for Struct {
    type Ok = Content;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if let Data::Tuple { values } = &mut self.r#struct.data {
            let value = value.serialize(Serializer {
                human_readable: self.human_readable,
            })?;
            values.push(value);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(Content::Struct(Box::new(self.r#struct)))
    }
}
