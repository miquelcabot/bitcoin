use num_bigint::BigUint;
use sha2::{Digest, Sha256};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash256() {
        let data = b"hello world";
        let hash = hash256(data);
        assert_eq!(
            hash,
            BigUint::from_bytes_be(
                b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423"
            )
        );
    }
}
