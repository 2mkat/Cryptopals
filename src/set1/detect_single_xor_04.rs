use std::{collections::HashMap, fs};

use crate::set1::xor_cipher_03::single_xor_cipher;

pub fn detect_single_xor(file_path: &str) -> String{
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let array_contents: Vec<_> = contents.split('\r').map(|c| c.trim()).collect();
    let mut table = HashMap::new();

    for i in array_contents {
        table.insert(single_xor_cipher(i).0, single_xor_cipher(i).1);
    }

    let res= table.iter().max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
    println!("{}", res.0);
    
    String::new()
}