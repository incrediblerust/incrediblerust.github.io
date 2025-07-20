---
title: "Distributed Systems with Rust"
difficulty: advanced
version: "1.85.0"
---

# Distributed Systems with Rust

Rust's ownership model and performance characteristics make it an excellent choice for building distributed systems. Let's explore patterns and implementations for distributed computing.

## Microservices Architecture

### Service Discovery

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub health_check_url: String,
    pub last_heartbeat: u64,
}

#[derive(Clone)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn register(&self, mut instance: ServiceInstance) {
        instance.id = Uuid::new_v4().to_string();
        instance.last_heartbeat = chrono::Utc::now().timestamp() as u64;
        
        let mut services = self.services.write().await;
        services.entry(instance.name.clone())
            .or_insert_with(Vec::new)
            .push(instance);
    }
    
    pub async fn discover(&self, service_name: &str) -> Vec<ServiceInstance> {
        let services = self.services.read().await;
        services.get(service_name).cloned().unwrap_or_default()
    }
    
    pub async fn heartbeat(&self, service_id: &str) {
        let mut services = self.services.write().await;
        let timestamp = chrono::Utc::now().timestamp() as u64;
        
        for instances in services.values_mut() {
            for instance in instances.iter_mut() {
                if instance.id == service_id {
                    instance.last_heartbeat = timestamp;
                    return;
                }
            }
        }
    }
    
    pub async fn cleanup_stale_services(&self, max_age_seconds: u64) {
        let mut services = self.services.write().await;
        let now = chrono::Utc::now().timestamp() as u64;
        
        for instances in services.values_mut() {
            instances.retain(|instance| {
                now - instance.last_heartbeat < max_age_seconds
            });
        }
        
        services.retain(|_, instances| !instances.is_empty());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ServiceRegistry::new();
    
    // Register a service
    let service = ServiceInstance {
        id: String::new(),
        name: "user-service".to_string(),
        host: "127.0.0.1".to_string(),
        port: 8080,
        health_check_url: "/health".to_string(),
        last_heartbeat: 0,
    };
    
    registry.register(service).await;
    
    // Discover services
    let instances = registry.discover("user-service").await;
    println!("Found {} instances of user-service", instances.len());
    
    Ok(())
}
```

### Circuit Breaker Pattern

```rust
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

pub struct CircuitBreaker {
    failure_threshold: usize,
    timeout: Duration,
    failure_count: AtomicUsize,
    last_failure_time: AtomicU64,
    state: Arc<std::sync::RwLock<CircuitState>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, timeout: Duration) -> Self {
        Self {
            failure_threshold,
            timeout,
            failure_count: AtomicUsize::new(0),
            last_failure_time: AtomicU64::new(0),
            state: Arc::new(std::sync::RwLock::new(CircuitState::Closed)),
        }
    }
    
    pub async fn call<F, T, E>(&self, f: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        // Check if circuit is open
        if self.is_open().await {
            return Err(CircuitBreakerError::CircuitOpen);
        }
        
        // Execute the function
        match f() {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(CircuitBreakerError::CallFailed(e))
            }
        }
    }
    
    async fn is_open(&self) -> bool {
        let state = *self.state.read().unwrap();
        
        match state {
            CircuitState::Open => {
                let now = Instant::now().elapsed().as_secs();
                let last_failure = self.last_failure_time.load(Ordering::Relaxed);
                
                if now - last_failure > self.timeout.as_secs() {
                    // Transition to half-open
                    *self.state.write().unwrap() = CircuitState::HalfOpen;
                    false
                } else {
                    true
                }
            }
            CircuitState::HalfOpen => false,
            CircuitState::Closed => false,
        }
    }
    
    async fn on_success(&self) {
        self.failure_count.store(0, Ordering::Relaxed);
        *self.state.write().unwrap() = CircuitState::Closed;
    }
    
    async fn on_failure(&self) {
        let failures = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
        
        if failures >= self.failure_threshold {
            *self.state.write().unwrap() = CircuitState::Open;
            let now = Instant::now().elapsed().as_secs();
            self.last_failure_time.store(now, Ordering::Relaxed);
        }
    }
}

#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    CircuitOpen,
    CallFailed(E),
}

