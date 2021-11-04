// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-desub.
//
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

use super::{Composite, Primitive, Value, Variant};
use serde::{self, de::Visitor, Deserialize, Deserializer};
use std::convert::TryInto;

/*
This module implements the [`Deserialize`] (no R!) trait on our [`Value`] enum.
======================================================================

See deserializer.rs for more of a description.

The Deserialize trait is responsible for describing how some other value (or at least,
the repreentastion of it in terms of the serde data model) can be turned into our `Value`
enum.

One thing we aim for is to be able to losslessly deserialize a [`Value`] into a
[`Value`]. This would allow for partial type deserialization, for instance we might want to turn
only part of our input into a struct, say, and leave the rest as [`Value`] types until we know what
to do with them.
*/

impl<'de> Deserialize<'de> for Value {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_any(ValueVisitor)
	}
}

impl<'de> Deserialize<'de> for Primitive {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_any(PrimitiveVisitor)
	}
}

impl<'de> Deserialize<'de> for Composite {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_any(CompositeVisitor)
	}
}

impl<'de> Deserialize<'de> for Variant {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_any(VariantVisitor)
	}
}

struct PrimitiveVisitor;

impl<'de> Visitor<'de> for PrimitiveVisitor {
	type Value = Primitive;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a type that can be decoded into a Primitive value")
	}

	fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::Bool(v))
	}

	fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::I8(v))
	}

	fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::I16(v))
	}

	fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::I32(v))
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::I64(v))
	}

	fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::I128(v))
	}

	fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::U8(v))
	}

	fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::U16(v))
	}

	fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::U32(v))
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::U64(v))
	}

	fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::U128(v))
	}

	fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::Char(v))
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::Str(v.into()))
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Primitive::Str(v))
	}

	fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		let val = v.try_into().map_err(|_| serde::de::Error::invalid_type(serde::de::Unexpected::Bytes(v), &self))?;
		Ok(Primitive::U256(val))
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::SeqAccess<'de>,
	{
		let mut vals = Vec::new();
		while let Some(el) = seq.next_element()? {
			vals.push(el)
		}
		let len = vals.len();
		let arr = vals.try_into().map_err(|_| serde::de::Error::invalid_length(len, &"exactly 32 bytes"))?;
		Ok(Primitive::U256(arr))
	}

	fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Primitive::deserialize(deserializer)
	}
}

struct CompositeVisitor;

impl<'de> Visitor<'de> for CompositeVisitor {
	type Value = Composite;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a type that can be decoded into a Composite value")
	}

	fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		let byte_values = v.iter().map(|&b| Value::Primitive(Primitive::U8(b))).collect();
		Ok(Composite::Unnamed(byte_values))
	}

	fn visit_none<E>(self) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Composite::Unnamed(Vec::new()))
	}

	fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		Composite::deserialize(deserializer)
	}

	fn visit_unit<E>(self) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Composite::Unnamed(Vec::new()))
	}

	fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		Composite::deserialize(deserializer)
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::SeqAccess<'de>,
	{
		let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
		while let Some(value) = seq.next_element()? {
			values.push(value);
		}
		Ok(Composite::Unnamed(values))
	}

	fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::MapAccess<'de>,
	{
		let mut values = Vec::with_capacity(map.size_hint().unwrap_or(0));
		while let Some(key_val) = map.next_entry()? {
			values.push(key_val);
		}
		Ok(Composite::Named(values))
	}
}

struct VariantVisitor;

impl<'de> Visitor<'de> for VariantVisitor {
	type Value = Variant;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a type that can be decoded into an enum Variant")
	}

	fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::EnumAccess<'de>,
	{
		data.variant().and_then(|(name, variant_access)| {
			use serde::de::VariantAccess;
			// We have to ask for a particular enum type, but we don't know what type
			// of enum to expect (we support anything!). So, we just call the visitor method
			// that doesn't require any extra fields, and we know that this will just give back
			// whatever it can based on our impl (who knows about other impls though).
			let values = variant_access.newtype_variant()?;
			Ok(Variant { name, values })
		})
	}
}

struct ValueVisitor;

