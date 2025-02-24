//! This module defines the **Blockchain** structure and its associated methods.
//!
//! The `Blockchain` struct manages a chain of blocks, ensuring data integrity
//! and validating blocks before adding them to the chain.

use crate::block::Block;  // Import the Block struct

/// Represents a blockchain, which consists of a sequence of blocks.
///
/// The blockchain starts with a **genesis block** and ensures each new block is
/// linked to the previous block through cryptographic hashes.
#[derive(Debug)]
pub struct Blockchain {
    /// The list of blocks in the blockchain.
    blocks: Vec<Block>,
}

impl Blockchain {
    /// Initializes a new blockchain with a **genesis block**.
    ///
    /// # Returns
    ///
    /// A `Blockchain` instance with a single genesis block.
    ///
    /// # Example
    ///
    /// ```rust
    /// let blockchain = Blockchain::new();
    /// ```
    pub fn new() -> Self {
        let genesis_block = Block::genesis_block();
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    /// Creates a blockchain from an existing list of blocks.
    ///
    /// This method ensures that the provided blocks form a valid blockchain.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of `Block` instances representing an existing blockchain.
    ///
    /// # Panics
    ///
    /// If the provided blockchain is invalid, the function **panics** to prevent corruption.
    ///
    /// # Returns
    ///
    /// A `Blockchain` instance built from the given blocks.
    ///
    /// # Example
    ///
    /// ```rust
    /// let blocks = vec![Block::genesis_block()];
    /// let blockchain = Blockchain::from_blocks(blocks);
    /// ```
    pub fn from_blocks(data: Vec<Block>) -> Self {
        let blockchain = Blockchain { blocks: data };
        if !blockchain.is_valid() {
            panic!("Invalid blockchain provided!");
        }
        blockchain
    }

    /// Adds a new block to the blockchain after validating its integrity.
    ///
    /// The new block must have a **previous hash** that matches the last block’s hash.
    ///
    /// # Arguments
    ///
    /// * `block` - The `Block` instance to be added.
    ///
    /// # Returns
    ///
    /// - `true` if the block was successfully added.
    /// - `false` if the block was rejected due to **an invalid previous hash**.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut blockchain = Blockchain::new();
    /// let prev_block = blockchain.get_last_block().unwrap();
    /// let new_block = Block::new_block(prev_block.get_hash(), prev_block.get_height() + 1);
    /// let added = blockchain.add_block(new_block);
    /// assert!(added);
    /// ```
    pub fn add_block(&mut self, block: Block) -> bool {
        // Get the last block for validation
        if let Some(last_block) = self.get_last_block() {
            // Check if the block’s previous hash matches the last block’s hash
            if block.get_prev_hash() != last_block.get_hash() {
                println!("Block rejected: Invalid previous hash.");
                return false;
            }

            // If validation passes, add the block
            self.blocks.push(block);
            println!("Block successfully added.");
            true
        } else {
            println!("Blockchain is empty. Cannot add block.");
            false
        }
    }
    
    /// Retrieves the entire blockchain as a reference to a vector of blocks.
    ///
    /// # Returns
    ///
    /// A reference to the list of `Block` instances stored in the blockchain.
    ///
    /// # Example
    ///
    /// ```rust
    /// let blockchain = Blockchain::new();
    /// let blocks = blockchain.get_blocks();
    /// assert_eq!(blocks.len(), 1); // Should contain the genesis block.
    /// ```
    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    /// Retrieves the last block in the blockchain.
    ///
    /// # Returns
    ///
    /// - `Some(&Block)` if the blockchain contains at least one block.
    /// - `None` if the blockchain is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// let blockchain = Blockchain::new();
    /// let last_block = blockchain.get_last_block().unwrap();
    /// ```
    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }    

    /// Validates the blockchain to ensure its integrity.
    ///
    /// This method checks:
    /// - That each block’s `prev_block_hash` matches the hash of the previous block.
    /// - That each block’s hash is correctly computed.
    ///
    /// # Returns
    ///
    /// - `true` if the blockchain is valid.
    /// - `false` if **any block is tampered with or out of order**.
    ///
    /// # Example
    ///
    /// ```rust
    /// let blockchain = Blockchain::new();
    /// assert!(blockchain.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // Check that the previous hash matches
            if current.get_prev_hash() != previous.get_hash() {
                println!("Block {} has an invalid previous hash!", i);
                return false;
            }

            // Recalculate hash and compare it to the stored hash
            let recalculated_hash = Block::calculate_hash(current.timestamp, &current.prev_block_hash);
            if current.get_hash() != recalculated_hash {
                println!("Block {} has been tampered with!", i);
                return false;
            }
        }
        true
    }
}
