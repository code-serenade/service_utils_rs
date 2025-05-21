/// A trait for extracting values from a query string.
pub trait QueryExtractor {
    /// Extracts the value associated with the specified key from the query string.
    ///
    /// # Arguments
    ///
    /// * `key` - The key whose associated value is to be returned.
    ///
    /// # Returns
    ///
    /// * `Option<&str>` - Some(&str) if the key exists, otherwise None.
    fn extract_value(&self, key: &str) -> Option<&str>;
}

impl<T: AsRef<str>> QueryExtractor for T {
    /// Implementation of the `extract_value` method for any type that implements `AsRef<str>`.
    ///
    /// This method splits the query string by '&' to get key-value pairs,
    /// then splits each pair by '=' to separate the key and value. It returns
    /// the value associated with the specified key if found.
    fn extract_value(&self, key: &str) -> Option<&str> {
        self.as_ref().split('&').find_map(|pair| {
            let mut parts = pair.split('=');
            let k = parts.next()?;
            let v = parts.next()?;
            if k == key { Some(v) } else { None }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::QueryExtractor;

    /// Tests the `extract_value` method with a standard query string.
    #[test]
    fn test_extract_value() {
        let query = "key1=val1&key2=val2&key3=val3";
        let query_string = String::from(query);

        assert_eq!(query.extract_value("key1"), Some("val1"));
        assert_eq!(query.extract_value("key2"), Some("val2"));
        assert_eq!(query.extract_value("key3"), Some("val3"));
        assert_eq!(query.extract_value("key4"), None);

        assert_eq!(query_string.extract_value("key1"), Some("val1"));
        assert_eq!(query_string.extract_value("key2"), Some("val2"));
        assert_eq!(query_string.extract_value("key3"), Some("val3"));
        assert_eq!(query_string.extract_value("key4"), None);
    }

    /// Tests the `extract_value` method with an empty query string.
    #[test]
    fn test_extract_value_empty() {
        let query = "";
        let query_string = String::from(query);

        assert_eq!(query.extract_value("key1"), None);
        assert_eq!(query_string.extract_value("key1"), None);
    }

    /// Tests the `extract_value` method with a query string where a key has no value.
    #[test]
    fn test_extract_value_no_value() {
        let query = "key1=&key2=val2";
        let query_string = String::from(query);

        assert_eq!(query.extract_value("key1"), Some(""));
        assert_eq!(query.extract_value("key2"), Some("val2"));

        assert_eq!(query_string.extract_value("key1"), Some(""));
        assert_eq!(query_string.extract_value("key2"), Some("val2"));
    }

    /// Tests the `extract_value` method with a query string containing multiple keys with the same
    /// name.
    #[test]
    fn test_extract_value_multiple_keys() {
        let query = "key1=val1&key1=val2&key3=val3";
        let query_string = String::from(query);

        assert_eq!(query.extract_value("key1"), Some("val1")); // Only the first occurrence
        assert_eq!(query.extract_value("key3"), Some("val3"));

        assert_eq!(query_string.extract_value("key1"), Some("val1")); // Only the first occurrence
        assert_eq!(query_string.extract_value("key3"), Some("val3"));
    }

    /// Tests the `extract_value` method with a malformed query string.
    #[test]
    fn test_extract_value_malformed_query() {
        let query = "key1=val1&key2&key3=val3";
        let query_string = String::from(query);

        assert_eq!(query.extract_value("key1"), Some("val1"));
        assert_eq!(query.extract_value("key2"), None);
        assert_eq!(query.extract_value("key3"), Some("val3"));

        assert_eq!(query_string.extract_value("key1"), Some("val1"));
        assert_eq!(query_string.extract_value("key2"), None);
        assert_eq!(query_string.extract_value("key3"), Some("val3"));
    }
}
