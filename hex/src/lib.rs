pub mod hex {
    pub fn decode(input: &str) -> Vec<u8> {
        if (input.len() % 2 != 0) & valid_hex_character(&input) {
            panic!("Invalid Hex string, must be multiple of 2.");
        }

        input
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(hex_to_byte)
            .collect::<Vec<u8>>()
    }

    fn valid_hex_character(input: &str) -> bool {
        input
            .chars()
            .map(|x| match x as u8 {
                48..=57 => true,
                65..=90 => true,
                97..=122 => true,
                _ => false,
            })
            .fold(true, |x, y| x & y)
    }

    fn hex_to_byte(input: &[char]) -> u8 {
        match input.len() {
            2 => {
                u8::from_str_radix(&input[0].to_string(), 16).unwrap() << 4
                    | u8::from_str_radix(&input[1].to_string(), 16).unwrap()
            }
            _ => unreachable!(),
        }
    }

    pub fn encode(input: Vec<u8>) -> String {
        input
            .iter()
            .map(|x| byte_to_nibble(*x))
            .flatten()
            .map(nibble_to_hex)
            .collect::<String>()
    }

    fn byte_to_nibble(byte: u8) -> Vec<u8> {
        vec![byte >> 4, byte & 0b00001111]
    }

    fn nibble_to_hex(nibble: u8) -> char {
        match nibble {
            0..=9 => (nibble + 48) as char,          // index into ascii numbers
            10..=16 => (nibble + (97 - 10)) as char, // index into char - value in hex

            _ => {
                println!("Invalid nibble value, freaking out");
                unreachable!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::{decode, encode};

    #[test]
    fn test_hello_world_decode() {
        assert_eq!(
            Vec::<u8>::from("Hello, World!"),
            decode("48656c6c6f2c20576f726c6421")
        )
    }

    #[test]
    fn test_sentence_decode() {
        assert_eq!(Vec::<u8>::from("With the start of the new year, we can see a new trend delineating Linux software. Phosh with its newbord libadwaita, KDE's Kirigami and Maui Shell show that - like it or not - the era of convergent Linux applications has just started."),
            decode("5769746820746865207374617274206f6620746865206e657720796561722c2077652063616e207365652061206e6577207472656e642064656c696e656174696e67204c696e757820736f6674776172652e2050686f7368207769746820697473206e6577626f7264206c6962616477616974612c204b44452773204b69726967616d6920616e64204d617569205368656c6c2073686f772074686174202d206c696b65206974206f72206e6f74202d2074686520657261206f6620636f6e76657267656e74204c696e7578206170706c69636174696f6e7320686173206a75737420737461727465642e"));
    }

    #[test]
    fn test_unicode_decode() {
        assert_eq!(
            Vec::<u8>::from("Ã±tÃ«rnÃ¢tiÃ´nÃ liÅ¾Ã¦tiÃ¸n"),
            decode("c383c2b174c383c2ab726ec383c2a27469c383c2b46ec383206c69c385c2bec383c2a67469c383c2b86e")
        )
    }

    #[test]
    #[should_panic]
    fn test_invalid_hexstr_decode() {
        decode("this ain't valid");
    }

    #[test]
    fn test_hello_world_encode() {
        assert_eq!(
            "48656c6c6f2c20576f726c6421".to_string(),
            encode(Vec::<u8>::from("Hello, World!"))
        )
    }

    #[test]
    fn test_sentence_encode() {
        assert_eq!(
            "5769746820746865207374617274206f6620746865206e657720796561722c2077652063616e207365652061206e6577207472656e642064656c696e656174696e67204c696e757820736f6674776172652e2050686f7368207769746820697473206e6577626f7264206c6962616477616974612c204b44452773204b69726967616d6920616e64204d617569205368656c6c2073686f772074686174202d206c696b65206974206f72206e6f74202d2074686520657261206f6620636f6e76657267656e74204c696e7578206170706c69636174696f6e7320686173206a75737420737461727465642e".to_string(),
            encode(Vec::<u8>::from("With the start of the new year, we can see a new trend delineating Linux software. Phosh with its newbord libadwaita, KDE's Kirigami and Maui Shell show that - like it or not - the era of convergent Linux applications has just started.")));
    }

    #[test]
    fn test_unicode_encode() {
        assert_eq!(
            "c383c2b174c383c2ab726ec383c2a27469c383c2b46ec383206c69c385c2bec383c2a67469c383c2b86e"
                .to_string(),
            encode(Vec::<u8>::from("Ã±tÃ«rnÃ¢tiÃ´nÃ liÅ¾Ã¦tiÃ¸n"))
        )
    }
}
