use serde_json;

pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, usize) {
    match encoded_value.chars().next().unwrap() {
        'i' => decode_integer(encoded_value),
        'l' => decode_list(encoded_value),
        'd' => decode_dictionary(encoded_value),
        c if c.is_digit(10) => decode_string(encoded_value),
        _ => panic ! ("Unhandled encoded value: {}", encoded_value)
    }
}

fn decode_dictionary(encoded: &str) -> (serde_json::Value, usize) {
    let mut total_len = 0;
    let mut dict = serde_json::Map::new();
    let mut input = &encoded[1..]; // Skip 'd'

    while !input.starts_with('e') {
        let (key_json, key_len) = decode_bencoded_value(input);
        total_len += key_len;
        input = &input[key_len..];

        let (value, val_len) = decode_bencoded_value(input);
        total_len += val_len;
        input = &input[val_len..];

        // Extract the raw key string from the JSON string
        let key = key_json.to_string();
        let trimmed_key: String = key.chars().skip(1).take(key.len() - 2).collect();

        dict.insert(trimmed_key, value);
    }

    (serde_json::Value::Object(dict), total_len + 2) // +2 for 'd' and 'e'
}


fn decode_list(encoded_value: &str) -> (serde_json::Value, usize) {
    // Parse list: l<values>e
    let mut arr: Vec<serde_json::Value> = Vec::new();
    let mut s_len: usize = 0;
    let mut encoded_value = &encoded_value[1..];
    while !encoded_value.starts_with("e") {
        let (value, len) = decode_bencoded_value(encoded_value);
        s_len += len;
        arr.push(value);
        encoded_value = &encoded_value[len..];
    }
    (serde_json::Value::Array(arr), s_len + 2)
}

fn decode_integer(encoded_value: &str) -> (serde_json::Value, usize) {
    // Parse integer: i<number>e
    let end = encoded_value.find('e').unwrap();
    let number = encoded_value[1..end].parse::<i64>().unwrap();
    (serde_json::Value::Number(number.into()), 1 + end)
}

fn decode_string(encoded_value: &str) -> (serde_json::Value, usize) {
    // Parse string: <length>:<string>
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    (serde_json::Value::String(string.to_string()), colon_index + 1 + number as usize)
}