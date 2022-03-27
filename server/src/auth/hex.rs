//! Module for encoding and decoding hex.

/// Encodes a hex string (lowercase).
pub fn encode(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len());

    for b in bytes.iter().copied() {
        for shift in (0..2).rev() {
            let nibble = (b >> (4 * shift)) & 0xf;
            let hex_offset = match nibble {
                0..=9 => b'0',
                _ => b'a' - 10,
            };

            hex.push((nibble + hex_offset) as char);
        }
    }

    hex
}

/// Decodes a hex string, ignoring any invalid characters.
pub fn decode(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(hex.len() / 2);

    let mut byte = 0;
    let mut i = 0;

    for &ch in hex.as_bytes() {
        let value = match ch {
            b'a'..=b'f' => ch - b'a' + 10,
            b'A'..=b'F' => ch - b'A' + 10,
            b'0'..=b'9' => ch - b'0',
            _ => continue,
        };

        i += 1;
        byte = (byte << 4) | value;

        if i % 2 == 0 {
            bytes.push(byte);
        }
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn empty() {
        assert!(encode(&[]).is_empty());
        assert!(decode("").is_empty());
    }

    #[test]
    fn single_char() {
        assert!(decode("a").is_empty());
        assert!(decode("?").is_empty());
    }

    #[test]
    fn bad_input() {
        assert_eq!(decode("a??b??c??dd").len(), 2);
    }

    #[test]
    fn encode_and_decode() {
        let arr = [0, 1, 10, 50, 100, 200, 255];
        let encoded = encode(&arr);

        assert_eq!(encoded, "00010a3264c8ff");
        assert_eq!(decode(&encoded), arr);
    }
}
