---
title: "Network Programming Fundamentals"
difficulty: intermediate
version: "1.85.0"
---

# Network Programming Fundamentals in Rust

Rust excels at network programming with its focus on safety, performance, and concurrency. Let's explore building network applications from the ground up.

## TCP Socket Programming

### Basic TCP Server

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received: {}", request);
                
                // Echo back to client
                if let Err(e) = stream.write_all(&buffer[..bytes_read]) {
                    println!("Failed to write to client: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
    
    Ok(())
}
```

### TCP Client

```rust
use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server");
    
    // Send message
    let message = "Hello, Server!";
    stream.write_all(message.as_bytes())?;
    
    // Read response
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Server response: {}", response);
    
    Ok(())
}
```

## Async Networking with Tokio

### Async TCP Server

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                return;
            }
            Ok(n) => {
                println!("Received {} bytes", n);
                
                // Echo back
                if let Err(e) = socket.write_all(&buffer[0..n]).await {
                    println!("Failed to write to socket: {}", e);
                    return;
                }
            }
            Err(e) => {
                println!("Failed to read from socket: {}", e);
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Async server listening on 127.0.0.1:8080");
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client: {}", addr);
        
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}
```

## HTTP Client Implementation

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse URL (simplified)
    let host = "httpbin.org";
    let path = "/get";
    let port = 80;
    
    // Connect to server
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
    
    // Prepare HTTP request
    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         \r\n",
        path, host
    );
    
    // Send request
    stream.write_all(request.as_bytes()).await?;
    
    // Read response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = http_get("http://httpbin.org/get").await?;
    println!("Response:\n{}", response);
    Ok(())
}
```

## UDP Socket Programming

### UDP Server

```rust
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("UDP server listening on 127.0.0.1:8080");
    
    let mut buf = [0; 1024];
    
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..len]);
        println!("Received from {}: {}", addr, message);
        
        // Echo back
        let response = format!("Echo: {}", message);
        socket.send_to(response.as_bytes(), addr).await?;
    }
}
```

### UDP Client

```rust
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    socket.connect("127.0.0.1:8080").await?;
    
    // Send message
    let message = "Hello UDP Server!";
    socket.send(message.as_bytes()).await?;
    
    // Receive response
    let mut buf = [0; 1024];
    let len = socket.recv(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..len]);
    println!("Server response: {}", response);
    
    Ok(())
}
```

## WebSocket Implementation

```rust
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://echo.websocket.org";
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to WebSocket server");
    
    let (mut write, mut read) = ws_stream.split();
    
    // Send message
    write.send(Message::Text("Hello WebSocket!".to_string())).await?;
    
    // Read response
    if let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => println!("Received: {}", text),
            Message::Binary(data) => println!("Received binary: {:?}", data),
            _ => println!("Received other message type"),
        }
    }
    
    Ok(())
}
```

## Network Protocol Implementation

### Simple Chat Protocol

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

async fn handle_client(
    stream: TcpStream,
    clients: Clients,
    mut rx: broadcast::Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream);
    let mut username = String::new();
    
    // Get username
    reader.read_line(&mut username).await?;
    username = username.trim().to_string();
    
    let (tx, _) = broadcast::channel(100);
    clients.lock().await.insert(username.clone(), tx.clone());
    
    println!("{} joined the chat", username);
    
    // Broadcast messages
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(message) => {
                    if let Err(_) = reader.get_mut().write_all(message.as_bytes()).await {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    
    // Read messages from client
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let message = format!("{}: {}", username, line.trim());
                
                // Broadcast to all clients
                let clients_guard = clients.lock().await;
                for (_, tx) in clients_guard.iter() {
                    let _ = tx.send(message.clone());
                }
            }
            Err(_) => break,
        }
    }
    
    // Remove client
    clients.lock().await.remove(&username);
    println!("{} left the chat", username);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let (tx, _) = broadcast::channel(100);
    
    println!("Chat server running on 127.0.0.1:8080");
    
    loop {
        let (stream, addr) = listener.accept().await?;
        let clients_clone = clients.clone();
        let rx = tx.subscribe();
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, clients_clone, rx).await {
                println!("Error handling client {}: {}", addr, e);
            }
        });
    }
}
```

