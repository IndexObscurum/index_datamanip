use std::collections::HashMap;
use std::str;

use nom::{bytes::complete::*, number::complete::*, IResult};
use serde::de::{self, DeserializeOwned, IntoDeserializer, Visitor};

use crate::common::parse_lstring;
use crate::error::{Error, Result};

/// Parses a byte buffer and string mapping into the given type T
pub fn parse<T: DeserializeOwned>(input: &[u8], strings: &HashMap<u32, String>) -> Result<T> {
    let mut deserializer = Deserializer::from_bytes(input, strings);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

/// Parses an external string
///
/// In-line is a 32bit key into the strings map
fn parse_estring<'a>(input: &'a [u8], strings: &HashMap<u32, String>) -> IResult<&'a [u8], String> {
    let (input, index) = le_u32(input)?;
    match strings.get(&index) {
        Some(s) => Ok((input, s.to_owned())),
        None => Err(nom::Err::Failure((input, nom::error::ErrorKind::Verify))),
    }
}

pub struct Deserializer<'de> {
    input: &'de [u8],
    strings: &'de HashMap<u32, String>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8], strings: &'de HashMap<u32, String>) -> Self {
        Deserializer { input, strings }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, i) = le_u32(self.input)?;
        self.input = new_input;
        visitor.visit_bool(i != 0)
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, i) = le_i32(self.input)?;
        self.input = new_input;
        visitor.visit_i32(i)
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, i) = le_u32(self.input)?;
        self.input = new_input;
        visitor.visit_u8(i as u8)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, i) = le_u32(self.input)?;
        self.input = new_input;
        visitor.visit_u32(i)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, i) = le_f32(self.input)?;
        self.input = new_input;
        visitor.visit_f32(i)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, s) = parse_estring(self.input, self.strings)?;
        self.input = new_input;
        visitor.visit_string(s)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, s) = parse_lstring(self.input)?;
        self.input = new_input;
        visitor.visit_byte_buf(s.as_bytes().to_owned())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, len) = le_u32(self.input)?;
        if len == 0 {
            self.input = new_input;
            return visitor.visit_none();
        }
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, len) = le_u32(self.input)?;
        self.input = new_input;
        self.deserialize_tuple(len as usize, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'a, 'de> {
            deserializer: &'a mut Deserializer<'de>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a> serde::de::SeqAccess<'de> for Access<'a, 'de> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (new_input, len) = le_u32(self.input)?;
        let (new_input, chunk) = take(len)(new_input)?;
        self.input = chunk;
        let ret = self.deserialize_tuple(fields.len(), visitor);
        // Class is known to be incomplete, so ignore those for now
        if name != "Class" && !self.input.is_empty() {
            eprintln!(
                "WARNING: {} had {} extra bytes at the end: {:?}",
                name,
                self.input.len(),
                self.input,
            );
        }
        self.input = new_input;
        ret
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        impl<'de, 'a> serde::de::EnumAccess<'de> for &'a mut Deserializer<'de> {
            type Error = crate::parse7::Error;
            type Variant = Self;

            fn variant_seed<V: serde::de::DeserializeSeed<'de>>(
                self,
                seed: V,
            ) -> std::result::Result<(V::Value, Self), Error> {
                let idx: u32 = serde::de::Deserialize::deserialize(&mut *self)?;
                let moo: serde::de::value::U32Deserializer<Error> = idx.into_deserializer();
                let v = serde::de::DeserializeSeed::deserialize(seed, moo)?;
                Ok((v, self))
            }
        }

        Ok(visitor.visit_enum(self)?)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

impl<'de, 'a> serde::de::VariantAccess<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }
    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}
