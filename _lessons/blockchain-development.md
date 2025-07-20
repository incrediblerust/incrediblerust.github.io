---
title: "Blockchain Development with Rust"
difficulty: advanced
version: "1.85.0"
---

# Blockchain Development with Rust

Rust's performance and safety make it ideal for blockchain development. Let's build blockchain applications from fundamentals to advanced concepts.

## Basic Blockchain Implementation

### Block Structure

```rust
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, fee: u64) -> Self {
        Self {
            from,
            to,
            amount,
            fee,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signature: String::new(),
        }
    }
    
    pub fn calculate_hash(&self) -> String {
        let transaction_string = format!(
            "{}{}{}{}{}",
            self.from, self.to, self.amount, self.fee, self.timestamp
        );
        
        let mut hasher = Sha256::new();
        hasher.update(transaction_string.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn sign(&mut self, private_key: &str) {
        // Simplified signing - in real implementation use proper cryptography
        let hash = self.calculate_hash();
        self.signature = format!("{}:{}", private_key, hash);
    }
    
    pub fn verify_signature(&self, public_key: &str) -> bool {
        // Simplified verification
        let expected_hash = self.calculate_hash();
        self.signature.ends_with(&expected_hash)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        
        block.hash = block.calculate_hash();
        block
    }
    
    pub fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            serde_json::to_string(&self.transactions).unwrap_or_default(),
            self.previous_hash,
            self.nonce
        );
        
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        println!("Mining block {}...", self.index);
        let start_time = SystemTime::now();
        
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        
        let duration = start_time.elapsed().unwrap();
        println!(
            "Block {} mined: {} (nonce: {}, time: {:?})",
            self.index, self.hash, self.nonce, duration
        );
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4,
            pending_transactions: Vec::new(),
            mining_reward: 100,
        };
        
        blockchain.create_genesis_block();
        blockchain
    }
    
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, Vec::new(), String::from("0"));
        self.chain.push(genesis_block);
    }
    
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    
    pub fn create_transaction(&mut self, transaction: Transaction) {
        // Validate transaction
        if !self.validate_transaction(&transaction) {
            println!("Invalid transaction: {:?}", transaction);
            return;
        }
        
        self.pending_transactions.push(transaction);
    }
    
    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        // Add mining reward transaction
        let reward_transaction = Transaction::new(
            String::new(), // From system
            mining_reward_address,
            self.mining_reward,
            0,
        );
        
        self.pending_transactions.push(reward_transaction);
        
        let previous_hash = self.get_latest_block().hash.clone();
        let mut block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            previous_hash,
        );
        
        block.mine_block(self.difficulty);
        
        self.chain.push(block);
        self.pending_transactions.clear();
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance = 0u64;
        
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from == address {
                    balance = balance.saturating_sub(transaction.amount + transaction.fee);
                }
                if transaction.to == address {
                    balance += transaction.amount;
                }
            }
        }
        
        balance
    }
    
    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        // Check if sender has enough balance
        if !transaction.from.is_empty() {
            let balance = self.get_balance(&transaction.from);
            if balance < transaction.amount + transaction.fee {
                return false;
            }
        }
        
        // Verify signature (simplified)
        if !transaction.from.is_empty() && transaction.signature.is_empty() {
            return false;
        }
        
        true
    }
    
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            // Check if current block's hash is valid
            if current_block.hash != current_block.calculate_hash() {
                println!("Invalid hash at block {}", i);
                return false;
            }
            
            // Check if current block points to previous block
            if current_block.previous_hash != previous_block.hash {
                println!("Invalid previous hash at block {}", i);
                return false;
            }
            
            // Check if block is properly mined
            if !current_block.hash.starts_with(&"0".repeat(self.difficulty)) {
                println!("Block {} not properly mined", i);
                return false;
            }
        }
        
        true
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut blockchain = Blockchain::new();
    
    println!("Starting blockchain mining...");
    
    // Create some transactions
    let mut tx1 = Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        50,
        1,
    );
    tx1.sign("alice_private_key");
    
    let mut tx2 = Transaction::new(
        "Bob".to_string(),
        "Charlie".to_string(),
        25,
        1,
    );
    tx2.sign("bob_private_key");
    
    blockchain.create_transaction(tx1);
    blockchain.create_transaction(tx2);
    
    // Mine block
    blockchain.mine_pending_transactions("Miner1".to_string());
    
    // Check balances
    println!("Alice balance: {}", blockchain.get_balance("Alice"));
    println!("Bob balance: {}", blockchain.get_balance("Bob"));
    println!("Charlie balance: {}", blockchain.get_balance("Charlie"));
    println!("Miner1 balance: {}", blockchain.get_balance("Miner1"));
    
    // Validate blockchain
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
    
    Ok(())
}
```

