//! This module initializes and manages the **P2P blockchain node**.
//!
//! It sets up the networking system, synchronizes the blockchain with peers,
//! and provides a command-line interface for interacting with the local blockchain.

use tokio::{io, io::AsyncBufReadExt, select, time::{timeout, Duration}};
use futures::stream::StreamExt;
use std::error::Error;
use network::{init_network, NetworkMessage, broadcast_message, list_peers, handle_event, handle_mdns};
use blockchain::*;
use block::Block;
use serde_json;

mod block;
mod blockchain;
mod network;

/// **Main entry point** for the P2P blockchain node.
///
/// This function:
/// - Initializes the **P2P networking** (GossipSub + mDNS).
/// - Handles blockchain synchronization with peers.
/// - Provides a **CLI-based menu** for user interactions.
///
/// # Returns
///
/// `Result<(), Box<dyn Error>>`
///
/// # Example
///
/// ```sh
/// cargo run
/// ```
///
/// This will start a blockchain node that can communicate with other peers.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the network swarm and topic for message broadcasting.
    let (mut swarm, topic) = init_network()?;

    // Local blockchain instance (starts fresh unless synchronized with peers).
    let mut local_blockchain = Blockchain::new();
    
    // Input reader for command-line interactions.
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    
    // Timeout duration for synchronization to prevent multiple genesis blocks.
    let sync_timeout = Duration::from_secs(10);

    println!("Node active.");
    println!("Initializing mDNS discovery...");

    // Attempt to discover peers within the sync timeout window.
    let sync_result = timeout(sync_timeout, handle_mdns(&mut swarm)).await;
    
    match sync_result {
        Ok(_) => println!("Initialization successful."),
        Err(_) => println!("Initialization failed."),
    }

    // Request blockchain data from peers to avoid duplicate genesis blocks.
    broadcast_message(&mut swarm, &topic, NetworkMessage::ChainRequest);

    // Command-line interface (CLI) loop for user interaction.
    loop {
        println!("\nOption menu:\n");
        println!("> Add Block (adds new block to blockchain)");
        println!("> List Peers (lists all active peers connected to the p2p network)");
        println!("> List Blockchain (prints the blocks of the local blockchain)\n");

        select! {
            // Read user input from the command line.
            Ok(Some(line)) = stdin.next_line() => {
                match line.as_str() {
                    
                    // Command to add a new block.
                    cmd if cmd.starts_with("Add Block") => {
                        let data = cmd.strip_prefix("Add Block").unwrap_or("").trim();
                        if !data.is_empty() {
                            
                            // Retrieve the last block in the local blockchain.
                            let prev_block = local_blockchain.get_last_block().unwrap();
                            
                            // Create a new block with incremented height.
                            let new_block = Block::new_block(
                                prev_block.get_hash().to_string(),
                                prev_block.get_height() + 1,
                            );

                            // Announce the new block to the P2P network.
                            let serialized_block = serde_json::to_string(&new_block).unwrap();
                            broadcast_message(&mut swarm, &topic, NetworkMessage::NewBlock(serialized_block));
                            
                            // Add the new block to the local blockchain.
                            local_blockchain.add_block(new_block);
                            println!("Block added and broadcasted to P2P network: {}", data);
                        }
                    }

                    // Command to list active peers.
                    cmd if cmd.starts_with("List Peers") => {
                        list_peers(&mut swarm);
                    }

                    // Command to display the blockchain.
                    cmd if cmd.starts_with("List Blockchain") => {
                        println!("\nCurrent Blockchain:");
                        for block in local_blockchain.get_blocks() {
                            println!("---------------------------");
                            println!("Timestamp: {}", block.get_timestamp());
                            println!("Previous Block Hash: {}", block.get_prev_hash());
                            println!("Current Block Hash: {}", block.get_hash());
                            println!("Height: {}", block.get_height());
                        }
                    }

                    // Handle unknown commands.
                    _ => println!("Unknown command."),
                }
            }

            // Process incoming network events (e.g., new blocks, peer messages).
            event = swarm.select_next_some() => handle_event(event, &mut swarm, &topic, &mut local_blockchain),
        }
    }
}
