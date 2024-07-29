use crate::ser::Value;
use crate::Data;
use crate::Error;
use crate::Serializer;
use alloc::boxed::Box;
use serde::ser;
use serde::ser::SerializeStructVariant;
use serde::ser::SerializeTupleVariant;

pub struct Enum {
    r#enum: crate::Enum<'static>,
    human_readable: bool,
}

impl Enum {
    pub(super) const fn new(r#enum: crate::Enum<'static>, human_readable: bool) -> Self {
        Self {
            r#enum,
            human_readable,
        }
    }
}

impl ser::Serialize for crate::Enum<'static> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match &self.data {
            Data::Unit => {
                serializer.serialize_unit_variant(self.name, self.variant_index, self.variant)
            }
            Data::NewType { value } => serializer.serialize_newtype_variant(
                self.name,
                self.variant_index,
                self.variant,
                value,
            ),
            Data::Tuple { values } => {
                let mut tup = serializer.serialize_tuple_variant(
                    self.name,
                    self.variant_index,
                    self.variant,
                    values.len(),
                )?;
                for value in values {
                    tup.serialize_field(value)?;
                }
                tup.end()
            }
            Data::Struct { fields } => {
                let mut map = serializer.serialize_struct_variant(
                    self.name,
                    self.variant_index,
                    self.variant,
                    fields.len(),
                )?;
                for (key, value) in fields {
                    map.serialize_field(key, value)?;
                }
                map.end()
            }
        }
    }
}

impl ser::SerializeStructVariant for Enum {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if let Data::Struct { fields } = &mut self.r#enum.data {
            let value = value.serialize(Serializer {
                human_readable: self.human_readable,
            })?;
            fields.push((key, value));
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(Value::Enum(Box::new(self.r#enum)))
    }
}

impl ser::SerializeTupleVariant for Enum {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if let Data::Tuple { values } = &mut self.r#enum.data {
            let value = value.serialize(Serializer {
                human_readable: self.human_readable,
            })?;
            values.push(value);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(Value::Enum(Box::new(self.r#enum)))
    }
}
