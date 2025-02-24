mod block;
mod blockchain;
use crate::block::Block;
use crate::blockchain::Blockchain;
use std::fs;
use std::io::{self, Write};

fn main() {
    // Step 1: Initialize a new blockchain with the Genesis Block
    let mut blockchain = Blockchain::new();
    println!("🚀 Blockchain initialized with Genesis Block:");
    print_block_details(blockchain.get_last_block().unwrap());

    // Step 2: Add multiple blocks to the blockchain
    println!("\n🔗 Adding new blocks...");
    for i in 1..=3 {
        let prev_hash = blockchain.get_last_block().unwrap().get_hash();
        let new_block = Block::new_block(prev_hash, i);
        blockchain.add_block(new_block);
    }

    // Step 3: Print the entire blockchain
    println!("\n📜 Current Blockchain:");
    for block in blockchain.get_blocks() {
        print_block_details(block);
    }

    // Step 4: Serialize and save blockchain to a file
    let blockchain_json = serde_json::to_string_pretty(blockchain.get_blocks()).expect("Serialization failed");
    let filename = "blockchain_data.json";
    fs::write(filename, &blockchain_json).expect("Failed to save blockchain to file");
    println!("\n💾 Blockchain saved to `{}`.", filename);

    // Step 5: Load blockchain from the file and verify integrity
    println!("\n📂 Loading blockchain from file...");
    let loaded_json = fs::read_to_string(filename).expect("Failed to read file");
    let loaded_blocks: Vec<Block> = serde_json::from_str(&loaded_json).expect("Failed to deserialize blockchain");
    let loaded_blockchain = Blockchain::from_blocks(loaded_blocks);

    // Step 6: Validate blockchain integrity
    println!("\n✅ Blockchain validity check: {}", loaded_blockchain.is_valid());

    // Optional: User interaction
    println!("\n🔍 Do you want to inspect a block? (Enter block index or `exit`)");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("👋 Exiting...");
            break;
        }

        match input.parse::<usize>() {
            Ok(index) if index < loaded_blockchain.get_blocks().len() => {
                print_block_details(&loaded_blockchain.get_blocks()[index]);
            }
            _ => println!("❌ Invalid index. Try again or type `exit`."),
        }
    }
}

/// Utility function to print block details
fn print_block_details(block: &Block) {
    println!(
        "📦 Block #{}\n  ⏳ Timestamp: {}\n  🔗 Prev Hash: {}\n  🔑 Hash: {}",
        block.get_height(),
        block.timestamp,
        block.get_prev_hash(),
        block.get_hash()
    );
}
