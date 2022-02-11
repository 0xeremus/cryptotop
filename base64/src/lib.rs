pub mod base64 {

    const UPPERCASEOFFSET: i8 = 65;
    const LOWERCASEOFFSET: i8 = 71;
    const DIGITOFFSET: i8 = -4;

    pub fn encode(input: Vec<u8>) -> String {
        let mut output: String = String::new();

        let _ = input
            .chunks(3)
            .for_each(|x| output.push_str(&String::from_utf8(encode_chunk(x)).unwrap()));

        // add padding
        let len = output.len() % 4;
        if len != 0 {
            for _ in 1..=len {
                output.push_str("=")
            }
        }

        output
    }

    fn encode_chunk(input: &[u8]) -> Vec<u8> {
        build_chunk(input).iter().map(|v| encode_index(v)).collect()
    }

    fn build_chunk(input: &[u8]) -> Vec<u8> {
        match input.len() {
            1 => return vec![&input[0] >> 2, (&input[0] & 0b00000011) << 4],
            2 => {
                return vec![
                    (&input[0] >> 2),
                    (&input[0] & 0b00000011) << 4 | (&input[1] >> 4),
                    (&input[1] & 0b00001111) << 2,
                ]
            }
            3 => {
                return vec![
                    (&input[0] >> 2),
                    (&input[0] & 0b00000011) << 4 | (&input[1] >> 4),
                    (&input[1] & 0b00001111) << 2 | (&input[2] & 0b11000000) >> 6,
                    (&input[2] & 0b00111111),
                ]
            }
            _ => unreachable!(),
        }
    }

    fn encode_index(input: &u8) -> u8 {
        // stolen from internet, not sure where. some base64 implementation blog.
        let ascii_index = match input {
            0..=25 => *input as i8 + UPPERCASEOFFSET,
            26..=51 => *input as i8 + LOWERCASEOFFSET,
            52..=61 => *input as i8 + DIGITOFFSET,
            62 => 43,
            63 => 47,

            _ => unreachable!(),
        } as u8;

        ascii_index
    }

    //pub fn decode(input: String) -> Result<Vec<u8>, Error> {}
}

#[cfg(test)]
mod tests {
    use crate::base64::encode;
    //assert_eq!("", encode(Vec::<u8>::from()));

    #[test]
    fn test_simple_encode() {
        assert_eq!("YWFhYWFh", encode(Vec::<u8>::from("aaaaaa")))
    }

    #[test]
    fn test_hello_encode() {
        assert_eq!(
            "SGVsbG8sIFdvcmxkIQ==",
            encode(Vec::<u8>::from("Hello, World!"))
        );
    }

    #[test]
    fn test_complex_encode() {
        assert_eq!(
            "WW91ciBpbmJveCBpcyBmdWxsIG9mIG1lc3NhZ2VzIGZyb20gZnJpZW5kcyBhbmQgZmFtaWx5LCBhcHBvaW50bWVudHMgZnJvbSBjb2xsZWFndWVzLCBhbmQgdXBkYXRlcyBmcm9tIG9yZ2FuaXphdGlvbnMgeW91IGNhcmUgYWJvdXQu",
            encode(Vec::<u8>::from("Your inbox is full of messages from friends and family, appointments from colleagues, and updates from organizations you care about."))
        );
    }

    #[test]
    fn test_random_encoding() {
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            encode(Vec::<u8>::from(
                "I'm killing your brain like a poisonous mushroom"
            ))
        );
    }
}
