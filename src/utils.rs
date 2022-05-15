pub fn percent_encode(param: &[u8]) -> String {
    let mut percent_encoded = String::with_capacity(50);
    for byte in param {
        match byte {
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'-' | b'_' | b'.' | b'~' => {
                percent_encoded.push(*byte as char)
            }
            reserved => percent_encoded.push_str(&("%".to_string() + &hex::encode([*reserved]))),
        }
    }
    percent_encoded
}

pub fn percent_decode(param: &[u8]) -> Vec<u8> {
    let mut percent_decoded = Vec::with_capacity(50);
    let mut cursor = 0;

    while let Some(param_char) = param.get(cursor) {
        cursor += 1;
        match param_char {
            b'%' => {
                let hex_val = hex::decode(
                    param
                        .get(cursor..cursor + 2)
                        .expect("Failed to get next 2 bytes"),
                )
                .expect("Failed to decode hex");
                cursor += 2;
                percent_decoded.extend(hex_val);
            }
            _ => percent_decoded.push(*param_char),
        }
    }

    percent_decoded
}

#[cfg(test)]
mod tests {
    use super::{percent_decode, percent_encode};

    #[test]
    fn test_percent_encode() {
        let bytes: [u8; 20] = [
            143, 185, 100, 238, 19, 194, 158, 219, 155, 135, 253, 196, 36, 54, 65, 35, 30, 231, 64,
            22,
        ];
        let expected_result = "%8f%b9d%ee%13%c2%9e%db%9b%87%fd%c4%246A%23%1e%e7%40%16";

        assert_eq!(expected_result, percent_encode(&bytes));
    }

    #[test]
    fn test_percent_decode() {
        let bytes = "%8f%b9d%ee%13%c2%9e%db%9b%87%fd%c4%246A%23%1e%e7%40%16".as_bytes();
        let expected_result: Vec<u8> = vec![
            143, 185, 100, 238, 19, 194, 158, 219, 155, 135, 253, 196, 36, 54, 65, 35, 30, 231, 64,
            22,
        ];

        assert_eq!(expected_result, percent_decode(&bytes));
    }
}
