---
title: Instalando Rust
difficulty: beginner
version: 1.85.0
next_lesson: /es/lessons/hola-mundo/
next_lesson_title: Hola Mundo
lang: es
---

# Instalando Rust

Antes de comenzar a escribir código Rust, necesitamos instalar el toolchain de Rust. Rust proporciona una excelente experiencia de instalación a través de `rustup`, que gestiona las versiones de Rust y las herramientas asociadas.

## Usando Rustup (Recomendado)

La forma más fácil de instalar Rust es a través de `rustup`, el instalador oficial de Rust y herramienta de gestión de versiones.

### En macOS y Linux

Abre una terminal y ejecuta:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Este comando descarga y ejecuta el script de instalación de `rustup`. Sigue las instrucciones en pantalla para completar la instalación.

### En Windows

Descarga y ejecuta el instalador desde [rustup.rs](https://rustup.rs/). El instalador te guiará a través del proceso.

## Verificando la Instalación

Después de la instalación, reinicia tu terminal y verifica que Rust se haya instalado correctamente:

```bash
rustc --version
```

Deberías ver una salida similar a:

```
rustc 1.85.0 (a28077b28 2025-01-07)
```

También verifica que Cargo (el gestor de paquetes de Rust) esté instalado:

```bash
cargo --version
```

Deberías ver una salida como:

```
cargo 1.85.0 (4ad3627e8 2025-01-07)
```

## Qué se Instala

Cuando instalas Rust a través de `rustup`, obtienes varias herramientas importantes:

- **rustc**: El compilador de Rust
- **cargo**: Gestor de paquetes y sistema de construcción de Rust
- **rustup**: Herramienta para gestionar instalaciones de Rust
- **Documentación de la biblioteca estándar** (disponible sin conexión)

## Actualizando Rust

Rust tiene un ciclo de lanzamiento rápido con nuevas versiones estables cada 6 semanas. Para actualizar a la versión más reciente:

```bash
rustup update
```

## Soporte del IDE

Para la mejor experiencia de desarrollo, considera usar un editor con soporte para Rust:

- **VS Code**: Instala la extensión "Rust Analyzer"
- **IntelliJ/CLion**: Usa el plugin oficial de Rust
- **Vim/Neovim**: Usa rust.vim o coc-rust-analyzer
- **Emacs**: Usa rust-mode

## Resolución de Problemas

### Problemas de PATH

Si obtienes errores "command not found", puede que necesites agregar el directorio bin de Cargo a tu PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Agrega esta línea al archivo de configuración de tu shell (`.bashrc`, `.zshrc`, etc.) para hacerlo permanente.

### Problemas Específicos de Windows

En Windows, puede que necesites instalar las Microsoft C++ Build Tools si encuentras errores del enlazador. El instalador de Rust te guiará a través de este proceso.

## Siguientes Pasos

Ahora que Rust está instalado, ¡escribamos nuestro primer programa Rust! Continúa con la lección [Hola Mundo]({{ '/es/lessons/hola-mundo/' | relative_url }}) para comenzar.

## Recursos Adicionales

- [Guía Oficial de Instalación de Rust](https://forge.rust-lang.org/infra/channel-layout.html)
- [Documentación de Rustup](https://rust-lang.github.io/rustup/)
- [El Libro del Lenguaje de Programación Rust](https://doc.rust-lang.org/book/)