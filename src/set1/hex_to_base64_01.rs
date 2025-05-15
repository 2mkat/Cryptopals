use base64::{engine::general_purpose, prelude::*};

pub fn convert_hex_to_base64(hex_string: &str) -> String {

    // hex to raw bytes
    let byte_string = match  hex::decode(hex_string) {
        Ok(vec_values) => vec_values,
        Err(error) => panic!("Problem to convert hex: {error:?}"),
    };
    // let byte_string = hex::decode(hex_string).unwrap();

    // raw bytes to base64
    let base64_string = general_purpose::STANDARD.encode(byte_string);

    base64_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set1_task1() {
        let hex_string = "49276d206b696c6c696e6720796f7\
                                        57220627261696e206c696b6\
                                        5206120706f69736f6e6f7573206d757368726f6f6d";

        assert_eq!(convert_hex_to_base64(hex_string), String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
    }
}
