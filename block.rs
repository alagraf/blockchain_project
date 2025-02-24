//! This module defines the `Block` structure used in the blockchain.
//!
//! It provides methods for creating new blocks, generating the genesis block, 
//! calculating block hashes, and serializing/deserializing blocks.

use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};  // Import serialization traits

/// Represents a single block in the blockchain.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    /// The timestamp of when the block was created (milliseconds since UNIX epoch).
    pub timestamp: u128,  

    /// The hash of the previous block in the blockchain.
    pub prev_block_hash: String,

    /// The unique hash of the current block.
    pub hash: String,

    /// The height (index) of the block in the blockchain.
    pub height: usize,
}

impl Block {
    /// Creates a new block that links to the previous block.
    ///
    /// # Arguments
    ///
    /// * `prev_block_hash` - The hash of the previous block.
    /// * `height` - The position of the block in the blockchain.
    ///
    /// # Returns
    ///
    /// A new `Block` instance with a calculated hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// let prev_hash = "abc123".to_string();
    /// let block = Block::new_block(prev_hash, 1);
    /// ```
    pub fn new_block(prev_block_hash: String, height: usize) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();  

        let hash = Self::calculate_hash(timestamp, &prev_block_hash);

        Block {
            timestamp,
            prev_block_hash,
            hash,
            height,
        }
    }

    /// Generates the **Genesis Block**, the first block in the blockchain.
    ///
    /// The genesis block has a height of `0` and a predefined previous hash (`64` zeros).
    ///
    /// # Returns
    ///
    /// A `Block` instance representing the genesis block.
    ///
    /// # Example
    ///
    /// ```rust
    /// let genesis = Block::genesis_block();
    /// assert_eq!(genesis.height, 0);
    /// ```
    pub fn genesis_block() -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let prev_block_hash = "0".repeat(64);  // Default hash for genesis block
        let hash = Self::calculate_hash(timestamp, &prev_block_hash);

        Block {
            timestamp,
            prev_block_hash,
            hash,
            height: 0,  // Genesis block always starts at height 0
        }
    }

    /// Computes the SHA-256 hash of the block based on its timestamp and previous hash.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The block's creation timestamp.
    /// * `prev_block_hash` - The hash of the previous block.
    ///
    /// # Returns
    ///
    /// A `String` containing the computed hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// let hash = Block::calculate_hash(1234567890, "previous_hash");
    /// ```
    pub fn calculate_hash(timestamp: u128, prev_block_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(timestamp.to_string());
        hasher.update(prev_block_hash);  // Include previous block's hash in hashing
        let result = hasher.finalize();
        format!("{:x}", result) // Convert hash bytes to hexadecimal string
    }

    /// Serializes the block into a JSON string.
    ///
    /// # Returns
    ///
    /// A `String` containing the serialized JSON representation of the block.
    ///
    /// # Example
    ///
    /// ```rust
    /// let block = Block::genesis_block();
    /// let json = block.serialize();
    /// ```
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize block")
    }

    /// Deserializes a JSON string into a `Block` instance.
    ///
    /// # Arguments
    ///
    /// * `json_data` - The JSON string representing a block.
    ///
    /// # Returns
    ///
    /// A `Block` instance parsed from the JSON data.
    ///
    /// # Example
    ///
    /// ```rust
    /// let json = r#"{"timestamp": 123456, "prev_block_hash": "0", "hash": "xyz", "height": 0}"#;
    /// let block = Block::deserialize(json);
    /// ```
    pub fn deserialize(json_data: &str) -> Block {
        serde_json::from_str(json_data).expect("Failed to deserialize block")
    }

    /// Returns the hash of the block.
    ///
    /// # Returns
    ///
    /// A `String` containing the block's hash.
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    /// Returns the hash of the previous block.
    ///
    /// # Returns
    ///
    /// A `String` containing the previous block's hash.
    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    /// Returns the height of the block.
    ///
    /// # Returns
    ///
    /// A `usize` representing the block's position in the blockchain.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Returns the timestamp of when the block was created.
    ///
    /// # Returns
    ///
    /// A `u128` representing the timestamp in milliseconds since the UNIX epoch.
    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }
}
