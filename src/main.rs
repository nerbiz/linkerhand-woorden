use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let timer = Instant::now();

    // Get the wordlist file
    // Source: https://github.com/OpenTaal/opentaal-wordlist
    let mut input_file: File = OpenOptions::new()
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
    // Indicates whether the current word is invalid
    let mut skip_word: bool = false;

    while input_file.read(&mut buffer)? > 0 {
        // End of line/word reached
        if buffer[0] == 10 {
            // Add the word to the output file if it's valid
            if ! skip_word {
                current_word.push(buffer[0]);
                output_file.write_all(&current_word)?;
            }

            current_word.clear();
            skip_word = false;
            continue;
        }

        // If the word is invalid, continue reading to end of line
        if skip_word {
            continue;
        }

        // The word is invalid if the byte value is outside boundaries,
        // or when using the byte value as bit-shift amount, the bit is not in the allowed bits.
        // To determine the bit-shift amount:
        //   Unset 5th bit (=32) to get uppercase code point,
        //   unset 6th bit (=64) to count from 1 to 26,
        //   then subtract 1 to make it a 0-based bit-shift amount
        if buffer[0] < 65
        || buffer[0] > 127
        || 1 << ((buffer[0] & !(32 + 64)) - 1) & allowed_bits == 0 {
            skip_word = true;
            continue;
        }

        // Character is valid, add it to the current word
        current_word.push(buffer[0]);
    }

    println!("Finished in {} ms", timer.elapsed().as_millis());

    Ok(())
}
