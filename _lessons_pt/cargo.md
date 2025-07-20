---
title: "Cargo - Gerenciador de Pacotes"
difficulty: beginner
version: "1.85.0"
---

# Cargo - Gerenciador de Pacotes do Rust

O Cargo é o sistema de build e gerenciador de pacotes oficial do Rust. Ele facilita o desenvolvimento, compilação e distribuição de projetos Rust.

## O que é o Cargo?

O Cargo é uma ferramenta que:
- Gerencia dependências
- Compila seu código
- Executa testes
- Gera documentação
- Publica pacotes no crates.io

## Criando um Novo Projeto

```bash
# Criar um novo projeto binário
cargo new meu_projeto
cd meu_projeto

# Criar um novo projeto biblioteca
cargo new minha_lib --lib
```

## Estrutura de um Projeto Cargo

```
meu_projeto/
├── Cargo.toml    # Arquivo de configuração
├── src/
│   └── main.rs   # Código principal
├── tests/        # Testes de integração
└── examples/     # Exemplos
```

## Arquivo Cargo.toml

```toml
[package]
name = "meu_projeto"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
tokio-test = "0.4"
```

## Comandos Essenciais

### Compilar e Executar

```bash
# Compilar o projeto
cargo build

# Compilar em modo release (otimizado)
cargo build --release

# Executar o projeto
cargo run

# Executar com argumentos
cargo run -- arg1 arg2
```

### Gerenciar Dependências

```bash
# Adicionar uma dependência
cargo add serde

# Atualizar dependências
cargo update

# Verificar dependências
cargo tree
```

## Exemplo Prático: Projeto com Dependências

Vamos criar um projeto que faz requisições HTTP:

```rust
// Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }

// src/main.rs
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    
    let post: Post = reqwest::get(url)
        .await?
        .json()
        .await?;
    
    println!("Post: {:#?}", post);
    
    Ok(())
}
```

## Workspaces (Espaços de Trabalho)

Para projetos grandes com múltiplos pacotes:

```toml
# Cargo.toml na raiz
[workspace]
members = [
    "web-server",
    "database",
    "shared-utils"
]
```

## Perfis de Build

```toml
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
debug = false
lto = true
```

## Comandos Avançados

```bash
# Executar testes
cargo test

# Gerar documentação
cargo doc --open

# Verificar código sem compilar
cargo check

# Formatar código
cargo fmt

# Análise estática
cargo clippy

# Limpar arquivos de build
cargo clean
```

## Publicando no Crates.io

```bash
# Fazer login
cargo login

# Publicar pacote
cargo publish

# Verificar antes de publicar
cargo publish --dry-run
```

## Exemplo: Ferramenta CLI

```rust
// Cargo.toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }

// src/main.rs
use clap::Parser;

#[derive(Parser)]
#[command(name = "minha-cli")]
#[command(about = "Uma ferramenta CLI simples")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    
    #[arg(short, long, default_value = "output.txt")]
    output: String,
    
    input: String,
}

fn main() {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Modo verboso ativado");
    }
    
    println!("Processando: {}", cli.input);
    println!("Salvando em: {}", cli.output);
}
```

## Features Condicionais

```toml
[features]
default = ["networking"]
networking = ["reqwest"]
database = ["sqlx"]

[dependencies]
reqwest = { version = "0.11", optional = true }
sqlx = { version = "0.7", optional = true }
```

```rust
#[cfg(feature = "networking")]
pub mod network {
    pub fn fetch_data() {
        println!("Buscando dados da rede...");
    }
}
```

## Scripts Personalizados

```toml
# Cargo.toml
[package.metadata.scripts]
setup = "cargo build && cargo test"
```

## Dicas Importantes

1. **Use `cargo check`** para verificação rápida durante desenvolvimento
2. **Cargo.lock** deve ser commitado em aplicações, ignorado em bibliotecas
3. **Use workspaces** para projetos com múltiplos componentes
4. **Configure .gitignore** para ignorar `/target`

## Próximos Passos

- Explore mais comandos do Cargo
- Aprenda sobre testing com `cargo test`
- Pratique criando diferentes tipos de projetos
- Experimente com features condicionais

O Cargo é fundamental para produtividade em Rust. Dominar suas funcionalidades fará você muito mais eficiente no desenvolvimento!