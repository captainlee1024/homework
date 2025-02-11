use anyhow::{Ok, Result};
use hex;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct PowResult {
    pub user_name: String,
    pub nonce: u64,
    pub hash_input: String,
    pub hash: String,
}

pub fn pow(user_name: String, difficulty: String) -> Result<PowResult> {
    let mut nonce = 0u64;

    loop {
        let mut hasher = Sha256::new();

        let input_str = user_name.clone() + &nonce.to_string();
        hasher.update(input_str.as_bytes());

        let result = hasher.finalize();
        let hash = hex::encode(result);

        if hash.starts_with(&difficulty) {
            return Ok(PowResult {
                user_name,
                nonce,
                hash_input: hex::encode(input_str.as_bytes()),
                hash: hex::encode(result),
            });
        }
        nonce += 1;
    }
}

mod tests {
    use super::*;
    use std::time;

    // cargo test test_pow_difficulty4 -- --nocapture
    // output:
    // PowResult {
    //     user_name: "Terry",
    //     nonce: 4051,
    //     hash_input: "546572727934303531",
    //     hash: "000073f0a11d96eec9592f6cba5eb15557009f7c2ff8c7bf9a8eea8819c6f3b1",
    // }
    #[test]
    fn test_pow_difficulty4() {
        let user_name = "Terry".to_string();
        let difficulty = "0000".to_string();

        let result = pow(user_name.clone(), difficulty).unwrap();
        println!("{:#?}", result);
    }

    // cargo test test_pow_difficulty5 -- --nocapture
    // output:
    // PowResult {
    //     user_name: "Terry",
    //     nonce: 626306,
    //     hash_input: "5465727279363236333036",
    //     hash: "00000edddc5e116c5f6bd39b78cb407bc50f90e3d7ba355fd3961e65131fcb05",
    // }
    // Elapsed: 1.860024826s
    #[test]
    fn test_pow_difficulty5() {
        let user_name = "Terry".to_string();
        let difficulty = "00000".to_string();

        let now = time::Instant::now();
        let result = pow(user_name.clone(), difficulty).unwrap();
        let elapsed = now.elapsed();
        println!("{:#?}", result);
        println!("Elapsed: {:?}", elapsed);
    }
}
