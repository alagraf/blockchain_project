//! This module handles peer-to-peer networking in the blockchain system.
//!
//! It defines network behaviors, message types, and peer discovery mechanisms using **libp2p**.
//! The module supports:
//! - **GossipSub** for decentralized message broadcasting
//! - **mDNS** for peer discovery
//! - **Handling blockchain synchronization requests and responses**

use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration,
};

use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux,
    identity, PeerId,
};
use serde::{Serialize, Deserialize};
use tracing_subscriber::EnvFilter;
use std::collections::HashSet;
use futures::StreamExt;

use crate::blockchain::Blockchain;
use crate::block::Block;

/// Defines the custom network behavior by combining **GossipSub** and **mDNS** for peer discovery.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "CustomBehaviourEvent")]
pub struct CustomBehaviour {
    /// GossipSub protocol for decentralized message propagation.
    pub gossipsub: gossipsub::Behaviour,

    /// mDNS for local peer discovery.
    pub mdns: mdns::tokio::Behaviour,
}

/// Represents the events emitted by the custom network behavior.
#[derive(Debug)]
pub enum CustomBehaviourEvent {
    /// Event triggered by the GossipSub protocol.
    GossipSub(gossipsub::Event),

    /// Event triggered by the mDNS protocol.
    Mdns(mdns::Event),
}

impl From<gossipsub::Event> for CustomBehaviourEvent {
    fn from(event: gossipsub::Event) -> Self {
        CustomBehaviourEvent::GossipSub(event)
    }
}

impl From<mdns::Event> for CustomBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        CustomBehaviourEvent::Mdns(event)
    }
}

/// Defines the types of messages exchanged between peers in the network.
#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    /// Announces a new block to the network.
    NewBlock(String),

    /// Requests the current blockchain state from peers.
    ChainRequest,

    /// Responds to a `ChainRequest` with the serialized blockchain.
    ChainResponse(Vec<String>),
}

/// Initializes the P2P network, setting up **GossipSub** and **mDNS** for communication.
///
/// # Returns
///
/// A tuple containing the **Swarm** (networking entity) and the **GossipSub topic**.
///
/// # Errors
///
/// Returns an error if the network fails to initialize.
///
/// # Example
///
/// ```rust
/// let (swarm, topic) = init_network().expect("Failed to initialize network");
/// ```
pub fn init_network() -> Result<(Swarm<CustomBehaviour>, gossipsub::IdentTopic), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .ok();

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("New node peer id: {}", local_peer_id.to_string());

    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(15))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()?;

    let gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    )?;
    
    let mdns_config = mdns::Config {
        enable_ipv6: false,
        ttl: Duration::from_secs(20),
        query_interval: Duration::from_secs(10),
        ..Default::default()
    };

    let mdns = mdns::tokio::Behaviour::new(mdns_config, local_peer_id.clone())?;
    let behaviour = CustomBehaviour { gossipsub, mdns };

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, || yamux::Config::default())?
        .with_behaviour(|_| Ok(behaviour))?
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    let topic = gossipsub::IdentTopic::new("p2p_network");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    Ok((swarm, topic))
}

/// Lists all active peers discovered through **mDNS**.
///
/// # Arguments
///
/// * `swarm` - The network swarm instance.
pub fn list_peers(swarm: &mut Swarm<CustomBehaviour>) {
    println!("Active peers:");
    let peers: HashSet<_> = swarm.behaviour().mdns.discovered_nodes().collect();
    for peer in peers {
        println!("{:?}", peer);
    }
}

/// Broadcasts a message to all connected peers.
///
/// # Arguments
///
/// * `swarm` - The network swarm instance.
/// * `topic` - The GossipSub topic to broadcast on.
/// * `msg` - The message to be sent.
///
/// If no peers are connected, the function logs a warning.
pub fn broadcast_message(
    swarm: &mut Swarm<CustomBehaviour>,
    topic: &gossipsub::IdentTopic,
    msg: NetworkMessage,
) {
    let connected_peers: Vec<_> = swarm.behaviour().mdns.discovered_nodes().collect();

    if connected_peers.is_empty() {
        println!("No active peers in the network. Message was not broadcasted.");
        return;
    }

    let data = serde_json::to_vec(&msg).unwrap();
    match swarm.behaviour_mut().gossipsub.publish(topic.clone(), data) {
        Ok(_) => println!("Message broadcasted to network."),
        Err(e) => println!("Failed to broadcast: {:?}", e),
    }
}

/// Handles events related to **mDNS peer discovery** asynchronously.
///
/// # Arguments
///
/// * `swarm` - The network swarm instance.
pub async fn handle_mdns(swarm: &mut Swarm<CustomBehaviour>) {
    loop {
        if let Some(event) = swarm.next().await {
            match event {
                SwarmEvent::Behaviour(CustomBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                    for (peer_id, addr) in &peers {
                        println!("Discovered peer: {} at {}", peer_id, addr);
                    }
                }
                _ => {}
            }
        }
    }
}

/// Handles incoming network events and processes **blockchain messages**.
///
/// # Arguments
///
/// * `event` - The event to be processed.
/// * `swarm` - The network swarm instance.
/// * `topic` - The GossipSub topic.
/// * `local_blockchain` - The local blockchain instance.
pub fn handle_event(
    event: SwarmEvent<CustomBehaviourEvent>,
    swarm: &mut Swarm<CustomBehaviour>,
    topic: &gossipsub::IdentTopic,
    local_blockchain: &mut Blockchain,
) {
    match event {
        SwarmEvent::Behaviour(CustomBehaviourEvent::GossipSub(gossipsub::Event::Message { message, .. })) => {
            if let Ok(decoded) = serde_json::from_slice::<NetworkMessage>(&message.data) {
                match decoded {
                    NetworkMessage::NewBlock(block_data) => {
                        println!("New Block Received: {:?}", block_data);
                        let block: Block = match serde_json::from_str(&block_data) {
                            Ok(b) => b,
                            Err(e) => {
                                println!("Failed to deserialize Block: {:?}", e);
                                return;
                            }
                        };
                        
                        if !local_blockchain.add_block(block) {
                            println!("NewBlock Error!");
                            return;
                        }
                        println!("Successfully added the block to local blockchain!");
                    }

                    NetworkMessage::ChainRequest => {
                        let serialized_blocks: Vec<String> = local_blockchain.get_blocks()
                            .iter()
                            .map(|block| serde_json::to_string(block).unwrap())
                            .collect();
                        
                        let response = NetworkMessage::ChainResponse(serialized_blocks);
                        let data = serde_json::to_vec(&response).unwrap();
                        swarm.behaviour_mut().gossipsub.publish(topic.clone(), data).unwrap();
                    }

                    _ => println!("⚠️ Received invalid message from {:?}", message.source),
                }
            }
        }
        _ => {}
    }
}
