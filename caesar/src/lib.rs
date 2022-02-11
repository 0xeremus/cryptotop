pub mod caesar {

    pub fn brute_force(input: &str) -> Vec<String> {
        let mut collect: Vec<String> = Vec::with_capacity(26);

        for i in 0..26 {
            collect.push(rotate_string(input.to_string(), i));
        }

        collect
    }

    pub fn rotate_string(input: String, index: u8) -> String {
        if index > 0 {
            input
                .chars()
                .map(|x: char| {
                    if x.is_uppercase() {
                        (0x41 + (((x as u8 - 0x41) + index) % 26)) as char
                    } else if x.is_lowercase() {
                        (0x61 + (((x as u8 - 0x61) + index) % 26)) as char
                    } else {
                        x
                    }
                })
                .collect::<String>()
        } else {
            input
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::caesar::*;
    #[test]
    fn rotate_single_char() {
        let lower = ["a", "b", "c", "d", "e", "f", "g"];
        let upper = ["A", "B", "C", "D", "E", "F", "G"];

        for i in 0..7 as usize {
            assert_eq!(rotate_string("a".to_string(), i as u8), lower[i]);

            assert_eq!(rotate_string("A".to_string(), i as u8), upper[i]);
        }
    }
    #[test]
    fn rotate_simple_str() {
        assert_eq!(rotate_string("input".to_string(), 1), "joqvu".to_string());
        assert_eq!(rotate_string("OAK".to_string(), 24), "MYI".to_string());
        assert_eq!(rotate_string("MYI".to_string(), 2), "OAK".to_string())
    }
    #[test]
    fn rotate_string_with_space() {
        assert_eq!(
            "CAVE PRAISE ALSO OAK",
            rotate_string("AYTC NPYGQC YJQM MYI".to_string(), 2)
        );
        assert_eq!(
            "trip display solid blame".to_string(),
            rotate_string("IGXE SXHEAPN HDAXS QAPBT".to_ascii_lowercase(), 11)
        );
    }

    #[test]
    fn brute_force_letter() {
        let mut lower = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            "j".to_string(),
            "k".to_string(),
            "l".to_string(),
            "m".to_string(),
            "n".to_string(),
            "o".to_string(),
            "p".to_string(),
            "q".to_string(),
            "r".to_string(),
            "s".to_string(),
            "t".to_string(),
            "u".to_string(),
            "v".to_string(),
            "w".to_string(),
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
        ];

        assert_eq!(brute_force("a"), lower);

        lower.rotate_right(1);
        assert_eq!(brute_force("z"), lower);

        lower.rotate_left(13);
        assert_eq!(brute_force("m"), lower);
    }

    #[test]
    fn brute_force_phrase() {
        let outcome = vec![
            "IGXE SXHEAPN HDAXS QAPBT".to_string(),
            "JHYF TYIFBQO IEBYT RBQCU".to_string(),
            "KIZG UZJGCRP JFCZU SCRDV".to_string(),
            "LJAH VAKHDSQ KGDAV TDSEW".to_string(),
            "MKBI WBLIETR LHEBW UETFX".to_string(),
            "NLCJ XCMJFUS MIFCX VFUGY".to_string(),
            "OMDK YDNKGVT NJGDY WGVHZ".to_string(),
            "PNEL ZEOLHWU OKHEZ XHWIA".to_string(),
            "QOFM AFPMIXV PLIFA YIXJB".to_string(),
            "RPGN BGQNJYW QMJGB ZJYKC".to_string(),
            "SQHO CHROKZX RNKHC AKZLD".to_string(),
            "TRIP DISPLAY SOLID BLAME".to_string(),
            "USJQ EJTQMBZ TPMJE CMBNF".to_string(),
            "VTKR FKURNCA UQNKF DNCOG".to_string(),
            "WULS GLVSODB VROLG EODPH".to_string(),
            "XVMT HMWTPEC WSPMH FPEQI".to_string(),
            "YWNU INXUQFD XTQNI GQFRJ".to_string(),
            "ZXOV JOYVRGE YUROJ HRGSK".to_string(),
            "AYPW KPZWSHF ZVSPK ISHTL".to_string(),
            "BZQX LQAXTIG AWTQL JTIUM".to_string(),
            "CARY MRBYUJH BXURM KUJVN".to_string(),
            "DBSZ NSCZVKI CYVSN LVKWO".to_string(),
            "ECTA OTDAWLJ DZWTO MWLXP".to_string(),
            "FDUB PUEBXMK EAXUP NXMYQ".to_string(),
            "GEVC QVFCYNL FBYVQ OYNZR".to_string(),
            "HFWD RWGDZOM GCZWR PZOAS".to_string(),
        ];

        let test_set = brute_force("IGXE SXHEAPN HDAXS QAPBT");

        test_set
            .iter()
            .zip(outcome)
            .filter_map(|x| if *x.0 == x.1 { Some(true) } else { Some(false) })
            .collect::<Vec<bool>>()
            .iter()
            .for_each(|x| assert_eq!(*x, true));
    }
}
