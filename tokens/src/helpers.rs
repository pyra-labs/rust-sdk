/// Compile-time validation and creation of Pyth feed IDs from hex strings.
/// Accepts either 64-character hex or 66-character hex with "0x" prefix.
#[macro_export]
macro_rules! feed_id {
    ($hex:expr) => {{
        const fn hex_char_to_byte(c: u8) -> u8 {
            match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                _ => panic!("Invalid hex character"),
            }
        }

        const fn parse_feed_id(hex: &str) -> FeedId {
            let hex_bytes = hex.as_bytes();
            let (start_offset, expected_len) =
                if hex_bytes.len() == 66 && hex_bytes[0] == b'0' && hex_bytes[1] == b'x' {
                    (2, 66)
                } else if hex_bytes.len() == 64 {
                    (0, 64)
                } else {
                    panic!("Feed ID must be 64 hex characters or 66 with 0x prefix")
                };

            if hex_bytes.len() != expected_len {
                panic!("Invalid hex string length");
            }

            let mut result = [0u8; 32];
            let mut i = 0;
            while i < 32 {
                let high = hex_char_to_byte(hex_bytes[start_offset + i * 2]);
                let low = hex_char_to_byte(hex_bytes[start_offset + i * 2 + 1]);
                result[i] = (high << 4) | low;
                i += 1;
            }
            result
        }

        const HEX: &str = $hex;
        const RESULT: FeedId = parse_feed_id(HEX);
        RESULT
    }};
}
