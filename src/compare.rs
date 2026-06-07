// Compare two json values, exact or partial match

use ureq::serde_json::Value;

/// Recursively compare two serde Values.
/// reference is the reference your value will be compared to (the one containing custom anotations)
/// datas are the actual datas you want to verify
pub fn compare_jsons(reference: &Value, datas: &Value) -> bool {
    // Three cases :
    // 1 - value to match
    // 2 - array to match
    // 3 - object to match

    match reference {
        Value::Null => datas.is_null(),
        Value::Number(_) => datas == reference,
        Value::Bool(_) => datas == reference,
        Value::String(s) => {
            // 1 - check if the json is correctly called
            match s.as_str() {
                "\\string" => datas.is_string(),
                "\\number" => datas.is_number(),
                "\\bool" => datas.is_boolean(),
                "\\boolean" => datas.is_boolean(),
                "\\object" => datas.is_object(),
                "\\array" => datas.is_array(),
                _ => datas == reference,
            }
        }
        Value::Array(arr) => match datas.as_array() {
            Some(data_arr) if data_arr.len() == arr.len() => arr
                .iter()
                .zip(data_arr.iter())
                .all(|(r, d)| compare_jsons(r, d)),
            _ => false,
        },

        Value::Object(obj) => match datas.as_object() {
            Some(data_obj) => obj.iter().all(|(key, ref_val)| {
                data_obj
                    .get(key)
                    .is_some_and(|data_val| compare_jsons(ref_val, data_val))
            }),
            None => false,
        },
    }
}
