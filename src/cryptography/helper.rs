use anyhow::Result;
use num_bigint::BigUint;
use sha2::{Digest, Sha256};

const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Perform a double sha256 hash on the given data.
/// # Arguments
/// * `data` - The data to hash
/// # Returns
/// * `Vec<u8>` - The hash of the data
pub fn hash256(data: &[u8]) -> BigUint {
    // First round of sha256
    let mut hasher = Sha256::new();
    hasher.update(data);
    let first_round = hasher.finalize();

    // Second round of sha256
    let mut hasher = Sha256::new();
    hasher.update(first_round);
    let second_round = hasher.finalize();

    // Convert the hash to a BigUint
    BigUint::from_bytes_be(&second_round)
}

/// Encode a byte array into a Base58 string.
/// # Arguments
/// * `data` - The byte array to encode
/// # Returns
/// * `String` - The Base58 encoded string
pub fn encode_base58(data: &[u8]) -> Result<String> {
    // Count leading zeroes
    let mut count = 0;
    for &byte in data {
        if byte == 0 {
            count += 1;
        } else {
            break;
        }
    }

    // Convert the byte array into a big integer
    let mut num = BigUint::parse_bytes(data, 16)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse prime from bytes"))?;
    let base = BigUint::from(58u8);
    let mut result = String::new();

    // Convert to Base58
    while num > BigUint::ZERO {
        let remainder_digits = (&num % &base).to_u32_digits();
        let mut remainder: usize = 0;
        if !remainder_digits.is_empty() {
            remainder = remainder_digits[0] as usize;
        }
        num /= &base;
        result.insert(0, BASE58_ALPHABET[remainder] as char);
    }

    // Add '1' prefix for each leading zero
    let prefix = "1".repeat(count);
    Ok(format!("{}{}", prefix, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash256() {
        let data = b"hello world";
        let hash = hash256(data);
        assert_eq!(
            hash,
            BigUint::parse_bytes(
                b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
                16
            )
            .unwrap()
        );
    }

    // Convert the following hex values to binary and then to Base58:
    // - 7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d
    // - eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c
    // - c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6
    #[test]
    fn test_encode_base58() -> Result<()> {
        let hex_values1 = b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let hex_values2 = b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c";
        let hex_values3 = b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6";

        let encoded_base58_1 = encode_base58(hex_values1)?;
        let encoded_base58_2 = encode_base58(hex_values2)?;
        let encoded_base58_3 = encode_base58(hex_values3)?;

        assert_eq!(
            encoded_base58_1,
            "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6"
        );
        assert_eq!(
            encoded_base58_2,
            "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd"
        );
        assert_eq!(
            encoded_base58_3,
            "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7"
        );

        Ok(())
    }
}
