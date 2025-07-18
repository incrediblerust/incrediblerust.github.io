---
title: Instalando Rust
difficulty: beginner
version: 1.85.0
next_lesson: /pt/lessons/ola-mundo/
next_lesson_title: Olá Mundo
lang: pt
---

# Instalando Rust

Antes de começarmos a escrever código Rust, precisamos instalar o toolchain Rust. Rust fornece uma excelente experiência de instalação através do `rustup`, que gerencia versões do Rust e ferramentas associadas.

## Usando Rustup (Recomendado)

A maneira mais fácil de instalar Rust é através do `rustup`, o instalador oficial do Rust e ferramenta de gerenciamento de versões.

### No macOS e Linux

Abra um terminal e execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Este comando baixa e executa o script de instalação do `rustup`. Siga as instruções na tela para completar a instalação.

### No Windows

Baixe e execute o instalador de [rustup.rs](https://rustup.rs/). O instalador irá guiá-lo através do processo.

## Verificando a Instalação

Após a instalação, reinicie seu terminal e verifique se Rust foi instalado corretamente:

```bash
rustc --version
```

Você deve ver uma saída similar a:

```
rustc 1.85.0 (a28077b28 2025-01-07)
```

Também verifique se Cargo (o gerenciador de pacotes do Rust) está instalado:

```bash
cargo --version
```

Você deve ver uma saída como:

```
cargo 1.85.0 (4ad3627e8 2025-01-07)
```

## O Que É Instalado

Quando você instala Rust através do `rustup`, você obtém várias ferramentas importantes:

- **rustc**: O compilador Rust
- **cargo**: Gerenciador de pacotes e sistema de build do Rust
- **rustup**: Ferramenta para gerenciar instalações do Rust
- **Documentação da biblioteca padrão** (disponível offline)

## Atualizando Rust

Rust tem um ciclo de lançamento rápido com novas versões estáveis a cada 6 semanas. Para atualizar para a versão mais recente:

```bash
rustup update
```

## Suporte ao IDE

Para a melhor experiência de desenvolvimento, considere usar um editor com suporte ao Rust:

- **VS Code**: Instale a extensão "Rust Analyzer"
- **IntelliJ/CLion**: Use o plugin oficial do Rust
- **Vim/Neovim**: Use rust.vim ou coc-rust-analyzer
- **Emacs**: Use rust-mode

## Resolução de Problemas

### Problemas de PATH

Se você receber erros "command not found", você pode precisar adicionar o diretório bin do Cargo ao seu PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Adicione esta linha ao arquivo de configuração do seu shell (`.bashrc`, `.zshrc`, etc.) para torná-la permanente.

### Problemas Específicos do Windows

No Windows, você pode precisar instalar as Microsoft C++ Build Tools se encontrar erros do linker. O instalador do Rust irá guiá-lo através deste processo.

## Próximos Passos

Agora que Rust está instalado, vamos escrever nosso primeiro programa Rust! Continue para a lição [Olá Mundo]({{ '/pt/lessons/ola-mundo/' | relative_url }}) para começar.

## Recursos Adicionais

- [Guia Oficial de Instalação do Rust](https://forge.rust-lang.org/infra/channel-layout.html)
- [Documentação do Rustup](https://rust-lang.github.io/rustup/)
- [O Livro da Linguagem de Programação Rust](https://doc.rust-lang.org/book/)