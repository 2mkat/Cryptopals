#[derive(Debug)]
pub enum XorError {
    MismatchdLength,
}

pub fn fixed_xor(slice1: &[u8], slice2: &[u8]) -> Result<Vec<u8>, XorError> {
    if slice1.len() != slice2.len() {
        return Err(XorError::MismatchdLength);
    }

    let xor_res: Vec<u8> = slice1
                .iter()
                .zip(slice2.iter())
                .map(|(x, y)| x ^ y)
                .collect();
    
    Ok(xor_res)
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn set1_task2() {
        let slice1 = "1c0111001f010100061a024b53535009181c";
        let slice2 = "686974207468652062756c6c277320657965";

        let hex_slice1 = hex::decode(slice1).unwrap();
        let hex_slice2 = hex::decode(slice2).unwrap();

        let res_xor = fixed_xor(&hex_slice1, &hex_slice2).unwrap();
        assert_eq!(res_xor, hex::decode("746865206b696420646f6e277420706c6179").unwrap());
    }
}