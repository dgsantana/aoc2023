#[derive(Debug)]
pub struct Node {
    pub name: u32,
    pub left_string: u32,
    pub right_string: u32,
    pub node_ref: NodeRef,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeRef {
    pub index: usize,
    pub left: usize,
    pub right: usize,
}

impl Node {
    pub fn from_line(input: &str) -> Self {
        let mut parts = input.split(" = ");
        let name = encode_string(parts.next().unwrap().trim());
        let to_parse = parts.next().unwrap();
        let (left_string, right_string) = to_parse.split_once(", ").expect("Invalid input");
        let left_string = encode_string(&left_string[1..]);
        let right_string = encode_string(&right_string[..right_string.len() - 1]);

        Self {
            name,
            left_string,
            right_string,
            node_ref: NodeRef {
                index: 0,
                left: 0,
                right: 0,
            },
        }
    }

    pub fn name(&self) -> String {
        decode_string(self.name)
    }

    pub fn match_name(&self, other: u32) -> bool {
        match_code(self.name, other)
    }
}

/// Encode string as a number
///
/// "ABC" -> 010203
/// "AAA" -> 010101
/// "B" -> FFFF02
/// "A" -> FFFF01
pub fn encode_string(input: &str) -> u32 {
    let mut encoded_value: u32 = 0;
    let mut bytes = input
        .trim()
        .bytes()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c - b'A' + 1
            } else {
                c - b'0' + 30
            }
        })
        .collect::<Vec<_>>();
    if input.len() < 3 {
        for _ in 0..(3 - input.len()) {
            bytes.insert(0, 0xFF);
        }
    }

    for ascii_code in bytes.into_iter() {
        encoded_value |= ascii_code as u32;
        encoded_value <<= 8;
    }
    encoded_value >>= 8;
    encoded_value
}

/// Decode number as a string
///
/// 0x010203 -> "ABC"
/// 0x010101 -> "AAA"
/// 0xFFFF02 -> "B"
fn decode_string(input: u32) -> String {
    let mut decoded_chars = Vec::new();
    let mut raw_input = input;

    while raw_input > 0 {
        let offset = (raw_input & 0xFF) as u8;
        if offset != 0xFF {
            let ascii_code = if offset >= 30 {
                offset + b'0' - 30
            } else {
                offset + b'A' - 1
            };
            decoded_chars.push(ascii_code as char);
        }
        raw_input >>= 8;
    }

    decoded_chars.into_iter().rev().collect()
}

/// Match nodes codes with masking
///
/// 0x010101 & 0xFFFF09 -> false
/// 0x010101 & 0xFFFF01 -> true
/// 0x010101 & 0xFFFF00 -> false
/// 0x010909 & 0xFF0909 -> true
/// 0x010909 & 0xFF0900 -> false
/// 0x010909 & 0x010909 -> true
fn match_code(code: u32, other: u32) -> bool {
    let mut code = code;
    let mut other = other;
    while code > 0 {
        let code_offset = code & 0xFF;
        let other_offset = other & 0xFF;
        if code_offset != 0xFF && other_offset != 0xFF && code_offset != other_offset {
            return false;
        }
        code >>= 8;
        other >>= 8;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_string() {
        assert_eq!(encode_string("ABC"), 0x010203);
        assert_eq!(encode_string("AAA"), 0x010101);
        assert_eq!(encode_string("B"), 0xFFFF02);
        assert_eq!(encode_string("A"), 0xFFFF01);
    }

    #[test]
    fn test_decode_string() {
        assert_eq!(decode_string(0x010203), "ABC");
        assert_eq!(decode_string(0x010101), "AAA");
        assert_eq!(decode_string(0xFFFF02), "B");
        assert_eq!(decode_string(0xFFFF01), "A");
        assert_eq!(decode_string(0xFF0102), "AB");
    }

    #[test]
    fn test_match_code() {
        assert!(match_code(0x010101, 0xFFFF01));
        assert!(match_code(0x010909, 0xFF0909));
        assert!(match_code(0x010909, 0x010909));
        assert!(!match_code(0x010101, 0xFFFF02));
        assert!(!match_code(0x010101, 0xFFFF09));
        assert!(!match_code(0x010909, 0xFF0901));
    }
}
