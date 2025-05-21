use std::mem;

/// A trait that provides a method to parse the header of a collection into a byte vector.
pub trait HeaderParser<T> {
    /// Parses the header of the implementing type and returns it as a vector of bytes.
    ///
    /// # Returns
    /// A `Vec<u8>` containing the byte representation of the header.
    fn parse_header(&self) -> Vec<u8>;

    /// Deserializes a vector of bytes into a vector of the implementing type.
    ///
    /// # Parameters
    /// - `header`: A `Vec<u8>` containing the byte representation to be deserialized.
    ///
    /// # Returns
    /// A `Vec<T>` where each element is deserialized from the byte sequence.
    fn deserialize_header(header: &Vec<u8>) -> Vec<T>;
}

impl<T> HeaderParser<T> for Vec<T>
where
    T: Copy + Into<u64> + TryFrom<u64>,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
{
    /// Parses the header of a `Vec<T>` and converts each element into its big-endian byte
    /// representation.
    ///
    /// This method works for any type `T` that implements the `Copy` and `Into<u64>` traits.
    ///
    /// # Returns
    /// A `Vec<u8>` where each element of the original vector is represented by its big-endian byte
    /// sequence.
    ///
    /// # Example
    /// ```
    /// use service_utils_rs::services::websocket::server::header_parser::HeaderParser;
    ///
    /// let data = vec![1u16, 2u16, 3u16];
    /// let header = data.parse_header();
    /// assert_eq!(header, vec![0, 1, 0, 2, 0, 3]);
    /// ```
    fn parse_header(&self) -> Vec<u8> {
        // Determine the size in bytes of the type `T`
        let type_size = mem::size_of::<T>();
        // Create a vector to hold the resulting byte sequence
        let mut header = vec![0u8; self.len() * type_size];
        // Iterate over each element in the vector
        for (i, &value) in self.iter().enumerate() {
            let s = i * type_size; // Start index for the byte slice
            let e = (i + 1) * type_size; // End index for the byte slice
            let bytes = value.into().to_be_bytes(); // Convert the value to a big-endian byte array
            // Copy the relevant portion of the byte array into the header vector
            header[s .. e].copy_from_slice(&bytes[bytes.len() - type_size ..]);
        }
        header // Return the resulting byte vector
    }

    /// Deserializes a vector of bytes into a vector of type `T`.
    ///
    /// This function assumes that the input byte vector (`header`) was serialized using
    /// the `parse_header` method. It converts each chunk of bytes back into the original
    /// type `T` using big-endian ordering.
    ///
    /// # Parameters
    /// - `header`: A `Vec<u8>` containing the byte sequence to be deserialized.
    ///
    /// # Returns
    /// A `Vec<T>` where each element is deserialized from the corresponding chunk of bytes.
    ///
    /// # Panics
    /// The function will panic if `T::try_from(value)` fails, or if the size of `header`
    /// is not a multiple of the size of `T`.
    fn deserialize_header(header: &Vec<u8>) -> Vec<T> {
        let type_size = mem::size_of::<T>();
        let mut result = Vec::with_capacity(header.len() / type_size);
        for chunk in header.chunks(type_size) {
            let mut array = [0u8; 8]; // Used to store a u64 with up to 64 bits
            let bytes_to_copy = &chunk[0 .. type_size];
            array[8 - type_size ..].copy_from_slice(bytes_to_copy); // Copy bytes to the end of the array
            let value = u64::from_be_bytes(array);
            result.push(T::try_from(value).unwrap());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_u16() {
        let data = vec![1u16, 2u16, 3u16];
        let expected = vec![0, 1, 0, 2, 0, 3]; // Big-endian representation of [1, 2, 3]
        let result = data.parse_header();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_header_u32() {
        let data = vec![1u32, 2u32, 3u32];
        let expected = vec![
            0, 0, 0, 1, // Big-endian representation of 1
            0, 0, 0, 2, // Big-endian representation of 2
            0, 0, 0, 3, // Big-endian representation of 3
        ];
        let result = data.parse_header();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_header_u8() {
        let data = vec![1u8, 2u8, 3u8];
        let expected = vec![1, 2, 3]; // u8 doesn't need endian conversion
        let result = data.parse_header();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_header_u64() {
        let data = vec![1u64, 2u64, 3u64];
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 1, // Big-endian representation of 1
            0, 0, 0, 0, 0, 0, 0, 2, // Big-endian representation of 2
            0, 0, 0, 0, 0, 0, 0, 3, // Big-endian representation of 3
        ];
        let result = data.parse_header();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_header_u16() {
        let original_data = vec![1u16, 2u16, 3u16];
        let serialized = original_data.parse_header();
        let deserialized = Vec::<u16>::deserialize_header(&serialized);
        assert_eq!(original_data, deserialized);
    }

    #[test]
    fn test_deserialize_header_u32() {
        let original_data = vec![1u32, 2u32, 3u32];
        let serialized = original_data.parse_header();
        let deserialized = Vec::<u32>::deserialize_header(&serialized);
        assert_eq!(original_data, deserialized);
    }

    #[test]
    fn test_deserialize_header_u64() {
        let original_data = vec![1u64, 2u64, 3u64];
        let serialized = original_data.parse_header();
        let deserialized = Vec::<u64>::deserialize_header(&serialized);
        assert_eq!(original_data, deserialized);
    }
}
