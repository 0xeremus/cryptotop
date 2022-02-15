mod hex {
    pub fn decode(input: &str) -> Vec<u8> {
        for i in (0..input.len()).step_by(2) {
            println!("pair = {},{}", input[i as usize], input[i as usize + 1]);
        }
        Vec::new()
    }

    // pub fn encode(_input: Vec<u8>) -> String {
    //     "".to_string()
    // }
}

#[cfg(test)]
mod tests {
    use crate::hex::decode;

    #[test]
    fn test_encode() {
        assert_eq!(
            Vec::<u8>::from("Hello, World"),
            decode("48656c6c6f2c20576f726c6421")
        )
    }
}