// Example usage
async fn unreliable_service() -> Result<String, &'static str> {
    // Simulate random failures
    if rand::random::<f64>() < 0.7 {
        Err("Service failed")
    } else {
        Ok("Success".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circuit_breaker = CircuitBreaker::new(3, Duration::from_secs(5));
    
    for i in 0..10 {
        match circuit_breaker.call(|| unreliable_service()).await {
            Ok(result) => println!("Call {}: Success - {}", i, result.await?),
            Err(CircuitBreakerError::CircuitOpen) => {
                println!("Call {}: Circuit breaker is open", i);
            }
            Err(CircuitBreakerError::CallFailed(e)) => {
                println!("Call {}: Failed - {}", i, e);
            }
        }
        
        sleep(Duration::from_millis(500)).await;
    }
    
    Ok(())
}
```

## Message Queues and Event Streaming

### Simple Message Queue

```rust
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub topic: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub retry_count: usize,
}

impl Message {
    pub fn new(topic: String, payload: Vec<u8>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            topic,
            payload,
            timestamp: chrono::Utc::now().timestamp() as u64,
            retry_count: 0,
        }
    }
}

pub struct MessageQueue {
    queue: Arc<Mutex<VecDeque<Message>>>,
    notify: Arc<Notify>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            notify: Arc::new(Notify::new()),
        }
    }
    
    pub async fn publish(&self, message: Message) {
        let mut queue = self.queue.lock().await;
        queue.push_back(message);
        self.notify.notify_one();
    }
    
    pub async fn consume(&self) -> Option<Message> {
        loop {
            {
                let mut queue = self.queue.lock().await;
                if let Some(message) = queue.pop_front() {
                    return Some(message);
                }
            }
            
            // Wait for notification
            self.notify.notified().await;
        }
    }
    
    pub async fn consume_by_topic(&self, topic: &str) -> Option<Message> {
        loop {
            {
                let mut queue = self.queue.lock().await;
                if let Some(pos) = queue.iter().position(|msg| msg.topic == topic) {
                    return queue.remove(pos);
                }
            }
            
            self.notify.notified().await;
        }
    }
}