## Smart Contract Platform

### Virtual Machine

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Gt,
    Jump(usize),
    JumpIf(usize),
    Call(String),
    Return,
    Store(String),
    Load(String),
    Halt,
}

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    stack: Vec<i64>,
    memory: HashMap<String, i64>,
    program_counter: usize,
    program: Vec<OpCode>,
    call_stack: Vec<usize>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            memory: HashMap::new(),
            program_counter: 0,
            program: Vec::new(),
            call_stack: Vec::new(),
        }
    }
    
    pub fn load_program(&mut self, program: Vec<OpCode>) {
        self.program = program;
        self.program_counter = 0;
    }
    
    pub fn execute(&mut self) -> Result<(), String> {
        while self.program_counter < self.program.len() {
            let instruction = self.program[self.program_counter].clone();
            
            match instruction {
                OpCode::Push(value) => {
                    self.stack.push(value);
                }
                
                OpCode::Pop => {
                    if self.stack.is_empty() {
                        return Err("Stack underflow".to_string());
                    }
                    self.stack.pop();
                }
                
                OpCode::Add => {
                    if self.stack.len() < 2 {
                        return Err("Not enough operands for ADD".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                
                OpCode::Sub => {
                    if self.stack.len() < 2 {
                        return Err("Not enough operands for SUB".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                
                OpCode::Mul => {
                    if self.stack.len() < 2 {
                        return Err("Not enough operands for MUL".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                
                OpCode::Div => {
                    if self.stack.len() < 2 {
                        return Err("Not enough operands for DIV".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if b == 0 {
                        return Err("Division by zero".to_string());
                    }
                    self.stack.push(a / b);
                }
                
                OpCode::Eq => {
                    if self.stack.len() < 2 {
                        return Err("Not enough operands for EQ".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a == b { 1 } else { 0 });
                }
                
                OpCode::Lt => {
                    if self.stack.len() < 2 {
                        return Err("Not enough operands for LT".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a < b { 1 } else { 0 });
                }
                
                OpCode::Gt => {
                    if self.stack.len() < 2 {
                        return Err("Not enough operands for GT".to_string());
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a > b { 1 } else { 0 });
                }
                
                OpCode::Jump(address) => {
                    self.program_counter = address;
                    continue;
                }
                
                OpCode::JumpIf(address) => {
                    if self.stack.is_empty() {
                        return Err("Stack underflow for JUMPIF".to_string());
                    }
                    let condition = self.stack.pop().unwrap();
                    if condition != 0 {
                        self.program_counter = address;
                        continue;
                    }
                }
                
                OpCode::Store(variable) => {
                    if self.stack.is_empty() {
                        return Err("Stack underflow for STORE".to_string());
                    }
                    let value = self.stack.pop().unwrap();
                    self.memory.insert(variable, value);
                }
                
                OpCode::Load(variable) => {
                    let value = self.memory.get(&variable).copied().unwrap_or(0);
                    self.stack.push(value);
                }
                
                OpCode::Call(function) => {
                    self.call_stack.push(self.program_counter + 1);
                    // In a real implementation, this would jump to function address
                    println!("Calling function: {}", function);
                }
                
                OpCode::Return => {
                    if let Some(return_address) = self.call_stack.pop() {
                        self.program_counter = return_address;
                        continue;
                    } else {
                        return Err("Return without call".to_string());
                    }
                }
                
                OpCode::Halt => {
                    break;
                }
            }
            
            self.program_counter += 1;
        }
        
        Ok(())
    }
    
    pub fn get_top(&self) -> Option<i64> {
        self.stack.last().copied()
    }
    
    pub fn get_memory(&self, key: &str) -> Option<i64> {
        self.memory.get(key).copied()
    }
}

// Smart Contract Implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContract {
    pub address: String,
    pub bytecode: Vec<OpCode>,
    pub state: HashMap<String, i64>,
    pub owner: String,
}

impl SmartContract {
    pub fn new(address: String, bytecode: Vec<OpCode>, owner: String) -> Self {
        Self {
            address,
            bytecode,
            state: HashMap::new(),
            owner,
        }
    }
    
    pub fn execute(&mut self, input: HashMap<String, i64>) -> Result<i64, String> {
        let mut vm = VirtualMachine::new();
        
        // Load contract state into VM memory
        for (key, value) in &self.state {
            vm.memory.insert(key.clone(), *value);
        }
        
        // Load input parameters
        for (key, value) in input {
            vm.memory.insert(key, value);
        }
        
        vm.load_program(self.bytecode.clone());
        vm.execute()?;
        
        // Save contract state
        self.state = vm.memory.clone();
        
        // Return result from stack
        vm.get_top().ok_or_else(|| "No return value".to_string())
    }
}

// Token Contract Example
fn create_token_contract() -> SmartContract {
    let bytecode = vec![
        // Initialize total supply
        OpCode::Push(1000000),
        OpCode::Store("total_supply".to_string()),
        
        // Set owner balance
        OpCode::Push(1000000),
        OpCode::Store("balance_owner".to_string()),
        
        // Transfer function simulation
        OpCode::Load("from_balance".to_string()),
        OpCode::Load("amount".to_string()),
        OpCode::Sub,
        OpCode::Store("from_balance".to_string()),
        
        OpCode::Load("to_balance".to_string()),
        OpCode::Load("amount".to_string()),
        OpCode::Add,
        OpCode::Store("to_balance".to_string()),
        
        OpCode::Push(1), // Success
        OpCode::Halt,
    ];
    
    SmartContract::new(
        "0x1234567890abcdef".to_string(),
        bytecode,
        "owner_address".to_string(),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test VM with simple program
    let mut vm = VirtualMachine::new();
    
    let program = vec![
        OpCode::Push(10),
        OpCode::Push(20),
        OpCode::Add,
        OpCode::Store("result".to_string()),
        OpCode::Halt,
    ];
    
    vm.load_program(program);
    vm.execute()?;
    
    println!("VM result: {:?}", vm.get_memory("result"));
    
    // Test smart contract
    let mut token_contract = create_token_contract();
    
    let mut input = HashMap::new();
    input.insert("from_balance".to_string(), 100);
    input.insert("to_balance".to_string(), 50);
    input.insert("amount".to_string(), 25);
    
    let result = token_contract.execute(input)?;
    println!("Contract execution result: {}", result);
    println!("Contract state: {:?}", token_contract.state);
    
    Ok(())
}
```

## Consensus Mechanism - Proof of Work

```rust
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone)]
pub struct ProofOfWork {
    pub difficulty: usize,
    pub target: String,
}

impl ProofOfWork {
    pub fn new(difficulty: usize) -> Self {
        let target = "0".repeat(difficulty);
        Self { difficulty, target }
    }
    
    pub fn mine(&self, data: &str) -> (u64, String) {
        let mut nonce = 0u64;
        
        loop {
            let input = format!("{}{}", data, nonce);
            let hash = self.calculate_hash(&input);
            
            if hash.starts_with(&self.target) {
                return (nonce, hash);
            }
            
            nonce += 1;
            
            if nonce % 100000 == 0 {
                println!("Tried {} nonces...", nonce);
            }
        }
    }
    
    pub fn mine_parallel(&self, data: &str, num_threads: usize) -> (u64, String) {
        let found = Arc::new(AtomicBool::new(false));
        let result = Arc::new(Mutex::new(None));
        let mut handles = vec![];
        
        for i in 0..num_threads {
            let data = data.to_string();
            let target = self.target.clone();
            let found_clone = found.clone();
            let result_clone = result.clone();
            
            let handle = thread::spawn(move || {
                let mut nonce = i as u64;
                
                while !found_clone.load(Ordering::Relaxed) {
                    let input = format!("{}{}", data, nonce);
                    let hash = calculate_hash(&input);
                    
                    if hash.starts_with(&target) {
                        found_clone.store(true, Ordering::Relaxed);
                        let mut result_guard = result_clone.lock().unwrap();
                        *result_guard = Some((nonce, hash));
                        break;
                    }
                    
                    nonce += num_threads as u64;
                }
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let result_guard = result.lock().unwrap();
        result_guard.clone().unwrap()
    }
    
    fn calculate_hash(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

fn calculate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Mining Pool Implementation
#[derive(Debug)]
pub struct MiningPool {
    miners: Vec<String>,
    shares: std::collections::HashMap<String, u64>,
    total_hashrate: u64,
}

impl MiningPool {
    pub fn new() -> Self {
        Self {
            miners: Vec::new(),
            shares: std::collections::HashMap::new(),
            total_hashrate: 0,
        }
    }
    
    pub fn add_miner(&mut self, miner_id: String, hashrate: u64) {
        self.miners.push(miner_id.clone());
        self.shares.insert(miner_id, 0);
        self.total_hashrate += hashrate;
    }
    
    pub fn submit_share(&mut self, miner_id: &str, difficulty: u64) {
        if let Some(shares) = self.shares.get_mut(miner_id) {
            *shares += difficulty;
        }
    }
    
    pub fn distribute_reward(&mut self, total_reward: u64) -> std::collections::HashMap<String, u64> {
        let total_shares: u64 = self.shares.values().sum();
        let mut rewards = std::collections::HashMap::new();
        
        if total_shares == 0 {
            return rewards;
        }
        
        for (miner_id, shares) in &self.shares {
            let reward = (total_reward * shares) / total_shares;
            rewards.insert(miner_id.clone(), reward);
        }
        
        // Reset shares for next round
        for shares in self.shares.values_mut() {
            *shares = 0;
        }
        
        rewards
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Proof of Work mining...");
    
    let pow = ProofOfWork::new(4); // 4 leading zeros
    let data = "Hello, Blockchain!";
    
    println!("Mining with difficulty {}...", pow.difficulty);
    let start_time = SystemTime::now();
    
    let (nonce, hash) = pow.mine(data);
    
    let duration = start_time.elapsed().unwrap();
    println!("Found solution!");
    println!("Nonce: {}", nonce);
    println!("Hash: {}", hash);
    println!("Time taken: {:?}", duration);
    
    // Test parallel mining
    println!("\nTesting parallel mining...");
    let start_time = SystemTime::now();
    
    let (nonce, hash) = pow.mine_parallel(data, 4);
    
    let duration = start_time.elapsed().unwrap();
    println!("Parallel mining result:");
    println!("Nonce: {}", nonce);
    println!("Hash: {}", hash);
    println!("Time taken: {:?}", duration);
    
    // Test mining pool
    println!("\nTesting mining pool...");
    let mut pool = MiningPool::new();
    
    pool.add_miner("miner1".to_string(), 1000);
    pool.add_miner("miner2".to_string(), 2000);
    pool.add_miner("miner3".to_string(), 1500);
    
    // Simulate share submissions
    pool.submit_share("miner1", 10);
    pool.submit_share("miner2", 25);
    pool.submit_share("miner3", 15);
    
    let rewards = pool.distribute_reward(1000);
    println!("Reward distribution: {:?}", rewards);
    
    Ok(())
}
```

## Cryptocurrency Wallet

```rust
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160, Digest as RipemdDigest};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        
        // Generate private key
        let private_key = SecretKey::new(&mut rng);
        let private_key_hex = hex::encode(private_key.as_ref());
        
        // Generate public key
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let public_key_hex = hex::encode(public_key.serialize());
        
        // Generate address
        let address = Self::public_key_to_address(&public_key)?;
        
        Ok(Wallet {
            private_key: private_key_hex,
            public_key: public_key_hex,
            address,
        })
    }
    
    pub fn from_private_key(private_key_hex: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        
        let private_key_bytes = hex::decode(private_key_hex)?;
        let private_key = SecretKey::from_slice(&private_key_bytes)?;
        
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let public_key_hex = hex::encode(public_key.serialize());
        
        let address = Self::public_key_to_address(&public_key)?;
        
        Ok(Wallet {
            private_key: private_key_hex.to_string(),
            public_key: public_key_hex,
            address,
        })
    }
    
    fn public_key_to_address(public_key: &PublicKey) -> Result<String, Box<dyn std::error::Error>> {
        // SHA256 hash of public key
        let mut hasher = Sha256::new();
        hasher.update(&public_key.serialize());
        let sha256_hash = hasher.finalize();
        
        // RIPEMD160 hash of SHA256 hash
        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(&sha256_hash);
        let ripemd_hash = ripemd_hasher.finalize();
        
        // Add version byte (0x00 for mainnet)
        let mut versioned_payload = vec![0x00];
        versioned_payload.extend_from_slice(&ripemd_hash);
        
        // Double SHA256 for checksum
        let mut hasher1 = Sha256::new();
        hasher1.update(&versioned_payload);
        let hash1 = hasher1.finalize();
        
        let mut hasher2 = Sha256::new();
        hasher2.update(&hash1);
        let hash2 = hasher2.finalize();
        
        // Take first 4 bytes as checksum
        let checksum = &hash2[0..4];
        
        // Combine versioned payload and checksum
        let mut full_payload = versioned_payload;
        full_payload.extend_from_slice(checksum);
        
        // Base58 encode (simplified - using hex for this example)
        Ok(hex::encode(full_payload))
    }
    
    pub fn sign_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let private_key_bytes = hex::decode(&self.private_key)?;
        let private_key = SecretKey::from_slice(&private_key_bytes)?;
        
        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        let message_hash = hasher.finalize();
        
        // Sign the hash
        let message = secp256k1::Message::from_slice(&message_hash)?;
        let signature = secp.sign_ecdsa(&message, &private_key);
        
        Ok(hex::encode(signature.serialize_compact()))
    }
    
    pub fn verify_signature(
        public_key_hex: &str,
        message: &str,
        signature_hex: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        
        let public_key_bytes = hex::decode(public_key_hex)?;
        let public_key = PublicKey::from_slice(&public_key_bytes)?;
        
        let signature_bytes = hex::decode(signature_hex)?;
        let signature = secp256k1::ecdsa::Signature::from_compact(&signature_bytes)?;
        
        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        let message_hash = hasher.finalize();
        
        let message = secp256k1::Message::from_slice(&message_hash)?;
        
        Ok(secp.verify_ecdsa(&message, &signature, &public_key).is_ok())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletManager {
    pub wallets: HashMap<String, Wallet>,
    pub balances: HashMap<String, u64>,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            balances: HashMap::new(),
        }
    }
    
    pub fn create_wallet(&mut self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        let wallet = Wallet::new()?;
        let address = wallet.address.clone();
        
        self.wallets.insert(name, wallet);
        self.balances.insert(address.clone(), 0);
        
        Ok(address)
    }
    
    pub fn import_wallet(
        &mut self,
        name: String,
        private_key: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let wallet = Wallet::from_private_key(private_key)?;
        let address = wallet.address.clone();
        
        self.wallets.insert(name, wallet);
        self.balances.insert(address.clone(), 0);
        
        Ok(address)
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }
    
    pub fn send_transaction(
        &mut self,
        from_wallet: &str,
        to_address: &str,
        amount: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let wallet = self.wallets.get(from_wallet)
            .ok_or("Wallet not found")?;
        
        let from_address = &wallet.address;
        let current_balance = self.get_balance(from_address);
        
        if current_balance < amount {
            return Err("Insufficient balance".into());
        }
        
        // Create transaction data
        let transaction_data = format!("{}:{}:{}", from_address, to_address, amount);
        
        // Sign transaction
        let signature = wallet.sign_message(&transaction_data)?;
        
        // Update balances (in real implementation, this would be handled by blockchain)
        *self.balances.get_mut(from_address).unwrap() -= amount;
        *self.balances.entry(to_address.to_string()).or_insert(0) += amount;
        
        println!("Transaction sent: {} -> {} (amount: {})", from_address, to_address, amount);
        println!("Signature: {}", signature);
        
        Ok(signature)
    }
    
    pub fn add_balance(&mut self, address: &str, amount: u64) {
        *self.balances.entry(address.to_string()).or_insert(0) += amount;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet_manager = WalletManager::new();
    
    // Create wallets
    let alice_address = wallet_manager.create_wallet("Alice".to_string())?;
    let bob_address = wallet_manager.create_wallet("Bob".to_string())?;
    
    println!("Alice address: {}", alice_address);
    println!("Bob address: {}", bob_address);
    
    // Add some initial balance to Alice
    wallet_manager.add_balance(&alice_address, 1000);
    
    println!("Alice balance: {}", wallet_manager.get_balance(&alice_address));
    println!("Bob balance: {}", wallet_manager.get_balance(&bob_address));
    
    // Send transaction
    let signature = wallet_manager.send_transaction("Alice", &bob_address, 250)?;
    
    println!("After transaction:");
    println!("Alice balance: {}", wallet_manager.get_balance(&alice_address));
    println!("Bob balance: {}", wallet_manager.get_balance(&bob_address));
    
    // Verify signature
    let alice_wallet = &wallet_manager.wallets["Alice"];
    let transaction_data = format!("{}:{}:{}", alice_address, bob_address, 250);
    
    let is_valid = Wallet::verify_signature(
        &alice_wallet.public_key,
        &transaction_data,
        &signature,
    )?;
    
    println!("Signature valid: {}", is_valid);
    
    Ok(())
}
```

## DeFi Protocol Example

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub token_a: String,
    pub token_b: String,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub total_liquidity: u64,
    pub liquidity_providers: HashMap<String, u64>,
}

impl LiquidityPool {
    pub fn new(token_a: String, token_b: String) -> Self {
        Self {
            token_a,
            token_b,
            reserve_a: 0,
            reserve_b: 0,
            total_liquidity: 0,
            liquidity_providers: HashMap::new(),
        }
    }
    
    pub fn add_liquidity(
        &mut self,
        provider: String,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<u64, &'static str> {
        // Calculate liquidity tokens to mint
        let liquidity_minted = if self.total_liquidity == 0 {
            // First liquidity provision
            (amount_a * amount_b).integer_sqrt()
        } else {
            // Subsequent provisions - maintain ratio
            let liquidity_a = (amount_a * self.total_liquidity) / self.reserve_a;
            let liquidity_b = (amount_b * self.total_liquidity) / self.reserve_b;
            std::cmp::min(liquidity_a, liquidity_b)
        };
        
        if liquidity_minted == 0 {
            return Err("Insufficient liquidity minted");
        }
        
        // Update reserves
        self.reserve_a += amount_a;
        self.reserve_b += amount_b;
        self.total_liquidity += liquidity_minted;
        
        // Update provider's liquidity
        *self.liquidity_providers.entry(provider).or_insert(0) += liquidity_minted;
        
        Ok(liquidity_minted)
    }
    
    pub fn remove_liquidity(
        &mut self,
        provider: &str,
        liquidity_amount: u64,
    ) -> Result<(u64, u64), &'static str> {
        let provider_liquidity = self.liquidity_providers.get(provider).copied().unwrap_or(0);
        
        if provider_liquidity < liquidity_amount {
            return Err("Insufficient liquidity");
        }
        
        // Calculate token amounts to return
        let amount_a = (liquidity_amount * self.reserve_a) / self.total_liquidity;
        let amount_b = (liquidity_amount * self.reserve_b) / self.total_liquidity;
        
        // Update reserves
        self.reserve_a -= amount_a;
        self.reserve_b -= amount_b;
        self.total_liquidity -= liquidity_amount;
        
        // Update provider's liquidity
        *self.liquidity_providers.get_mut(provider).unwrap() -= liquidity_amount;
        
        Ok((amount_a, amount_b))
    }
    
    pub fn swap_a_for_b(&mut self, amount_a_in: u64) -> Result<u64, &'static str> {
        if amount_a_in == 0 {
            return Err("Invalid input amount");
        }
        
        // Constant product formula: x * y = k
        // amount_out = (amount_in * reserve_out) / (reserve_in + amount_in)
        let amount_b_out = (amount_a_in * self.reserve_b) / (self.reserve_a + amount_a_in);
        
        if amount_b_out >= self.reserve_b {
            return Err("Insufficient liquidity");
        }
        
        // Update reserves
        self.reserve_a += amount_a_in;
        self.reserve_b -= amount_b_out;
        
        Ok(amount_b_out)
    }
    
    pub fn swap_b_for_a(&mut self, amount_b_in: u64) -> Result<u64, &'static str> {
        if amount_b_in == 0 {
            return Err("Invalid input amount");
        }
        
        let amount_a_out = (amount_b_in * self.reserve_a) / (self.reserve_b + amount_b_in);
        
        if amount_a_out >= self.reserve_a {
            return Err("Insufficient liquidity");
        }
        
        self.reserve_b += amount_b_in;
        self.reserve_a -= amount_a_out;
        
        Ok(amount_a_out)
    }
    
    pub fn get_price(&self) -> (f64, f64) {
        if self.reserve_a == 0 || self.reserve_b == 0 {
            return (0.0, 0.0);
        }
        
        let price_a_in_b = self.reserve_b as f64 / self.reserve_a as f64;
        let price_b_in_a = self.reserve_a as f64 / self.reserve_b as f64;
        
        (price_a_in_b, price_b_in_a)
    }
}

#[derive(Debug)]
pub struct DEX {
    pools: HashMap<String, LiquidityPool>,
    user_balances: HashMap<String, HashMap<String, u64>>,
}

impl DEX {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            user_balances: HashMap::new(),
        }
    }
    
    pub fn create_pool(&mut self, token_a: String, token_b: String) -> String {
        let pool_id = format!("{}-{}", token_a, token_b);
        let pool = LiquidityPool::new(token_a, token_b);
        self.pools.insert(pool_id.clone(), pool);
        pool_id
    }
    
    pub fn deposit(&mut self, user: String, token: String, amount: u64) {
        *self.user_balances
            .entry(user)
            .or_insert_with(HashMap::new)
            .entry(token)
            .or_insert(0) += amount;
    }
    
    pub fn get_balance(&self, user: &str, token: &str) -> u64 {
        self.user_balances
            .get(user)
            .and_then(|balances| balances.get(token))
            .copied()
            .unwrap_or(0)
    }
    
    pub fn provide_liquidity(
        &mut self,
        pool_id: &str,
        user: String,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<u64, &'static str> {
        let pool = self.pools.get_mut(pool_id).ok_or("Pool not found")?;
        
        // Check user balances
        let balance_a = self.get_balance(&user, &pool.token_a);
        let balance_b = self.get_balance(&user, &pool.token_b);
        
        if balance_a < amount_a || balance_b < amount_b {
            return Err("Insufficient balance");
        }
        
        // Add liquidity to pool
        let liquidity_minted = pool.add_liquidity(user.clone(), amount_a, amount_b)?;
        
        // Deduct tokens from user
        *self.user_balances.get_mut(&user).unwrap().get_mut(&pool.token_a).unwrap() -= amount_a;
        *self.user_balances.get_mut(&user).unwrap().get_mut(&pool.token_b).unwrap() -= amount_b;
        
        Ok(liquidity_minted)
    }
    
    pub fn swap(
        &mut self,
        pool_id: &str,
        user: String,
        token_in: String,
        amount_in: u64,
    ) -> Result<u64, &'static str> {
        let pool = self.pools.get_mut(pool_id).ok_or("Pool not found")?;
        
        // Check user balance
        let balance = self.get_balance(&user, &token_in);
        if balance < amount_in {
            return Err("Insufficient balance");
        }
        
        // Perform swap
        let amount_out = if token_in == pool.token_a {
            pool.swap_a_for_b(amount_in)?
        } else if token_in == pool.token_b {
            pool.swap_b_for_a(amount_in)?
        } else {
            return Err("Invalid token");
        };
        
        // Update user balances
        let token_out = if token_in == pool.token_a {
            pool.token_b.clone()
        } else {
            pool.token_a.clone()
        };
        
        *self.user_balances.get_mut(&user).unwrap().get_mut(&token_in).unwrap() -= amount_in;
        *self.user_balances.entry(user).or_insert_with(HashMap::new).entry(token_out).or_insert(0) += amount_out;
        
        Ok(amount_out)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dex = DEX::new();
    
    // Create a trading pool
    let pool_id = dex.create_pool("ETH".to_string(), "USDC".to_string());
    println!("Created pool: {}", pool_id);
    
    // Give users some tokens
    dex.deposit("Alice".to_string(), "ETH".to_string(), 1000);
    dex.deposit("Alice".to_string(), "USDC".to_string(), 2000000); // 2000 USDC
    dex.deposit("Bob".to_string(), "ETH".to_string(), 500);
    
    println!("Alice ETH balance: {}", dex.get_balance("Alice", "ETH"));
    println!("Alice USDC balance: {}", dex.get_balance("Alice", "USDC"));
    
    // Alice provides liquidity
    let liquidity = dex.provide_liquidity(&pool_id, "Alice".to_string(), 100, 200000)?;
    println!("Alice provided liquidity, received {} liquidity tokens", liquidity);
    
    // Check pool state
    if let Some(pool) = dex.pools.get(&pool_id) {
        let (price_eth, price_usdc) = pool.get_price();
        println!("Pool reserves: {} ETH, {} USDC", pool.reserve_a, pool.reserve_b);
        println!("ETH price: {} USDC, USDC price: {} ETH", price_eth, price_usdc);
    }
    
    // Bob swaps ETH for USDC
    let usdc_received = dex.swap(&pool_id, "Bob".to_string(), "ETH".to_string(), 50)?;
    println!("Bob swapped 50 ETH for {} USDC", usdc_received);
    
    println!("Bob ETH balance: {}", dex.get_balance("Bob", "ETH"));
    println!("Bob USDC balance: {}", dex.get_balance("Bob", "USDC"));
    
    // Check updated pool state
    if let Some(pool) = dex.pools.get(&pool_id) {
        let (price_eth, price_usdc) = pool.get_price();
        println!("Updated pool reserves: {} ETH, {} USDC", pool.reserve_a, pool.reserve_b);
        println!("Updated ETH price: {} USDC, USDC price: {} ETH", price_eth, price_usdc);
    }
    
    Ok(())
}
```

## Key Concepts

1. **Cryptographic Hashing**: SHA-256 for block hashes and proof of work
2. **Digital Signatures**: ECDSA for transaction authentication
3. **Merkle Trees**: Efficient transaction verification
4. **Consensus Mechanisms**: Proof of Work, Proof of Stake
5. **Smart Contracts**: Programmable blockchain logic
6. **Decentralization**: No single point of failure

## Best Practices

- Always validate inputs and signatures
- Use proper cryptographic libraries
- Implement robust error handling
- Design for scalability and efficiency
- Consider gas/fee mechanisms
- Test extensively before deployment
- Keep private keys secure
- Implement proper access controls

Blockchain development in Rust provides excellent performance and security guarantees. These examples demonstrate the core concepts needed to build blockchain applications!