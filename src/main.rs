use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let timer = Instant::now();

    // Get the wordlist file
    // Source: https://github.com/OpenTaal/opentaal-wordlist
    let mut input_file = OpenOptions::new()
        .read(true)
        .open("res/wordlist.txt")?;

    // Open the output file (create it if it doesn't exist)
    let mut output_file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("res/filtered.txt")?;

    // Represent allowed characters as 1-bits in a 32-byte int:
    // 00000010111011110000000001111111
    //       ZYXWVUTSRQPONMLKJIHGFEDCBA
    let allowed_bits: u32 = "QWERTASDFGZXCVB"
        .chars()
        .into_iter()
        .fold(0, |int, char| {
            // Place the characters as bits at positions 0-25,
            // -65 makes codepoints 0-based
            int | 1 << char as u8 - 65
        });

    // File read buffer
    let mut buffer: [u8; 1] = [0; 1];
    // The current word being read (as bytes)
    let mut current_word: Vec<u8> = vec!();
    let mut word_is_valid: bool = true;

    while input_file.read(&mut buffer)? > 0 {
        // End of line reached
        if buffer[0] == 10 {
            // Add the word to the output file
            if word_is_valid == true {
                current_word.push(buffer[0]);
                output_file.write_all(&current_word)?;
            }

            current_word.clear();
            word_is_valid = true;
            continue;
        }

        // Skip if the byte value is outside the boundaries
        if buffer[0] < 65 || buffer[0] > 127 {
            word_is_valid = false;
            continue;
        }

        // Unset 5th bit (subtract 32) to get uppercase code point,
        // unset 6th bit (subtract 64) to count from 1 to 26,
        // then subtract 1 to make it a 0-based bit-shift amount
        let shift_amount: u8 = (buffer[0] & !96) - 1;

        // Check if the character is allowed by comparing bits
        if 1 << shift_amount & allowed_bits == 0 {
            word_is_valid = false;
            continue;
        }

        if word_is_valid == false {
            continue;
        }

        word_is_valid = true;
        current_word.push(buffer[0]);
    }

    println!("Finished in {} ms", timer.elapsed().as_millis());

    Ok(())
}
