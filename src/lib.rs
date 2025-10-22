//! This library provides simple file encryption and decryption using XOR logic.
//! It allows you to encode or decode any file with a defined password,
//! producing `.rxor` encrypted files.
//!
//! # Example: Encoding a file
//! ```no_run
//! use rexor::encode;
//!
//! fn main() -> std::io::Result<()> {
//!     let input = "example.txt";
//!     let password = "password123";
//!
//!     // Encode the file into "example.txt.rxor"
//!     let output = encode(input, password, None)?;
//!     println!("Encoded file saved at: {}", output);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Example: Decoding a file
//! ```no_run
//! use rexor::decode;
//!
//! fn main() -> std::io::Result<()> {
//!     let input = "example.txt.rxor";
//!     let password = "password123";
//!
//!     // Decode the file back to its original form
//!     let output = decode(input, password, None)?;
//!     println!("Decoded file saved at: {}", output);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Example: Custom output path
//! ```no_run
//! use rexor::{encode, decode};
//!
//! fn main() -> std::io::Result<()> {
//!     let input = "example.txt";
//!     let password = "password123";
//!
//!     // Encode to a custom location
//!     let encoded = encode(input, password, Some("encrypted/output.rxor"))?;
//!
//!     // Decode back into another file
//!     let decoded = decode(&encoded, password, Some("decrypted/example.txt"))?;
//!
//!     println!("Encoded: {}", encoded);
//!     println!("Decoded: {}", decoded);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Errors
//! The functions return [`std::io::Result<String>`].
//! Typical errors include:
//! - Invalid input paths or missing files
//! - Empty password ([`std::io::ErrorKind::InvalidInput`])
//! - I/O issues while reading or writing files

use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Write};

/// XOR's all bytes from the input path against the provided password then writes
/// the result to the output path by proceeding in chunks of 8K.
fn xor_process(input_path: &str, output_path: &str, password: &str) -> Result<()> {
    // Opens input + output
    let mut input_file = BufReader::new(File::open(input_path)?);
    let mut output_file = BufWriter::new(File::create(output_path)?);

    // Converts the password into bytes
    let password_bytes = password.as_bytes();
    if password_bytes.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "The password cannot be empty",
        ));
    }

    // Chunk buffer
    let mut buffer = [0u8; 8192]; // 8K chunks
    let mut offset = 0;

    loop {
        // Reads bytes up to 8K from the input file into the buffer, returns the number of bytes read
        let bytes_read = input_file.read(&mut buffer)?;

        // Stops the loop when no more bytes can be read from the file
        if bytes_read == 0 {
            break;
        }

        // XOR's each byte with the password
        for i in 0..bytes_read {
            buffer[i] ^= password_bytes[(offset + i) % password_bytes.len()];
        }

        // Writes chunk to the output file
        output_file.write_all(&buffer[..bytes_read])?;
        offset += bytes_read;
    }

    Ok(())
}

/// Encodes a file using XOR with the provided password and input path.
/// Returns the output path of the encoded file.
pub fn encode(input_path: &str, password: &str, output_path: Option<&str>) -> Result<String> {
    let output = match output_path {
        Some(path) => path.to_string(),
        None => format!("{}.rxor", input_path),
    };
    xor_process(input_path, &output, password)?;
    Ok(output)
}

/// Decodes a previously XOR-encoded file using the provided password.
/// Returns the output path of the decoded file.
pub fn decode(input_path: &str, password: &str, output_path: Option<&str>) -> Result<String> {
    let output = match output_path {
        Some(path) => path.to_string(),
        None => {
            if input_path.ends_with(".rxor") {
                input_path.strip_suffix(".rxor").unwrap().to_string()
            } else {
                input_path.to_string()
            }
        }
    };
    xor_process(input_path, &output, password)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{read, remove_file, write};

    #[test]
    fn test_encode_decode() {
        let input = "test.txt";
        let password = "password123";
        let original_content = b"Test ReXOR";

        write(input, original_content).unwrap();

        let encoded = encode(input, password, None).unwrap();
        let decoded = decode(&encoded, password, None).unwrap();

        let result = read(decoded).unwrap();
        assert_eq!(result, original_content);

        // Cleanup
        remove_file(input).unwrap();
        remove_file(encoded).unwrap();
    }
}
