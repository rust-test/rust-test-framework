use crate::attributes::common::value_to_suffix;
use serde_json::Value;

#[test]
fn test_value_to_suffix_all_variants() {
    assert_eq!(value_to_suffix(&Value::Null), "null");
    assert_eq!(value_to_suffix(&Value::Bool(true)), "true");
    assert_eq!(
        value_to_suffix(&Value::Number(serde_json::Number::from(123))),
        "123"
    );
    assert_eq!(value_to_suffix(&Value::String("A b".to_string())), "a_b");
    assert_eq!(
        value_to_suffix(&Value::Array(vec![Value::from(1), Value::from(2)])),
        "1_2"
    );
    let mut map = serde_json::Map::new();
    map.insert("k".to_string(), Value::from(3));
    assert_eq!(value_to_suffix(&Value::Object(map)), "3");
}