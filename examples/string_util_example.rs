use service_utils_rs::utils::string_util::QueryExtractor;

fn main() {
    let query = "key1=val1&key2=val2&key3=val3";

    if let Some(value) = query.extract_value("key2") {
        println!("Extracted value: {}", value);
    } else {
        println!("No value found");
    }
}

// cargo run --example string_util_example
