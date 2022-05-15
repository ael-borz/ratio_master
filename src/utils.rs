pub fn url_encode(param: &[u8]) -> String {
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

#[cfg(test)]
mod tests {
    use super::url_encode;

    #[test]
    fn test_url_encode() {
        let bytes: [u8; 20] = [
            143, 185, 100, 238, 19, 194, 158, 219, 155, 135, 253, 196, 36, 54, 65, 35, 30, 231, 64,
            22,
        ];
        let expected_result = "%8f%b9d%ee%13%c2%9e%db%9b%87%fd%c4%246A%23%1e%e7%40%16";

        assert_eq!(expected_result, url_encode(&bytes));
    }
}