// It gets repetitive writing out the visitor impls to delegate to the Value subtypes;
// this helper makes that a little easier:
macro_rules! delegate_visitor_fn {
	(
		$visitor:ident $mapping:path,
		$( $name:ident($($ty:ty)?) )+
	) => {
		$(
			fn $name<E>(self, $(v: $ty)?) -> Result<Self::Value, E>
			where E: serde::de::Error {
				$visitor.$name($(v as $ty)?).map($mapping)
			}
		)+
	}
}

impl<'de> Visitor<'de> for ValueVisitor {
	type Value = Value;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a type that can be decoded into a Value")
	}

	delegate_visitor_fn!(
		PrimitiveVisitor Value::Primitive,
		visit_bool(bool)
		visit_i8(i8)
		visit_i16(i16)
		visit_i32(i32)
		visit_i64(i64)
		visit_i128(i128)
		visit_u8(u8)
		visit_u16(u16)
		visit_u32(u32)
		visit_u64(u64)
		visit_u128(u128)
		visit_char(char)
		visit_str(&str)
		visit_string(String)
	);

	delegate_visitor_fn!(
		CompositeVisitor Value::Composite,
		visit_none()
		visit_unit()
		visit_bytes(&[u8])
	);

	fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Value::deserialize(deserializer)
	}

	fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Value::deserialize(deserializer)
	}

	fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::SeqAccess<'de>,
	{
		CompositeVisitor.visit_seq(seq).map(Value::Composite)
	}

	fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::MapAccess<'de>,
	{
		CompositeVisitor.visit_map(map).map(Value::Composite)
	}

	fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::EnumAccess<'de>,
	{
		VariantVisitor.visit_enum(data).map(Value::Variant)
	}
}

#[cfg(test)]
mod test {

	use super::*;
	use crate::value::DeserializeError;

