use std::collections::HashMap;
use crate::set1::fixed_xor_02::fixed_xor;

static FREQUENCIES: [(u8, f32); 28] = [
    (b' ', 12.17), 
    (b'.', 6.57),
    (b'a', 8.12),
    (b'b', 1.49),
    (b'c', 2.71),
    (b'd', 4.32),
    (b'e', 12.02),
    (b'f', 2.30),
    (b'g', 2.03),
    (b'h', 5.92),
    (b'i', 7.31),
    (b'j', 0.10),
    (b'k', 0.69),
    (b'l', 3.98),
    (b'm', 2.61),
    (b'n', 6.95),
    (b'o', 7.68),
    (b'p', 1.82),
    (b'q', 0.11),
    (b'r', 6.02),
    (b's', 6.28),
    (b't', 9.10),
    (b'u', 2.88),
    (b'v', 1.11),
    (b'w', 2.09),
    (b'x', 0.17),
    (b'y', 2.11),
    (b'z', 0.07),
];

fn is_alphabetic(u: u8) -> bool {
    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A) || (u == 0x20)
}

fn compute_score(vec_u8: &[u8]) -> f32 {
    let score: f32 = vec_u8
                    .iter()
                    .filter(|&c| is_alphabetic(*c))
                    .map(|&c| FREQUENCIES
                                .iter()
                                .find(|&(a, _)| c.to_ascii_lowercase() == *a)
                                .map(|&(_, n)| n)
                                .unwrap_or(0.0))
                    .sum();
    score 
}

pub fn single_xor_cipher(hex_str: &str) -> (String, f32, u8) {
    // hex string to raw bytes
    let hex_bytes = hex::decode(hex_str).unwrap();

    let mut bytes_and_scores = HashMap::new();

    for i in 0..=255u8 {
        let key = vec![i; hex_bytes.len()];
        let res = fixed_xor(&key, &hex_bytes).unwrap();
        let score = compute_score(&res);
        bytes_and_scores.insert(i, score);
    }

    let (mut score, mut key): (f32, u8) = (0., 8u8);

    for (i, n) in bytes_and_scores {
        if score < n {
            score = n;
            key = i;
        }
    }

    let res = fixed_xor(&vec![key; hex_bytes.len()], &hex_bytes).unwrap();
    (String::from_utf8(res).unwrap(), score, key)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn set1_task3() {
        let decode_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        assert_eq!("Cooking MC's like a pound of bacon", single_xor_cipher(&decode_string).0);
    }
}