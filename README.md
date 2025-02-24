# P2P Blockchain Command Menu

This README explains how to use the interactive command menu for the P2P Blockchain network.

## Running the Program

Ensure you have the required dependencies installed and run the Rust program. Once running, you will see an interactive menu that allows you to interact with the blockchain.

## Command Menu

After launching the program, you will see the following menu:

```
Option menu:

> Add Block (adds new block to blockchain)
> List Peers (lists all active peers connected to the p2p network)
> List Blockchain (prints the blocks of the local blockchain)
```

## Commands

### 1. Add a New Block

To add a new block to the blockchain, use the following command:

```
Add Block <block_data>
```

- `<block_data>`: Replace this with the actual data you want to store in the block.
- The program will:
  - Retrieve the last block in the blockchain.
  - Create a new block with an incremented height.
  - Broadcast the new block to the P2P network.
  - Add the new block to the local blockchain.

#### Example:

```
Add Block Transaction123
```

### 2. List Active Peers

To list all active peers connected to the P2P network, use:

```
List Peers
```

- This will print all active peers currently connected to the network.

### 3. List the Blockchain

To display the local blockchain, use:

```
List Blockchain
```

- This will print all blocks currently stored in the local blockchain, including:
  - **Timestamp**
  - **Previous block hash**
  - **Current block hash**
  - **Block height**

## Unknown Commands

If an unknown command is entered, the system will display:

```
Unknown command.
```

