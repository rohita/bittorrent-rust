use serde_json::Value;

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let start_char = encoded_value.chars().next().unwrap();
    if start_char.is_digit(10) {
        decode_string(encoded_value)
    } else if start_char == 'i' {
        decode_integer(encoded_value)
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn decode_integer(encoded_value: &str) -> Value {
    let end = encoded_value.find('e').unwrap();
    let number = encoded_value[1..end].parse::<i64>().unwrap();
    serde_json::Value::Number(serde_json::Number::from(number))
}

fn decode_string(encoded_value: &str) -> Value {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    serde_json::Value::String(string.to_string())
}