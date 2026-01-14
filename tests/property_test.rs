//! Property-based tests for decodeck

use proptest::prelude::*;

// T087: Property-based tests for Base64 decoding
mod decode_properties {
    use super::*;
    use base64::{engine::general_purpose::STANDARD, Engine};
    use decodeck::decoder::EncodedData;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn roundtrip_any_bytes(data in prop::collection::vec(any::<u8>(), 0..1000)) {
            // Encode with standard base64
            let encoded = STANDARD.encode(&data);

            // Decode with decodeck
            let parsed = EncodedData::parse(&encoded).unwrap();
            let decoded = parsed.decode().unwrap();

            // Should match original
            prop_assert_eq!(decoded, data);
        }

        #[test]
        fn handles_whitespace_gracefully(
            data in prop::collection::vec(any::<u8>(), 1..100),
            whitespace in prop::collection::vec(prop::sample::select(vec![' ', '\n', '\t', '\r']), 0..10)
        ) {
            let encoded = STANDARD.encode(&data);

            // Insert whitespace at random positions
            let mut chars: Vec<char> = encoded.chars().collect();
            for (i, ws) in whitespace.iter().enumerate() {
                let pos = i % (chars.len() + 1);
                chars.insert(pos, *ws);
            }
            let with_whitespace: String = chars.into_iter().collect();

            // Should still decode correctly
            let parsed = EncodedData::parse(&with_whitespace).unwrap();
            let decoded = parsed.decode().unwrap();
            prop_assert_eq!(decoded, data);
        }

        #[test]
        fn detects_invalid_characters(
            valid_base64 in "[A-Za-z0-9+/]{4,100}",
            invalid_char in prop::sample::select(vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')'])
        ) {
            // Insert invalid character
            let mut invalid = valid_base64.clone();
            if !invalid.is_empty() {
                invalid.insert(invalid.len() / 2, invalid_char);
            }

            // Should return error
            let result = EncodedData::parse(&invalid);
            prop_assert!(result.is_err());
        }

        #[test]
        fn never_panics_on_arbitrary_input(input in ".*") {
            // Should never panic, just return Ok or Err
            let _ = EncodedData::parse(&input);
        }
    }
}

mod urlsafe_properties {
    use super::*;
    use base64::{engine::general_purpose::URL_SAFE, Engine};
    use decodeck::decoder::EncodedData;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn roundtrip_urlsafe_bytes(data in prop::collection::vec(any::<u8>(), 1..500)) {
            let encoded = URL_SAFE.encode(&data);
            let parsed = EncodedData::parse(&encoded).unwrap();
            let decoded = parsed.decode().unwrap();
            prop_assert_eq!(decoded, data);
        }
    }
}
