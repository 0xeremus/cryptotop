pub mod base64 {

    use core::panic;

    // These variables govern the alphabet that is being encoded or decoded.
    const UPPERCASEOFFSET: i8 = 65;
    const LOWERCASEOFFSET: i8 = 71;
    const DIGITOFFSET: i8 = -4;
    const PADDING: &str = "=";

    pub fn encode(input: Vec<u8>) -> String {
        let mut output: String = String::new();

        let _ = input
            .chunks(3)
            .for_each(|x| output.push_str(&String::from_utf8(encode_chunk(x)).unwrap()));

        // add padding
        let len = output.len() % 4;
        if len != 0 {
            for _ in 1..=len {
                output.push_str(PADDING)
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
        // We know that 2^6 can't exceed these values so will get a mapping

        // This function finds the relative offset into the b64 alphabet and
        // returns the index.
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

    pub fn decode(input: &str) -> Vec<u8> {
        // handle padding string
        let input = if input.contains('=') {
            input.split('=').take(1).next().unwrap()
        } else {
            input
        };

        if validate_input(input) {
            Vec::<u8>::from(input)
                .chunks(4)
                .map(|x| decode_chunk(x))
                .flatten()
                .filter(|x| *x > 0_u8)
                .collect()
        } else {
            panic!("Invalid base64 string to decode.");
        }
    }

    fn validate_input(input: &str) -> bool {
        let res = input.len() % 4 != 0;

        res && input
            .chars()
            .map(|x| in_alphabet_domain(x))
            .fold(res, |f, x| f && x)
    }

    fn in_alphabet_domain(candidate: char) -> bool {
        match candidate as u8 {
            // base64 characters
            65..=90 => true,
            97..=122 => true,
            48..=57 => true,
            43 => true,
            47 => true,

            // non-base64 characters
            _ => false,
        }
    }

    fn decode_chunk(input_pre: &[u8]) -> Vec<u8> {
        let input = input_pre
            .iter()
            .map(|x| decode_index(*x))
            .collect::<Vec<u8>>();

        match input_pre.len() {
            2 => vec![
                (input[0] & 0b00111111) << 2 | input[1] >> 4,
                (input[1] & 0b00001111) << 4,
            ],
            3 => vec![
                (input[0] & 0b00111111) << 2 | input[1] >> 4,
                (input[1] & 0b00001111) << 4 | input[2] >> 2,
                (input[2] & 0b00000011) << 6,
            ],
            4 => vec![
                (input[0] & 0b00111111) << 2 | (input[1] >> 4),
                (input[1] & 0b00001111) << 4 | input[2] >> 2,
                (input[2] & 0b00000011) << 6 | input[3],
            ],

            // We know from how we are parsing chunks in that we are limited to 1-4
            _ => unreachable!(),
        }
    }

    fn decode_index(input: u8) -> u8 {
        // Finds the relative offset into the b64 alphabet and corrects for offset.
        match input {
            65..=90 => (input as i8 - UPPERCASEOFFSET) as u8,
            97..=122 => (input as i8 - LOWERCASEOFFSET) as u8,
            48..=57 => (input as i8 - DIGITOFFSET) as u8,
            43 => 62,
            47 => 63,

            _ => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod base64_tests {
    use crate::base64::{decode, encode};
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

    #[test]
    fn test_hello_decode() {
        assert_eq!(
            "Hello, World!".to_string().into_bytes(),
            decode("SGVsbG8sIFdvcmxkIQ==")
        );
    }

    #[test]
    fn test_character_decode() {
        assert_eq!("a".to_string().into_bytes(), decode("YQ=="));
    }

    #[test]
    fn test_nonalpha_character_decode() {
        assert_eq!("abcdefghijklmnopqrstuvwxyz123456789!#@$%^*&() 'ABCDEFGHIJKLMNOPQRSTUVYXYZ".to_string().into_bytes(), decode("YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXoxMjM0NTY3ODkhI0AkJV4qJigpICdBQkNERUZHSElKS0xNTk9QUVJTVFVWWVhZWg=="));
    }

    #[test]
    fn test_passage_decode() {
        assert_eq!("In our previous post, we announced that Android now supports the Rust programming language for developing the OS itself. Related to this, we are also participating in the effort to evaluate the use of Rust as a supported language for developing the Linux kernel. In this post, we discuss some technical aspects of this work using a few simple examples.".to_string().into_bytes(), decode("SW4gb3VyIHByZXZpb3VzIHBvc3QsIHdlIGFubm91bmNlZCB0aGF0IEFuZHJvaWQgbm93IHN1cHBvcnRzIHRoZSBSdXN0IHByb2dyYW1taW5nIGxhbmd1YWdlIGZvciBkZXZlbG9waW5nIHRoZSBPUyBpdHNlbGYuIFJlbGF0ZWQgdG8gdGhpcywgd2UgYXJlIGFsc28gcGFydGljaXBhdGluZyBpbiB0aGUgZWZmb3J0IHRvIGV2YWx1YXRlIHRoZSB1c2Ugb2YgUnVzdCBhcyBhIHN1cHBvcnRlZCBsYW5ndWFnZSBmb3IgZGV2ZWxvcGluZyB0aGUgTGludXgga2VybmVsLiBJbiB0aGlzIHBvc3QsIHdlIGRpc2N1c3Mgc29tZSB0ZWNobmljYWwgYXNwZWN0cyBvZiB0aGlzIHdvcmsgdXNpbmcgYSBmZXcgc2ltcGxlIGV4YW1wbGVzLg=="))
    }

    #[test]
    fn test_encode_decode_symmetry() {
        assert_eq!(
            decode(
                &encode(Vec::<u8>::from("In our previous post, we announced that Android now supports the Rust programming language for developing the OS itself. Related to this, we are also participating in the effort to evaluate the use of Rust as a supported language for developing the Linux kernel. In this post, we discuss some technical aspects of this work using a few simple examples."))),
            "In our previous post, we announced that Android now supports the Rust programming language for developing the OS itself. Related to this, we are also participating in the effort to evaluate the use of Rust as a supported language for developing the Linux kernel. In this post, we discuss some technical aspects of this work using a few simple examples.".to_string().into_bytes()
        );
    }

    #[test]
    fn test_decode_encode_symmetry() {
        assert_eq!(
            encode(
                decode("SW4gb3VyIHByZXZpb3VzIHBvc3QsIHdlIGFubm91bmNlZCB0aGF0IEFuZHJvaWQgbm93IHN1cHBvcnRzIHRoZSBSdXN0IHByb2dyYW1taW5nIGxhbmd1YWdlIGZvciBkZXZlbG9waW5nIHRoZSBPUyBpdHNlbGYuIFJlbGF0ZWQgdG8gdGhpcywgd2UgYXJlIGFsc28gcGFydGljaXBhdGluZyBpbiB0aGUgZWZmb3J0IHRvIGV2YWx1YXRlIHRoZSB1c2Ugb2YgUnVzdCBhcyBhIHN1cHBvcnRlZCBsYW5ndWFnZSBmb3IgZGV2ZWxvcGluZyB0aGUgTGludXgga2VybmVsLiBJbiB0aGlzIHBvc3QsIHdlIGRpc2N1c3Mgc29tZSB0ZWNobmljYWwgYXNwZWN0cyBvZiB0aGlzIHdvcmsgdXNpbmcgYSBmZXcgc2ltcGxlIGV4YW1wbGVzLg==")),
                "SW4gb3VyIHByZXZpb3VzIHBvc3QsIHdlIGFubm91bmNlZCB0aGF0IEFuZHJvaWQgbm93IHN1cHBvcnRzIHRoZSBSdXN0IHByb2dyYW1taW5nIGxhbmd1YWdlIGZvciBkZXZlbG9waW5nIHRoZSBPUyBpdHNlbGYuIFJlbGF0ZWQgdG8gdGhpcywgd2UgYXJlIGFsc28gcGFydGljaXBhdGluZyBpbiB0aGUgZWZmb3J0IHRvIGV2YWx1YXRlIHRoZSB1c2Ugb2YgUnVzdCBhcyBhIHN1cHBvcnRlZCBsYW5ndWFnZSBmb3IgZGV2ZWxvcGluZyB0aGUgTGludXgga2VybmVsLiBJbiB0aGlzIHBvc3QsIHdlIGRpc2N1c3Mgc29tZSB0ZWNobmljYWwgYXNwZWN0cyBvZiB0aGlzIHdvcmsgdXNpbmcgYSBmZXcgc2ltcGxlIGV4YW1wbGVzLg=="
        );
    }
}
