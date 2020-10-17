use crate::{
    configuration::{ConfigurationRoot, Key, TypedValue},
    error::{ConfigurationError, ErrorCode},
};
use serde::{
    de::{
        self, value::StrDeserializer, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess,
        SeqAccess, VariantAccess, Visitor,
    },
    forward_to_deserialize_any,
};
use std::{
    collections::hash_map::{Keys, Values},
    convert::TryInto,
    iter::{Enumerate, Peekable},
    slice::Iter,
};

impl<'de> de::Deserializer<'de> for &'de ConfigurationRoot {
    type Error = ConfigurationError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            ConfigurationRoot::Value(vt) => match vt {
                Some(TypedValue::Float(v)) => visitor.visit_f64(*v),
                Some(TypedValue::String(v)) => visitor.visit_string(v.clone()),
                Some(TypedValue::SignedInteger(v)) => visitor.visit_i64(*v),
                Some(TypedValue::Bool(v)) => visitor.visit_bool(*v),
                None => visitor.visit_none(),
            },
            ConfigurationRoot::Map(m) => {
                visitor.visit_map(MapAccessor(m.keys().peekable(), m.values()))
            }
            ConfigurationRoot::Array(a) => visitor.visit_seq(SeqAccessor(a.iter().enumerate())),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(TryInto::try_into(self)?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(TryInto::try_into(self)?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(TryInto::try_into(self)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(TryInto::try_into(self)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(TryInto::try_into(self)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(TryInto::try_into(self)?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(TryInto::try_into(self)?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(TryInto::try_into(self)?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(TryInto::try_into(self)?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(TryInto::try_into(self)?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(TryInto::try_into(self)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let self_string: String = TryInto::try_into(self)?;
        let characters: Vec<char> = self_string.chars().collect();
        if characters.len() == 1 {
            visitor.visit_char(characters[0])
        } else {
            Err(ErrorCode::SerdeError(
                "Expected string to have length 1 to deserialize as char.".into(),
            )
            .into()) // TODO: Fix this
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(TryInto::try_into(self)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(TryInto::try_into(self)?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            ConfigurationRoot::Value(Some(_)) => visitor.visit_some(self),
            ConfigurationRoot::Value(None) => visitor.visit_none(),
            _ => Err(
                ErrorCode::SerdeError("Expected value to deserialize optional value.".into())
                    .into(),
            ), // TODO: Fix this
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            ConfigurationRoot::Value(Some(TypedValue::String(s))) => {
                if s.trim().is_empty() {
                    visitor.visit_unit()
                } else {
                    Err(ErrorCode::SerdeError(
                        "Expected string to be empty or whitespace to deserialize unit".into(),
                    )
                    .into())
                }
            }
            ConfigurationRoot::Value(None) => visitor.visit_unit(),
            _ => Err(ErrorCode::SerdeError("Expected None to deserialize unit".into()).into()), // TODO: Fix this
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(EnumAccessor {
            enum_name: name,
            root: self,
        })
    }

    forward_to_deserialize_any!(bytes byte_buf seq map tuple tuple_struct struct identifier ignored_any);
}

struct MapAccessor<'conf>(
    Peekable<Keys<'conf, String, ConfigurationRoot>>,
    Values<'conf, String, ConfigurationRoot>,
);

impl<'de> MapAccess<'de> for MapAccessor<'de> {
    type Error = ConfigurationError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.0.peek() {
            Some(v) => {
                let deserializer: StrDeserializer<ConfigurationError> =
                    v.as_str().into_deserializer();
                let key = seed
                    .deserialize(deserializer)
                    .map_err(|e| e.enrich_with_key(Key::Map((*v).to_owned())))?;
                Ok(Some(key))
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let key = self.0.next();
        if let None = key {
            return Err(ErrorCode::SerdeError(
                "Unknown key in map while deserializing value".into(),
            )
            .into());
        }

        let key = key.unwrap();

        match self.1.next() {
            Some(v) => Ok(seed
                .deserialize(v)
                .map_err(|e| e.enrich_with_key(Key::Map(key.to_owned())))?),
            None => Err(ErrorCode::SerdeError(
                "Expected value to be Some for next map entry".into(),
            )
            .into()), // TODO: Fix it
        }
    }
}

struct SeqAccessor<'conf>(Enumerate<Iter<'conf, ConfigurationRoot>>);

impl<'de> SeqAccess<'de> for SeqAccessor<'de> {
    type Error = ConfigurationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.0.next() {
            Some((index, v)) => Ok(Some(
                seed.deserialize(v)
                    .map_err(|e| e.enrich_with_key(Key::Array(index)))?,
            )),
            None => Ok(None),
        }
    }
}

struct EnumAccessor<'conf> {
    enum_name: &'static str,
    root: &'conf ConfigurationRoot,
}

impl<'de> EnumAccess<'de> for EnumAccessor<'de> {
    type Error = ConfigurationError;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.root {
            ConfigurationRoot::Value(Some(TypedValue::String(v))) => {
                let deserializer: StrDeserializer<ConfigurationError> =
                    v.as_str().into_deserializer();
                let value = seed.deserialize(deserializer)?;

                Ok((value, self))
            }
            ConfigurationRoot::Map(m) => {
                if m.len() != 1 {
                    return Err(ErrorCode::SerdeError(
                        "Attempt to deserialize enum from map longer than 1.".into(),
                    )
                    .into());
                }

                // if !m.contains_key(self.enum_name) {
                //     return Err(ErrorCode::SerdeError(format!(
                //         "Map does not contain key {} required by enum deserializer.",
                //         self.enum_name
                //     ))
                //     .into());
                // }

                let key = m.keys().nth(0).unwrap().as_str();
                let deserializer: StrDeserializer<ConfigurationError> = key.into_deserializer();

                let value = seed.deserialize(deserializer)?;

                self.root = m.get(key).unwrap(); // safe due to previous check;

                Ok((value, self))
            }
            _ => Err(ErrorCode::SerdeError("".into()).into()), // TODO: Fix it
        }
    }
}

impl<'de> VariantAccess<'de> for EnumAccessor<'de> {
    type Error = ConfigurationError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(()) // TODO: Is it correct?
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.root {
            ConfigurationRoot::Value(Some(tv)) => seed.deserialize(tv),
            _ => Err(ErrorCode::SerdeError("".into()).into()),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.root {
            ConfigurationRoot::Array(a) => visitor.visit_seq(SeqAccessor(a.iter().enumerate())),
            _ => Err(ErrorCode::SerdeError("".into()).into()),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.root {
            ConfigurationRoot::Map(m) => {
                visitor.visit_map(MapAccessor(m.keys().peekable(), m.values()))
            }
            _ => Err(ErrorCode::SerdeError("".into()).into()),
        }
    }
}

impl<'de> de::Deserializer<'de> for &TypedValue {
    type Error = ConfigurationError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            TypedValue::String(s) => visitor.visit_str(s.as_str()),
            TypedValue::Bool(b) => visitor.visit_bool(*b),
            TypedValue::Float(f) => visitor.visit_f64(*f),
            TypedValue::SignedInteger(i) => visitor.visit_i64(*i),
        }
    }

    forward_to_deserialize_any!(
        ignored_any identifier enum struct map tuple_struct tuple
        seq newtype_struct unit_struct byte_buf bytes unit option
        string str char f32 f64 i8 i16 i32 i64 u8 u16 u32 u64 bool
    );
}