// Producer
async fn producer(queue: Arc<MessageQueue>) {
    for i in 0..10 {
        let message = Message::new(
            "orders".to_string(),
            format!("Order #{}", i).into_bytes(),
        );
        
        queue.publish(message).await;
        println!("Published order #{}", i);
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

// Consumer
async fn consumer(queue: Arc<MessageQueue>, consumer_id: usize) {
    loop {
        if let Some(message) = queue.consume().await {
            println!(
                "Consumer {} processed message: {}",
                consumer_id,
                String::from_utf8_lossy(&message.payload)
            );
            
            // Simulate processing time
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let queue = Arc::new(MessageQueue::new());
    
    // Start consumers
    let mut handles = vec![];
    for i in 0..3 {
        let queue_clone = queue.clone();
        handles.push(tokio::spawn(async move {
            consumer(queue_clone, i).await;
        }));
    }
    
    // Start producer
    let queue_clone = queue.clone();
    handles.push(tokio::spawn(async move {
        producer(queue_clone).await;
    }));
    
    // Wait for all tasks
    for handle in handles {
        handle.await?;
    }
    
    Ok(())
}
```

## Consensus Algorithms

### Simplified Raft Implementation

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaftState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRequest {
    pub term: u64,
    pub candidate_id: String,
    pub last_log_index: u64,
    pub last_log_term: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResponse {
    pub term: u64,
    pub vote_granted: bool,
}

pub struct RaftNode {
    pub id: String,
    pub state: Arc<RwLock<RaftState>>,
    pub current_term: Arc<RwLock<u64>>,
    pub voted_for: Arc<RwLock<Option<String>>>,
    pub log: Arc<RwLock<Vec<LogEntry>>>,
    pub commit_index: Arc<RwLock<u64>>,
    pub last_applied: Arc<RwLock<u64>>,
    pub peers: Vec<String>,
}

impl RaftNode {
    pub fn new(id: String, peers: Vec<String>) -> Self {
        Self {
            id,
            state: Arc::new(RwLock::new(RaftState::Follower)),
            current_term: Arc::new(RwLock::new(0)),
            voted_for: Arc::new(RwLock::new(None)),
            log: Arc::new(RwLock::new(Vec::new())),
            commit_index: Arc::new(RwLock::new(0)),
            last_applied: Arc::new(RwLock::new(0)),
            peers,
        }
    }
    
    pub async fn start_election(&self) {
        println!("Node {} starting election", self.id);
        
        // Become candidate
        *self.state.write().await = RaftState::Candidate;
        *self.current_term.write().await += 1;
        *self.voted_for.write().await = Some(self.id.clone());
        
        let term = *self.current_term.read().await;
        let log = self.log.read().await;
        let last_log_index = log.len() as u64;
        let last_log_term = log.last().map(|e| e.term).unwrap_or(0);
        
        let vote_request = VoteRequest {
            term,
            candidate_id: self.id.clone(),
            last_log_index,
            last_log_term,
        };
        
        // Request votes from peers (simplified)
        let mut votes = 1; // Vote for self
        for peer in &self.peers {
            if self.request_vote(peer, &vote_request).await {
                votes += 1;
            }
        }
        
        // Check if won election
        if votes > (self.peers.len() + 1) / 2 {
            self.become_leader().await;
        } else {
            *self.state.write().await = RaftState::Follower;
        }
    }
    
    async fn request_vote(&self, peer: &str, request: &VoteRequest) -> bool {
        // Simplified: In real implementation, this would be a network call
        println!("Requesting vote from {}", peer);
        // Simulate vote response
        true
    }
    
    async fn become_leader(&self) {
        println!("Node {} became leader for term {}", self.id, *self.current_term.read().await);
        *self.state.write().await = RaftState::Leader;
        
        // Start sending heartbeats
        self.start_heartbeat().await;
    }
    
    async fn start_heartbeat(&self) {
        let mut interval = interval(Duration::from_millis(100));
        
        loop {
            interval.tick().await;
            
            let state = self.state.read().await.clone();
            if !matches!(state, RaftState::Leader) {
                break;
            }
            
            self.send_heartbeats().await;
        }
    }
    
    async fn send_heartbeats(&self) {
        for peer in &self.peers {
            // Send heartbeat (simplified)
            println!("Sending heartbeat to {}", peer);
        }
    }
    
    pub async fn append_entry(&self, command: String) -> Result<(), &'static str> {
        let state = self.state.read().await.clone();
        if !matches!(state, RaftState::Leader) {
            return Err("Not a leader");
        }
        
        let term = *self.current_term.read().await;
        let mut log = self.log.write().await;
        let index = log.len() as u64 + 1;
        
        let entry = LogEntry {
            term,
            index,
            command,
        };
        
        log.push(entry);
        println!("Leader {} appended entry at index {}", self.id, index);
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node1 = Arc::new(RaftNode::new(
        "node1".to_string(),
        vec!["node2".to_string(), "node3".to_string()],
    ));
    
    let node2 = Arc::new(RaftNode::new(
        "node2".to_string(),
        vec!["node1".to_string(), "node3".to_string()],
    ));
    
    let node3 = Arc::new(RaftNode::new(
        "node3".to_string(),
        vec!["node1".to_string(), "node2".to_string()],
    ));
    
    // Start election on node1
    node1.start_election().await;
    
    // Append some entries
    tokio::time::sleep(Duration::from_millis(200)).await;
    node1.append_entry("SET x = 10".to_string()).await?;
    node1.append_entry("SET y = 20".to_string()).await?;
    
    // Keep running for a bit
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    Ok(())
}
```

## Distributed Caching

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Clone)]
pub struct CacheNode {
    pub id: String,
    pub host: String,
    pub port: u16,
}

pub struct ConsistentHashRing {
    nodes: Vec<CacheNode>,
    ring: Arc<RwLock<HashMap<u64, String>>>,
    virtual_nodes: usize,
}

impl ConsistentHashRing {
    pub fn new(virtual_nodes: usize) -> Self {
        Self {
            nodes: Vec::new(),
            ring: Arc::new(RwLock::new(HashMap::new())),
            virtual_nodes,
        }
    }
    
    pub async fn add_node(&mut self, node: CacheNode) {
        self.nodes.push(node.clone());
        let mut ring = self.ring.write().await;
        
        for i in 0..self.virtual_nodes {
            let key = format!("{}:{}", node.id, i);
            let hash = self.hash(&key);
            ring.insert(hash, node.id.clone());
        }
    }
    
    pub async fn remove_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n.id != node_id);
        let mut ring = self.ring.write().await;
        
        for i in 0..self.virtual_nodes {
            let key = format!("{}:{}", node_id, i);
            let hash = self.hash(&key);
            ring.remove(&hash);
        }
    }
    
    pub async fn get_node(&self, key: &str) -> Option<String> {
        let ring = self.ring.read().await;
        if ring.is_empty() {
            return None;
        }
        
        let hash = self.hash(key);
        
        // Find the first node with hash >= key hash
        let mut min_hash = u64::MAX;
        let mut selected_node = None;
        
        for (&node_hash, node_id) in ring.iter() {
            if node_hash >= hash {
                if selected_node.is_none() || node_hash < min_hash {
                    min_hash = node_hash;
                    selected_node = Some(node_id.clone());
                }
            }
        }
        
        // If no node found, wrap around to the first node
        if selected_node.is_none() {
            if let Some((_, node_id)) = ring.iter().min_by_key(|(&h, _)| h) {
                selected_node = Some(node_id.clone());
            }
        }
        
        selected_node
    }
    
    fn hash(&self, key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub value: String,
    pub ttl: u64,
    pub created_at: u64,
}

pub struct DistributedCache {
    local_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    hash_ring: Arc<RwLock<ConsistentHashRing>>,
    node_id: String,
}

impl DistributedCache {
    pub fn new(node_id: String) -> Self {
        Self {
            local_cache: Arc::new(RwLock::new(HashMap::new())),
            hash_ring: Arc::new(RwLock::new(ConsistentHashRing::new(100))),
            node_id,
        }
    }
    
    pub async fn set(&self, key: String, value: String, ttl: u64) -> Result<(), &'static str> {
        let ring = self.hash_ring.read().await;
        let target_node = ring.get_node(&key).await;
        
        if let Some(node_id) = target_node {
            if node_id == self.node_id {
                // Store locally
                let entry = CacheEntry {
                    value,
                    ttl,
                    created_at: chrono::Utc::now().timestamp() as u64,
                };
                
                let mut cache = self.local_cache.write().await;
                cache.insert(key, entry);
                Ok(())
            } else {
                // Forward to the appropriate node
                self.forward_set(&node_id, key, value, ttl).await
            }
        } else {
            Err("No nodes available")
        }
    }
    
    pub async fn get(&self, key: &str) -> Option<String> {
        let ring = self.hash_ring.read().await;
        let target_node = ring.get_node(key).await;
        
        if let Some(node_id) = target_node {
            if node_id == self.node_id {
                // Get from local cache
                let cache = self.local_cache.read().await;
                if let Some(entry) = cache.get(key) {
                    let now = chrono::Utc::now().timestamp() as u64;
                    if now - entry.created_at < entry.ttl {
                        return Some(entry.value.clone());
                    }
                }
                None
            } else {
                // Forward to the appropriate node
                self.forward_get(&node_id, key).await
            }
        } else {
            None
        }
    }
    
    async fn forward_set(&self, node_id: &str, key: String, value: String, ttl: u64) -> Result<(), &'static str> {
        // In real implementation, this would be a network call
        println!("Forwarding SET {} to node {}", key, node_id);
        Ok(())
    }
    
    async fn forward_get(&self, node_id: &str, key: &str) -> Option<String> {
        // In real implementation, this would be a network call
        println!("Forwarding GET {} to node {}", key, node_id);
        None
    }
    
    pub async fn cleanup_expired(&self) {
        let mut cache = self.local_cache.write().await;
        let now = chrono::Utc::now().timestamp() as u64;
        
        cache.retain(|_, entry| now - entry.created_at < entry.ttl);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = DistributedCache::new("node1".to_string());
    
    // Set some values
    cache.set("user:1".to_string(), "Alice".to_string(), 3600).await?;
    cache.set("user:2".to_string(), "Bob".to_string(), 3600).await?;
    
    // Get values
    if let Some(value) = cache.get("user:1").await {
        println!("user:1 = {}", value);
    }
    
    Ok(())
}
```

## Event Sourcing

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub version: u64,
    pub timestamp: u64,
}

impl Event {
    pub fn new(aggregate_id: String, event_type: String, data: serde_json::Value, version: u64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            aggregate_id,
            event_type,
            data,
            version,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
}

pub trait EventStore: Send + Sync {
    async fn append_events(&self, aggregate_id: &str, events: Vec<Event>, expected_version: u64) -> Result<(), &'static str>;
    async fn get_events(&self, aggregate_id: &str, from_version: u64) -> Vec<Event>;
}

pub struct InMemoryEventStore {
    events: Arc<RwLock<HashMap<String, Vec<Event>>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl EventStore for InMemoryEventStore {
    async fn append_events(&self, aggregate_id: &str, events: Vec<Event>, expected_version: u64) -> Result<(), &'static str> {
        let mut store = self.events.write().await;
        let aggregate_events = store.entry(aggregate_id.to_string()).or_insert_with(Vec::new);
        
        let current_version = aggregate_events.len() as u64;
        if current_version != expected_version {
            return Err("Concurrency conflict");
        }
        
        aggregate_events.extend(events);
        Ok(())
    }
    
    async fn get_events(&self, aggregate_id: &str, from_version: u64) -> Vec<Event> {
        let store = self.events.read().await;
        if let Some(events) = store.get(aggregate_id) {
            events.iter()
                .skip(from_version as usize)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub balance: f64,
    pub version: u64,
}

impl BankAccount {
    pub fn new(id: String) -> Self {
        Self {
            id,
            balance: 0.0,
            version: 0,
        }
    }
    
    pub fn apply_event(&mut self, event: &Event) {
        match event.event_type.as_str() {
            "AccountOpened" => {
                if let Ok(initial_balance) = event.data["initial_balance"].as_f64().ok_or("Invalid data") {
                    self.balance = initial_balance;
                }
            }
            "MoneyDeposited" => {
                if let Ok(amount) = event.data["amount"].as_f64().ok_or("Invalid data") {
                    self.balance += amount;
                }
            }
            "MoneyWithdrawn" => {
                if let Ok(amount) = event.data["amount"].as_f64().ok_or("Invalid data") {
                    self.balance -= amount;
                }
            }
            _ => {}
        }
        self.version = event.version;
    }
    
    pub async fn open_account(&self, initial_balance: f64, event_store: &dyn EventStore) -> Result<(), &'static str> {
        let event = Event::new(
            self.id.clone(),
            "AccountOpened".to_string(),
            serde_json::json!({"initial_balance": initial_balance}),
            1,
        );
        
        event_store.append_events(&self.id, vec![event], 0).await
    }
    
    pub async fn deposit(&self, amount: f64, event_store: &dyn EventStore) -> Result<(), &'static str> {
        if amount <= 0.0 {
            return Err("Amount must be positive");
        }
        
        let event = Event::new(
            self.id.clone(),
            "MoneyDeposited".to_string(),
            serde_json::json!({"amount": amount}),
            self.version + 1,
        );
        
        event_store.append_events(&self.id, vec![event], self.version).await
    }
    
    pub async fn withdraw(&self, amount: f64, event_store: &dyn EventStore) -> Result<(), &'static str> {
        if amount <= 0.0 {
            return Err("Amount must be positive");
        }
        
        if self.balance < amount {
            return Err("Insufficient funds");
        }
        
        let event = Event::new(
            self.id.clone(),
            "MoneyWithdrawn".to_string(),
            serde_json::json!({"amount": amount}),
            self.version + 1,
        );
        
        event_store.append_events(&self.id, vec![event], self.version).await
    }
}

pub async fn rebuild_aggregate(aggregate_id: &str, event_store: &dyn EventStore) -> BankAccount {
    let events = event_store.get_events(aggregate_id, 0).await;
    let mut account = BankAccount::new(aggregate_id.to_string());
    
    for event in events {
        account.apply_event(&event);
    }
    
    account
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_store = InMemoryEventStore::new();
    let account_id = "account-123";
    
    // Open account
    let account = BankAccount::new(account_id.to_string());
    account.open_account(100.0, &event_store).await?;
    
    // Rebuild from events
    let mut current_account = rebuild_aggregate(account_id, &event_store).await;
    println!("Account balance after opening: {}", current_account.balance);
    
    // Deposit money
    current_account.deposit(50.0, &event_store).await?;
    current_account = rebuild_aggregate(account_id, &event_store).await;
    println!("Account balance after deposit: {}", current_account.balance);
    
    // Withdraw money
    current_account.withdraw(25.0, &event_store).await?;
    current_account = rebuild_aggregate(account_id, &event_store).await;
    println!("Account balance after withdrawal: {}", current_account.balance);
    
    Ok(())
}
```

## Key Concepts for Distributed Systems

1. **CAP Theorem**: Choose between Consistency, Availability, and Partition tolerance
2. **Eventually Consistent**: Accept temporary inconsistency for availability
3. **Idempotency**: Operations should be safe to retry
4. **Circuit Breakers**: Prevent cascade failures
5. **Service Discovery**: Dynamic service location
6. **Load Balancing**: Distribute requests across instances

## Best Practices

- Design for failure - everything will fail eventually
- Use asynchronous communication where possible
- Implement proper monitoring and observability
- Plan for data consistency requirements
- Use timeouts and circuit breakers
- Design idempotent operations
- Implement proper retry mechanisms with exponential backoff

Distributed systems are complex, but Rust's safety guarantees and performance characteristics make it an excellent choice for building reliable distributed applications!