	/// Does a value deserialize to itself?
	fn assert_value_isomorphic<'de, V: Deserializer<'de> + Deserialize<'de> + PartialEq + std::fmt::Debug + Clone>(
		val: V,
	) {
		assert_value_to_value(val.clone(), val)
	}

	/// Does a value `a` deserialize to the expected value `b`?
	fn assert_value_to_value<'de, V1, V2>(a: V1, b: V2)
	where
		V1: Deserializer<'de>,
		V2: Deserialize<'de> + PartialEq + std::fmt::Debug + Clone,
	{
		let new_val = V2::deserialize(a).expect("Can deserialize");
		assert_eq!(b, new_val);
	}

	#[test]
	fn deserialize_primitives_isomorphic() {
		assert_value_isomorphic(Value::Primitive(Primitive::U8(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::U16(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::U32(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::U64(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::U128(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::I8(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::I16(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::I32(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::I64(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::I128(123)));
		assert_value_isomorphic(Value::Primitive(Primitive::Bool(true)));
		assert_value_isomorphic(Value::Primitive(Primitive::Char('a')));
		assert_value_isomorphic(Value::Primitive(Primitive::Str("Hello!".into())));

		// Alas, I256 and U256 are both a sequence of bytes, which could equally be represented
		// by a composite sequence (as other sequences-of-things are). We could have a special case where
		// precisely 32 u8's is deserialized to one of U256 or I256, but for now we use our more general
		// composite type as the sequence catch-all:
		assert_value_to_value(
			Value::Primitive(Primitive::I256([1; 32])),
			Value::Composite(Composite::Unnamed(
				vec![1; 32].into_iter().map(|b| Value::Primitive(Primitive::U8(b))).collect(),
			)),
		);
		assert_value_to_value(
			Value::Primitive(Primitive::U256([1; 32])),
			Value::Composite(Composite::Unnamed(
				vec![1; 32].into_iter().map(|b| Value::Primitive(Primitive::U8(b))).collect(),
			)),
		);

		// .. that said; if you want a primitive value back, you can use that type directly to get it
		// (as long as we are given exactly 32 bytes):

		assert_value_to_value(Value::Primitive(Primitive::I256([1; 32])), Primitive::U256([1; 32]));
		assert_value_to_value(Value::Primitive(Primitive::U256([1; 32])), Primitive::U256([1; 32]));

		// Unwrapped versions also work:

		assert_value_isomorphic(Primitive::U8(123));
		assert_value_isomorphic(Primitive::U16(123));
		assert_value_isomorphic(Primitive::U32(123));
		assert_value_isomorphic(Primitive::U64(123));
		assert_value_isomorphic(Primitive::U128(123));
		assert_value_isomorphic(Primitive::U256([1; 32]));
		assert_value_isomorphic(Primitive::I8(123));
		assert_value_isomorphic(Primitive::I16(123));
		assert_value_isomorphic(Primitive::I32(123));
		assert_value_isomorphic(Primitive::I64(123));
		assert_value_isomorphic(Primitive::I128(123));
		assert_value_isomorphic(Primitive::Bool(true));
		assert_value_isomorphic(Primitive::Char('a'));
		assert_value_isomorphic(Primitive::Str("Hello!".into()));
		assert_value_to_value(Primitive::I256([1; 32]), Primitive::U256([1; 32]));

		// We can also go from wrapped to unwrapped:

		assert_value_to_value(Value::Primitive(Primitive::U8(123)), Primitive::U8(123));
		assert_value_to_value(Value::Primitive(Primitive::U16(123)), Primitive::U16(123));
		assert_value_to_value(Value::Primitive(Primitive::U32(123)), Primitive::U32(123));
		assert_value_to_value(Value::Primitive(Primitive::U64(123)), Primitive::U64(123));

		// Or vice versa:

		assert_value_to_value(Primitive::U8(123), Value::Primitive(Primitive::U8(123)));
		assert_value_to_value(Primitive::U16(123), Value::Primitive(Primitive::U16(123)));
		assert_value_to_value(Primitive::U32(123), Value::Primitive(Primitive::U32(123)));
		assert_value_to_value(Primitive::U64(123), Value::Primitive(Primitive::U64(123)));
	}

	#[test]
	fn deserialize_composites_isomorphic() {
		assert_value_isomorphic(Value::Composite(Composite::Unnamed(vec![
			Value::Primitive(Primitive::U64(123)),
			Value::Primitive(Primitive::Bool(true)),
		])));
		assert_value_isomorphic(Value::Composite(Composite::Unnamed(vec![])));
		assert_value_isomorphic(Value::Composite(Composite::Named(vec![
			("a".into(), Value::Primitive(Primitive::U64(123))),
			("b".into(), Value::Primitive(Primitive::Bool(true))),
		])));
		assert_value_isomorphic(Value::Composite(Composite::Named(vec![
			("a".into(), Value::Primitive(Primitive::U64(123))),
			(
				"b".into(),
				Value::Composite(Composite::Named(vec![
					("c".into(), Value::Primitive(Primitive::U128(123))),
					("d".into(), Value::Primitive(Primitive::Str("hell".into()))),
				])),
			),
		])));

		// unwrapped:

		assert_value_isomorphic(Composite::Unnamed(vec![
			Value::Primitive(Primitive::U64(123)),
			Value::Primitive(Primitive::Bool(true)),
		]));
		assert_value_isomorphic(Composite::Unnamed(vec![]));
		assert_value_isomorphic(Composite::Named(vec![
			("a".into(), Value::Primitive(Primitive::U64(123))),
			("b".into(), Value::Primitive(Primitive::Bool(true))),
		]));
		assert_value_isomorphic(Composite::Named(vec![
			("a".into(), Value::Primitive(Primitive::U64(123))),
			(
				"b".into(),
				Value::Composite(Composite::Named(vec![
					("c".into(), Value::Primitive(Primitive::U128(123))),
					("d".into(), Value::Primitive(Primitive::Str("hell".into()))),
				])),
			),
		]));
	}

	#[test]
	fn deserialize_variants_isomorphic() {
		assert_value_isomorphic(Value::Variant(Variant {
			name: "Foo".into(),
			values: Composite::Unnamed(vec![
				Value::Primitive(Primitive::U64(123)),
				Value::Primitive(Primitive::Bool(true)),
			]),
		}));
		assert_value_isomorphic(Value::Variant(Variant { name: "Foo".into(), values: Composite::Unnamed(vec![]) }));
		assert_value_isomorphic(Value::Variant(Variant {
			name: "Foo".into(),
			values: Composite::Named(vec![
				("a".into(), Value::Primitive(Primitive::U64(123))),
				("b".into(), Value::Primitive(Primitive::Bool(true))),
			]),
		}));

		// unwrapped work as well:

		assert_value_isomorphic(Variant {
			name: "Foo".into(),
			values: Composite::Unnamed(vec![
				Value::Primitive(Primitive::U64(123)),
				Value::Primitive(Primitive::Bool(true)),
			]),
		});
		assert_value_isomorphic(Variant { name: "Foo".into(), values: Composite::Unnamed(vec![]) });
		assert_value_isomorphic(Variant {
			name: "Foo".into(),
			values: Composite::Named(vec![
				("a".into(), Value::Primitive(Primitive::U64(123))),
				("b".into(), Value::Primitive(Primitive::Bool(true))),
			]),
		});
	}

	#[test]
	fn sequence_to_value() {
		use serde::de::{value::SeqDeserializer, IntoDeserializer};

		let de: SeqDeserializer<_, DeserializeError> = vec![1u8, 2, 3, 4].into_deserializer();

		assert_value_to_value(
			de.clone(),
			Value::Composite(Composite::Unnamed(vec![
				Value::Primitive(Primitive::U8(1)),
				Value::Primitive(Primitive::U8(2)),
				Value::Primitive(Primitive::U8(3)),
				Value::Primitive(Primitive::U8(4)),
			])),
		);
		assert_value_to_value(
			de,
			Composite::Unnamed(vec![
				Value::Primitive(Primitive::U8(1)),
				Value::Primitive(Primitive::U8(2)),
				Value::Primitive(Primitive::U8(3)),
				Value::Primitive(Primitive::U8(4)),
			]),
		);
	}

	#[test]
	fn sequence_to_primitive() {
		use serde::de::{value::SeqDeserializer, IntoDeserializer};

		let de: SeqDeserializer<_, DeserializeError> = vec![1u8; 32].into_deserializer();

		assert_value_to_value(de, Primitive::U256([1; 32]));
	}

	#[test]
	fn map_to_value() {
		use serde::de::{value::MapDeserializer, IntoDeserializer};
		use std::collections::HashMap;

		let map = {
			let mut map = HashMap::<&'static str, i32>::new();
			map.insert("a", 1i32);
			map.insert("b", 2i32);
			map.insert("c", 3i32);
			map
		};

		let de: MapDeserializer<_, DeserializeError> = map.into_deserializer();

		let value = Value::deserialize(de).expect("should deserialize OK");
		if let Value::Composite(Composite::Named(vals)) = value {
			// These could come back in any order so we need to search for them:
			assert!(vals.contains(&("a".into(), Value::Primitive(Primitive::I32(1)))));
			assert!(vals.contains(&("b".into(), Value::Primitive(Primitive::I32(2)))));
			assert!(vals.contains(&("c".into(), Value::Primitive(Primitive::I32(3)))));
		} else {
			panic!("Map should deserialize into Composite::Named value but we have {:?}", value);
		}
	}

	#[test]
	fn partially_deserialize_value() {
		let value = Value::Composite(Composite::Named(vec![
			("a".into(), Value::Primitive(Primitive::U64(123))),
			(
				"b".into(),
				Value::Composite(Composite::Named(vec![
					("c".into(), Value::Primitive(Primitive::U128(123))),
					("d".into(), Value::Primitive(Primitive::Str("hell".into()))),
					("e".into(), Value::Composite(Composite::Unnamed(vec![]))),
				])),
			),
		]));

		#[derive(Deserialize, Debug, PartialEq)]
		struct Partial {
			a: Value,
			b: PartialB,
		}

		#[derive(Deserialize, Debug, PartialEq)]
		struct PartialB {
			c: u128,
			d: String,
			e: Value,
		}

		let partial: Partial = crate::value::from_value(value).expect("should work");

		assert_eq!(
			partial,
			Partial {
				a: Value::Primitive(Primitive::U64(123)),
				b: PartialB { c: 123, d: "hell".into(), e: Value::Composite(Composite::Unnamed(vec![])) }
			}
		)
	}
}
