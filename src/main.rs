use rand::Rng;

const ASCII_A: u8 = 'A' as u8; // 65
const ASCII_Z: u8 = 'Z' as u8; // 91
const ABC_SIZE: u8 = ASCII_Z - ASCII_A + 1;

// utils

fn char_shift(c: char, shift: u8) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    };

    if shift >= ABC_SIZE {
        panic!("invalid shift");
    };

    let input_upper = c.to_ascii_uppercase();

    let input_index = input_upper as u8 - ASCII_A;

    let output_index = (input_index + shift).rem_euclid(ABC_SIZE);

    let mut output_upper = ((output_index + ASCII_A) as u8) as char;

    if c.is_ascii_lowercase() {
        output_upper = output_upper.to_ascii_lowercase();
    };

    output_upper
}

fn generate_random_number(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..max)
}

fn calculate_matrix_vector_product(matrix: Vec<Vec<usize>>, vector: Vec<usize>) -> Vec<usize> {
    let mut output_vector = vec![];

    for i in 0..matrix.len() {
        let mut sum = 0;

        for j in 0..matrix[i].len() {
            sum += matrix[i][j] * vector[j]
        };

        output_vector.push(sum);
    };

    output_vector
}

fn generate_random_matrix(rows: usize, columns: usize, min: usize, max: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![];

    for _ in 0..rows {
        let mut row = vec![];

        for _ in 0..columns {
            row.push(generate_random_number(min, max));
        };

        matrix.push(row);
    };

    matrix
}

// ciphers

fn ceasar_cipher(plain_text: String, shift: u8) -> String {
    let mut cipher_text = String::new();

    for c in plain_text.chars() {
        cipher_text.push(char_shift(c, shift));
    }

    cipher_text
}

fn affine_cipher(plain_text: String, a: u8, b: u8) -> String {
    let mut cipher_text = String::new();

    for c in plain_text.chars() {
        let x = (c.to_ascii_uppercase() as u8) - ASCII_A as u8;

        let output_index = (a * x + b) % ABC_SIZE as u8;

        let output = ((output_index + ASCII_A as u8) as u8) as char;

        cipher_text.push(output);
    }

    cipher_text
}

// https://pages.mtu.edu/~shene/NSF-4/Tutorial/VIG/Vig-Base.html

fn vigenere_cipher(plain_text: String, keyword: String) -> String {
    let mut cipher_text = String::new();

    for (i, c) in plain_text
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .into_iter()
        .enumerate() 
    {
        if i > 0 && i % 5 == 0 { cipher_text.push(' ') };
        
        if c.is_alphabetic() {
            let key = keyword.as_bytes()[i % keyword.len()];

            let ascii_char = c as u8 - ASCII_A;
            let ascii_key = key as u8 - ASCII_A;

            let ascii_output = (ascii_char + ascii_key) % ABC_SIZE;
            let output = ((ascii_output + ASCII_A) as u8) as char;

            cipher_text.push(output);
        }
    }

    cipher_text
}

fn hill_cipher(plain_text: String, key: Vec<Vec<usize>>) -> String {
    let mut cipher_text = String::new();

    let mut plain_text_vector = vec![];

    for c in plain_text.chars() {
        if c.is_alphabetic() {
            plain_text_vector.push((c.to_ascii_uppercase() as u8 - ASCII_A) as usize);
        }
    }

    let cipher_text_vector = calculate_matrix_vector_product(key, plain_text_vector);

    for i in cipher_text_vector {
        cipher_text.push(((i % 26) as u8 + ASCII_A) as char);
    }

    cipher_text
}

// asymmetric encryption
// user 1 generates a public and a private key pair
// the keys are mathematically linked
// user 2 generates a key pair
// user 1 gets user 2's public key and vice versa
// user 1 encrypts a message using user 2's public key
// only user 2 can decrypt the message using user 2's private key

// md5 hashing: https://rosettacode.org/wiki/MD5/Implementation

fn main() {
    // let plain_text = String::from("hi MOM");

    // println!("plain text is: {}", plain_text);

    // let cipher_text = ceaser_cipher(plain_text, 3);

    // println!("shifted by 3 to make: {}", cipher_text);

    // let o = affine_cipher(String::from("ABC"), 1, 2);

    // println!("{}", o);


    // let plain_text = String::from("MICHIGAN TECHNOLOGICAL UNIVERSITY");

    // let o = vigenere_cipher(plain_text.clone(), String::from("HOUGHTON"));

    // println!("{}\n{}", plain_text, o);

    // let m = generate_random_matrix(3, 3, 0, 26);

    // println!("{:?}", m);

    // let plain_text = String::from("CAT");

    // let o = hill_cipher(plain_text.clone(), vec![
    //     vec![6, 24, 1],
    //     vec![13, 16, 10],
    //     vec![20, 17, 15],
    // ]);

    // println!("{}\n{}", plain_text, o);
}
