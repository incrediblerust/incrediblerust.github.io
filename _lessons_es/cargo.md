---
title: "Cargo - Gestor de Paquetes"
difficulty: beginner
version: "1.85.0"
---

# Cargo - Gestor de Paquetes de Rust

Cargo es el sistema de construcción y gestor de paquetes oficial de Rust. Facilita el desarrollo, compilación y distribución de proyectos Rust.

## ¿Qué es Cargo?

Cargo es una herramienta que:
- Gestiona dependencias
- Compila tu código
- Ejecuta pruebas
- Genera documentación
- Publica paquetes en crates.io

## Creando un Nuevo Proyecto

```bash
# Crear un nuevo proyecto binario
cargo new mi_proyecto
cd mi_proyecto

# Crear un nuevo proyecto de biblioteca
cargo new mi_lib --lib
```

## Estructura de un Proyecto Cargo

```
mi_proyecto/
├── Cargo.toml    # Archivo de configuración
├── src/
│   └── main.rs   # Código principal
├── tests/        # Pruebas de integración
└── examples/     # Ejemplos
```

## Archivo Cargo.toml

```toml
[package]
name = "mi_proyecto"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
tokio-test = "0.4"
```

## Comandos Esenciales

### Compilar y Ejecutar

```bash
# Compilar el proyecto
cargo build

# Compilar en modo release (optimizado)
cargo build --release

# Ejecutar el proyecto
cargo run

# Ejecutar con argumentos
cargo run -- arg1 arg2
```

### Gestionar Dependencias

```bash
# Añadir una dependencia
cargo add serde

# Actualizar dependencias
cargo update

# Verificar dependencias
cargo tree
```

## Ejemplo Práctico: Proyecto con Dependencias

Vamos a crear un proyecto que hace peticiones HTTP:

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

## Workspaces (Espacios de Trabajo)

Para proyectos grandes con múltiples paquetes:

```toml
# Cargo.toml en la raíz
[workspace]
members = [
    "web-server",
    "database",
    "shared-utils"
]
```

## Perfiles de Construcción

```toml
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
debug = false
lto = true
```

## Comandos Avanzados

```bash
# Ejecutar pruebas
cargo test

# Generar documentación
cargo doc --open

# Verificar código sin compilar
cargo check

# Formatear código
cargo fmt

# Análisis estático
cargo clippy

# Limpiar archivos de construcción
cargo clean
```

## Publicando en Crates.io

```bash
# Iniciar sesión
cargo login

# Publicar paquete
cargo publish

# Verificar antes de publicar
cargo publish --dry-run
```

## Ejemplo: Herramienta CLI

```rust
// Cargo.toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }

// src/main.rs
use clap::Parser;

#[derive(Parser)]
#[command(name = "mi-cli")]
#[command(about = "Una herramienta CLI simple")]
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
        println!("Modo verbose activado");
    }
    
    println!("Procesando: {}", cli.input);
    println!("Guardando en: {}", cli.output);
}
```

## Features Condicionales

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
        println!("Obteniendo datos de la red...");
    }
}
```

## Scripts Personalizados

```toml
# Cargo.toml
[package.metadata.scripts]
setup = "cargo build && cargo test"
```

## Consejos Importantes

1. **Usa `cargo check`** para verificación rápida durante desarrollo
2. **Cargo.lock** debe ser commiteado en aplicaciones, ignorado en bibliotecas
3. **Usa workspaces** para proyectos con múltiples componentes
4. **Configura .gitignore** para ignorar `/target`

## Próximos Pasos

- Explora más comandos de Cargo
- Aprende sobre testing con `cargo test`
- Practica creando diferentes tipos de proyectos
- Experimenta con features condicionales

¡Cargo es fundamental para la productividad en Rust. Dominar sus funcionalidades te hará mucho más eficiente en el desarrollo!