// Declare the modules we created
pub mod core;
pub mod error;
pub mod loader;
pub mod model;
pub mod registry;

// Re-export the key components for easy access
pub use core::CoreBPE;
pub use error::{DecodeError, DecodeKeyError};
pub use model::encoding_name_for_model;
pub use registry::get_encoding;

// Define the Rank type, which is used throughout the library
pub type Rank = u32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_encoding() {
        let enc = get_encoding("cl100k_base");
        assert!(enc.is_some());
    }

    #[test]
    fn test_encoding_for_model() {
        let model_name = "gpt-4o";
        let encoding_name = encoding_name_for_model(model_name).unwrap();
        assert_eq!(encoding_name, "o200k_base");

        let enc = get_encoding(encoding_name);
        assert!(enc.is_some());
    }

    #[test]
    fn test_roundtrip() {
        let enc = get_encoding("cl100k_base").unwrap();
        let text = "Hello world! <|endoftext|>";
        let encoded = enc.encode_with_special_tokens(text);
        let decoded = enc.decode_bytes(&encoded).unwrap();
        assert_eq!(text.as_bytes(), decoded.as_slice());
    }

    #[test]
    fn test_count_tokens() {
        let enc = get_encoding("o200k_base").unwrap();
        let text = "hello world";
        assert_eq!(enc.count_tokens(text), 2);

        let text_with_special = "hello world<|endoftext|>";
        assert_eq!(enc.count_tokens(text_with_special), 3);
    }
}
