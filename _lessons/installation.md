---
title: Installing Rust
difficulty: beginner
version: 1.85.0
next_lesson: /lessons/hello-world/
next_lesson_title: Hello World
---

# Installing Rust

Before we can start writing Rust code, we need to install the Rust toolchain. Rust provides an excellent installation experience through `rustup`, which manages Rust versions and associated tools.

## Using Rustup (Recommended)

The easiest way to install Rust is through `rustup`, the official Rust installer and version management tool.

### On macOS and Linux

Open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command downloads and runs the `rustup` installer script. Follow the on-screen instructions to complete the installation.

### On Windows

Download and run the installer from [rustup.rs](https://rustup.rs/). The installer will guide you through the process.

## Verifying the Installation

After installation, restart your terminal and verify that Rust is installed correctly:

```bash
rustc --version
```

You should see output similar to:

```
rustc 1.85.0 (a28077b28 2025-01-07)
```

Also verify that Cargo (Rust's package manager) is installed:

```bash
cargo --version
```

You should see output like:

```
cargo 1.85.0 (4ad3627e8 2025-01-07)
```

## What Gets Installed

When you install Rust through `rustup`, you get several important tools:

- **rustc**: The Rust compiler
- **cargo**: Rust's package manager and build system
- **rustup**: Tool for managing Rust installations
- **Standard library documentation** (available offline)

## Updating Rust

Rust has a rapid release cycle with new stable versions every 6 weeks. To update to the latest version:

```bash
rustup update
```

## IDE Support

For the best development experience, consider using an editor with Rust support:

- **VS Code**: Install the "Rust Analyzer" extension
- **IntelliJ/CLion**: Use the official Rust plugin
- **Vim/Neovim**: Use rust.vim or coc-rust-analyzer
- **Emacs**: Use rust-mode

## Troubleshooting

### Path Issues

If you get "command not found" errors, you may need to add Cargo's bin directory to your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this line to your shell's configuration file (`.bashrc`, `.zshrc`, etc.) to make it permanent.

### Windows-Specific Issues

On Windows, you might need to install the Microsoft C++ Build Tools if you encounter linker errors. The Rust installer will guide you through this process.

## Next Steps

Now that Rust is installed, let's write our first Rust program! Continue to the [Hello World]({{ '/lessons/hello-world/' | relative_url }}) lesson to get started.

## Additional Resources

- [Official Rust Installation Guide](https://forge.rust-lang.org/infra/channel-layout.html)
- [Rustup Documentation](https://rust-lang.github.io/rustup/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)