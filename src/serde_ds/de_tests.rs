use std;
use std::collections::HashMap;
use datastore::{Entity, Value};
use serde_ds::de;
use serde_ds::Error;

#[test]
fn test_deserialize_ints() {
    let input = Value::from(42);

    // Signed integer types

    let res_i8: i8 = de::from_value(input.clone()).expect("i8 deserialization failed");
    assert_eq!(42, res_i8);

    let res_i16: i16 = de::from_value(input.clone()).expect("i16 deserialization failed");
    assert_eq!(42, res_i16);

    let res_i32: i32 = de::from_value(input.clone()).expect("i32 deserialization failed");
    assert_eq!(42, res_i32);

    let res_i64: i64 = de::from_value(input.clone()).expect("i64 deserialization failed");
    assert_eq!(42, res_i64);

    // Unsigned integer types

    let res_u8: u8 = de::from_value(input.clone()).expect("u8 deserialization failed");
    assert_eq!(42, res_u8);

    let res_u16: u16 = de::from_value(input.clone()).expect("u16 deserialization failed");
    assert_eq!(42, res_u16);

    let res_u32: u32 = de::from_value(input.clone()).expect("u32 deserialization failed");
    assert_eq!(42, res_u32);

    let res_u64: u64 = de::from_value(input.clone()).expect("u64 deserialization failed");
    assert_eq!(42, res_u64);
}

#[test]
fn test_deserialize_floats() {
    let input = Value::from(10.0);

    let res_f32: f32 = de::from_value(input.clone()).expect("f32 deserialization failed");
    assert_eq!(10.0, res_f32);

    let res_f64: f64 = de::from_value(input.clone()).expect("f64 deserialization failed");
    assert_eq!(10.0, res_f64);
}

#[test]
fn test_float_overflow() {
    let input = Value::from(std::f64::MAX);

    let res_f32 = de::from_value::<f32>(input.clone()).unwrap_err();
    assert_eq!(Error::DoubleSizeMismatch(), res_f32);

    de::from_value::<f64>(input.clone()).expect("f64::MAX deserialization failed");
}

#[test]
fn test_map_deserialization() {
    let one = Value::from(42);
    let entity_value = Entity {
        properties: hashmap!(
            "one".to_string() => one,
        ),
    };
    let input = Value::from(entity_value);

    let result: HashMap<String, u8> = de::from_value(input).expect("map deserialization failed");
    let expected = hashmap!(
        "one".to_string() => 42,
    );

    assert_eq!(expected, result)
}

#[test]
fn test_struct_deserialization() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Language {
        name: String,
        strongly_typed: bool,
    };

    let properties = hashmap!(
        "name".to_string() => Value::from("Rust"),
        "strongly_typed".to_string() => Value::from(true),
    );

    let input = Value::from(Entity { properties });

    let expected = Language {
        name: String::from("Rust"),
        strongly_typed: true,
    };

    let result: Language = de::from_value(input).expect("struct deserialization failed");

    assert_eq!(expected, result);
}

#[test]
fn test_sequence_deserialization() {
    let input = Value::from(vec![Value::from("Hello"), Value::from("World!")]);
    let expected = vec!["Hello".to_string(), "World!".to_string()];
    let result: Vec<String> = de::from_value(input).expect("sequence deserialization failed");

    assert_eq!(expected, result)
}
