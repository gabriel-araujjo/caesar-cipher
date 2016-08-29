use std::string::String;
use std::vec::Vec;

use unicode_normalization::UnicodeNormalization;

const ALPHABET_LENGTH: u8 = 26;

pub fn encrypt(message: &String, key: u8) -> String {
    let mut result: Vec<char> = Vec::new();
    for grapheme in message.nfd() {
        if grapheme.is_alphabetic() {
            let origin = if grapheme.is_uppercase() { b'A'} else { b'a'};
            result.push((origin + (grapheme as u8 - origin + key) % ALPHABET_LENGTH) as char);
        } else {
            result.push(grapheme);
        }
    }
    result.iter().map(|c| *c).collect::<String>()
}

pub fn decrypt(message: &String, key: u8) -> String{
    encrypt(message, ALPHABET_LENGTH - key)
}