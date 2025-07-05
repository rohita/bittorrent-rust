use serde_json;

pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next().unwrap() {
        'i' => decode_integer(encoded_value),
        'l' => decode_list(encoded_value),
        'd' => decode_dictionary(encoded_value),
        c if c.is_digit(10) => decode_string(encoded_value),
        _ => panic!("Unhandled encoded value: {}", encoded_value)
    }
}

/// Parse dictionary: d<key><value><key><value>e
fn decode_dictionary(encoded: &str) -> (serde_json::Value, &str) {
    let mut dict = serde_json::Map::new();
    let mut remaining = &encoded[1..]; // Skip 'd'

    while !remaining.is_empty() && !remaining.starts_with('e') {
        let (key, next) = decode_bencoded_value(remaining);
        let (value, next) = decode_bencoded_value(next);
        if let serde_json::Value::String(k) = key {
            dict.insert(k, value);
        }
        remaining = next;
    }

    (serde_json::Value::Object(dict), &remaining[1..]) // Skip 'e'
}

/// Parse list: l<values>e
fn decode_list(encoded_value: &str) -> (serde_json::Value, &str) {
    let mut arr: Vec<serde_json::Value> = Vec::new();
    let mut remaining = &encoded_value[1..]; // Skip 'l'
    while !remaining.is_empty() &&  !remaining.starts_with("e") {
        let (value, next) = decode_bencoded_value(remaining);
        arr.push(value);
        remaining = next;
    }
    (serde_json::Value::Array(arr), &remaining[1..]) // Skip 'e'
}

/// Parse integer: i<number>e
fn decode_integer(encoded_value: &str) -> (serde_json::Value, &str) {
    let end = encoded_value.find('e').unwrap();
    let number = encoded_value[1..end].parse::<i64>().unwrap();
    (serde_json::Value::Number(number.into()), &encoded_value[end+1..])
}

/// Parse string: <length>:<string>
fn decode_string(encoded_value: &str) -> (serde_json::Value, &str) {
    let colon_index = encoded_value.find(':').unwrap();
    let length_str = &encoded_value[..colon_index];
    let length = length_str.parse::<usize>().unwrap();

    let start = colon_index + 1;
    let end = start + length;
    let string = &encoded_value[start..end];

    (serde_json::Value::String(string.to_string()), &encoded_value[end..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        // String
        assert("4:pear", "\"pear\"");
        assert("9:raspberry", "\"raspberry\"");
        assert("55:http://bittorrent-test-tracker.codecrafters.io/announce",
               "\"http://bittorrent-test-tracker.codecrafters.io/announce\"");

        // Integer
        assert("i1052617151e", "1052617151");
        assert("i-52e", "-52");

        // List
        assert("le", "[]");
        assert("l9:pineapplei317ee", "[\"pineapple\",317]");
        assert("lli317e9:pineappleee", "[[317,\"pineapple\"]]");
        assert("lli4eei5ee", "[[4],5]");

        // Dictionary
        assert("de", "{}");
        assert("d3:foo5:apple5:helloi52ee", "{\"foo\":\"apple\",\"hello\":52}");
        assert("d10:inner_dictd4:key16:value14:key2i42e8:list_keyl5:item15:item2i3eeee",
               "{\"inner_dict\":{\"key1\":\"value1\",\"key2\":42,\"list_key\":[\"item1\",\"item2\",3]}}");
    }

    fn assert(input: &str, expected: &str) {
        assert_eq!(decode_bencoded_value(input).0.to_string(), expected);
    }
}