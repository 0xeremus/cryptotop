pub mod frequency_analysis {
    use std::collections::HashMap;

    pub fn score_strings(candidates: Vec<String>) -> Vec<(String, f64)> {
        // Assign a score to each string in Vec
        let mut temp = candidates
            .iter()
            .map(move |c| (c.to_string(), score_text(&c)))
            .collect::<Vec<(String, f64)>>();

        // Order by highest score first
        temp.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // return scored strings
        temp
    }

    // Implements an englishness test where a score of 1 is english and 0 is not english.
    // based off stackoverflow article it implements the 'Bhattacharyya Coefficient' and links to the below github
    // https://crypto.stackexchange.com/questions/30209/developing-algorithm-for-detecting-plain-text-via-frequency-analysis
    pub fn score_text(candidate: &str) -> f64 {
        // frequencey of the english language
        let letter_freq = HashMap::from([
            ('a', 0.08167),
            ('b', 0.01492),
            ('c', 0.02782),
            ('d', 0.04253),
            ('e', 0.1270),
            ('f', 0.02228),
            ('g', 0.02015),
            ('h', 0.06094),
            ('i', 0.06966),
            ('j', 0.00153),
            ('k', 0.00772),
            ('l', 0.04025),
            ('m', 0.02406),
            ('n', 0.06749),
            ('o', 0.07507),
            ('p', 0.01929),
            ('q', 0.00095),
            ('r', 0.05987),
            ('s', 0.06327),
            ('t', 0.09056),
            ('u', 0.02758),
            ('v', 0.00978),
            ('w', 0.02360),
            ('x', 0.00150),
            ('y', 0.01974),
            ('z', 0.00074),
        ]);

        // how many times each letter is in candidate
        let population: HashMap<char, f64> = candidate
            .chars()
            .map(|c| (c.to_ascii_lowercase(), candidate.matches(c).count() as f64))
            .collect();

        //calculate 'Bhattacharyya Coefficient'
        population
            .iter()
            .map(|(x, y)| match letter_freq.get(x) {
                Some(n) => ((n * *y) / (candidate.len() as f64)).sqrt(),
                None => 0.0,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::frequency_analysis::{score_strings, score_text};
    use std::io::Read;

    #[test]
    fn test_score_single_string() {
        let outcome = score_strings(vec!["candidates".to_string()]);

        assert!(outcome.len() == 1);
        assert!(outcome[0].0 == "candidates".to_string());
        assert!((outcome[0].1 - 0.7257) < 0.0001);
    }

    #[test]
    fn test_score_text_not_english() {
        assert_eq!(score_text("?\'\")(); "), 0_f64)
    }

    #[test]
    fn test_score_text_english() {
        assert!(score_text("In recent years we have witnessed an explosion of Internet-connected applications. Whether it is a new mobile app to find your soulmate, the latest wearable to monitor your vitals, or an industrial solution to detect corrosion, our life is becoming packed with connected systems.") > 0.88)
    }

    #[test]
    fn test_uppercase_equality() {
        assert!(score_text(&"In recent years we have witnessed an explosion of Internet-connected applications. Whether it is a new mobile app to find your soulmate, the latest wearable to monitor your vitals, or an industrial solution to detect corrosion, our life is becoming packed with connected systems.".to_ascii_lowercase()) - score_text(&"In recent years we have witnessed an explosion of Internet-connected applications. Whether it is a new mobile app to find your soulmate, the latest wearable to monitor your vitals, or an industrial solution to detect corrosion, our life is becoming packed with connected systems.".to_ascii_uppercase()) < 0.00001);
    }

    #[test]
    fn test_a_book() {
        use std::fs::File;

        let mut test_str = String::new();

        let _ = File::open("pride_prejudice.txt")
            .expect("failed to open ./pride_prejudice.txt")
            .read_to_string(&mut test_str);

        // this is a whole book, should be pretty close to 1
        assert!(score_text(&test_str) > 0.80);
    }

    #[test]
    fn test_caesar_cipher_selection() {
        use caesar::caesar::brute_force;

        let out = brute_force("IGXE SXHEAPN HDAXS QAPBT");

        let scored = score_strings(out);

        assert_eq!("TRIP DISPLAY SOLID BLAME", scored[0].0);
    }
}
