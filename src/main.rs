use set1::hex_to_base64_01::convert_hex_to_base64;
use set1::fixed_xor_02::fixed_xor;
use set1::xor_cipher_03::single_xor_cipher;
use set1::detect_single_xor_04::detect_single_xor;

mod set1;

fn main() {
    //----------- Challenge 1 -----------//
    println!("Challenge 1  - Convert hex to base64\n");
    let hex_string = "49276d206b696c6c696e6720796f7\
                                        57220627261696e206c696b6\
                                        5206120706f69736f6e6f7573206d757368726f6f6d";

    println!("{}", convert_hex_to_base64(hex_string));

    //----------- Challenge 2 -----------//
    println!("\nChallenge 2 - Fixed XOR\n");
    let slice1 = "1c0111001f010100061a024b53535009181c";
    let slice2 = "686974207468652062756c6c277320657965";

    let hex_slice1 = hex::decode(slice1).unwrap();
    let hex_slice2 = hex::decode(slice2).unwrap();

    let res_xor = fixed_xor(&hex_slice1, &hex_slice2).unwrap();
    println!("{}", hex::encode(res_xor));

    //----------- Challenge 3 -----------//
    println!("\nChallenge 3 - Single-byte XOR cipher\n");
    let decode_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("{}", single_xor_cipher(&decode_string).0);  // Cooking MC's like a pound of bacon

    //----------- Challenge 4 -----------//
    println!("\nChallenge 4 - Detect single-character XOR\n");
    let file_path = "src/set1/4.txt";
    println!("{}", detect_single_xor(&file_path));

}