## Network Security

### TLS Client

```rust
use tokio_native_tls::{TlsConnector, TlsStream};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = TlsConnector::from(native_tls::TlsConnector::new()?);
    let stream = TcpStream::connect("www.google.com:443").await?;
    let mut stream = connector.connect("www.google.com", stream).await?;
    
    // Send HTTPS request
    let request = "GET / HTTP/1.1\r\nHost: www.google.com\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes()).await?;
    
    // Read response
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;
    
    println!("Response: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
```

## Load Balancer Example

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct LoadBalancer {
    backends: Vec<String>,
    current: AtomicUsize,
}

impl LoadBalancer {
    fn new(backends: Vec<String>) -> Self {
        Self {
            backends,
            current: AtomicUsize::new(0),
        }
    }
    
    fn next_backend(&self) -> &str {
        let index = self.current.fetch_add(1, Ordering::Relaxed) % self.backends.len();
        &self.backends[index]
    }
}

async fn proxy_request(
    mut client: TcpStream,
    balancer: Arc<LoadBalancer>,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = balancer.next_backend();
    let mut backend_stream = TcpStream::connect(backend).await?;
    
    // Copy data bidirectionally
    let (mut client_read, mut client_write) = client.split();
    let (mut backend_read, mut backend_write) = backend_stream.split();
    
    let client_to_backend = async {
        let mut buffer = [0; 4096];
        loop {
            match client_read.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    if backend_write.write_all(&buffer[..n]).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    };
    
    let backend_to_client = async {
        let mut buffer = [0; 4096];
        loop {
            match backend_read.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    if client_write.write_all(&buffer[..n]).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    };
    
    tokio::select! {
        _ = client_to_backend => {},
        _ = backend_to_client => {},
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backends = vec![
        "127.0.0.1:8081".to_string(),
        "127.0.0.1:8082".to_string(),
        "127.0.0.1:8083".to_string(),
    ];
    
    let balancer = Arc::new(LoadBalancer::new(backends));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    println!("Load balancer listening on 127.0.0.1:8080");
    
    loop {
        let (client, _) = listener.accept().await?;
        let balancer_clone = balancer.clone();
        
        tokio::spawn(async move {
            if let Err(e) = proxy_request(client, balancer_clone).await {
                eprintln!("Proxy error: {}", e);
            }
        });
    }
}
```

## Network Monitoring

```rust
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::timeout;

async fn ping_host(addr: SocketAddr) -> Result<Duration, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    let _stream = timeout(Duration::from_secs(5), TcpStream::connect(addr)).await??;
    
    Ok(start.elapsed())
}

async fn monitor_hosts(hosts: Vec<SocketAddr>) {
    loop {
        for host in &hosts {
            match ping_host(*host).await {
                Ok(duration) => {
                    println!("{} is up ({}ms)", host, duration.as_millis());
                }
                Err(e) => {
                    println!("{} is down: {}", host, e);
                }
            }
        }
        
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hosts = vec![
        "8.8.8.8:53".parse()?,
        "1.1.1.1:53".parse()?,
        "google.com:80".parse()?,
    ];
    
    monitor_hosts(hosts).await;
    
    Ok(())
}
```

## Key Concepts

1. **Async/Await**: Essential for handling many concurrent connections
2. **Error Handling**: Network operations can fail in many ways
3. **Buffering**: Efficient data transfer with proper buffer management
4. **Connection Pooling**: Reuse connections for better performance
5. **Security**: Always consider TLS and input validation

## Best Practices

- Use `tokio` for async networking
- Handle errors gracefully
- Implement proper timeouts
- Use connection pooling for clients
- Validate all input data
- Consider rate limiting
- Monitor network performance

Network programming in Rust provides excellent performance and safety guarantees. These examples form the foundation for building robust network applications!