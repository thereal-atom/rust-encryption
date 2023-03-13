mod utils;

use utils::*;

fn ceaser_cipher(plain_text: String, shift: i8) -> String {
    let mut cipher_text = String::new();

    for c in plain_text.chars() {
        cipher_text.push(char_shift(c, shift));
    }

    cipher_text
}

fn main() {
    let plain_text = String::from("hi MOM");

    println!("plain text is: {}", plain_text);

    let cipher_text = ceaser_cipher(plain_text, 3);

    println!("shifted by 3 to make: {}", cipher_text);
}
