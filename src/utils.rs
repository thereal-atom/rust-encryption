const ASCII_A: i8 = 'A' as i8; // 65
const ASCII_Z: i8 = 'Z' as i8; // 91
const ABC_SIZE: i8 = ASCII_Z - ASCII_A + 1;

pub fn char_shift(c: char, shift: i8) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    };

    if shift >= ABC_SIZE || shift <= -ABC_SIZE {
        panic!("invalid shift");
    };

    let input_upper = c.to_ascii_uppercase();

    let input_index = input_upper as i8 - ASCII_A;

    let output_index = (input_index + shift).rem_euclid(ABC_SIZE);

    let mut output_upper = ((output_index + ASCII_A) as u8) as char;

    if c.is_ascii_lowercase() {
        output_upper = output_upper.to_ascii_lowercase();
    };

    output_upper
}