---
title: Olá Mundo
difficulty: beginner
version: 1.85.0
prev_lesson: /pt/lessons/instalacao/
prev_lesson_title: Instalação
next_lesson: /pt/lessons/cargo/
next_lesson_title: Gerenciador de Pacotes Cargo
lang: pt
---

# Olá Mundo

Vamos escrever seu primeiro programa Rust! Seguindo a tradição da programação, vamos fazer um programa simples que imprime "Olá, mundo!" na tela.

## Criando um Arquivo Rust

Primeiro, crie um novo diretório para seus projetos Rust:

```bash
mkdir projetos_rust
cd projetos_rust
```

Agora crie um arquivo chamado `main.rs`:

```bash
touch main.rs
```

Abra `main.rs` no seu editor de texto favorito e adicione o seguinte código:

```rust
fn main() {
    println!("Olá, mundo!");
}
```

Vamos analisar o que este código faz:

1. `fn main()` - Isso define uma função chamada `main`. A função `main` é especial: é o primeiro código que executa em todo programa executável Rust.
2. `println!("Olá, mundo!");` - Isso chama a macro `println!` para imprimir texto na tela. O `!` indica que é uma macro, não uma função.

## Compilando e Executando

Para executar este programa, precisamos compilá-lo primeiro. Rust é uma linguagem compilada, o que significa que precisamos traduzir nosso código fonte em código de máquina antes de podermos executá-lo.

Compile o programa:

```bash
rustc main.rs
```

Isso cria um arquivo executável. No Linux e macOS, ele é chamado `main`. No Windows, é `main.exe`.

Execute o programa:

```bash
./main        # No Linux/macOS
main.exe      # No Windows
```

Você deve ver:

```
Olá, mundo!
```

Parabéns! Você escreveu e executou seu primeiro programa Rust!

## Entendendo a Sintaxe

Vamos examinar o código mais de perto:

```rust
fn main() {
    println!("Olá, mundo!");
}
```

### Definição de Função

```rust
fn main() {
    // corpo da função
}
```

- `fn` é a palavra-chave para definir funções
- `main` é o nome da função
- `()` indica que esta função não recebe parâmetros
- `{}` contém o corpo da função

### A Macro println!

```rust
println!("Olá, mundo!");
```

- `println!` é uma macro que imprime texto no console
- O `!` nos diz que é uma macro, não uma função
- O texto dentro das aspas é chamado de string literal
- O ponto e vírgula `;` termina a declaração

## Mais Exemplos

Vamos tentar algumas variações:

### Imprimindo Múltiplas Linhas

```rust
fn main() {
    println!("Olá, mundo!");
    println!("Estou aprendendo Rust!");
    println!("Isso é empolgante!");
}
```

### Usando Variáveis

```rust
fn main() {
    let nome = "Rustáceo";
    println!("Olá, {}!", nome);
}
```

Isso introduz:
- Palavra-chave `let` para criar variáveis
- `{}` como placeholder na string
- Interpolação de strings

### Múltiplos Placeholders

```rust
fn main() {
    let linguagem = "Rust";
    let ano = 2025;
    println!("Estou aprendendo {} em {}!", linguagem, ano);
}
```

## Erros Comuns

Ao começar com Rust, fique atento a estes problemas comuns:

### Esquecendo Ponto e Vírgula

```rust
fn main() {
    println!("Olá, mundo!")  // Faltou ponto e vírgula - isso causará erro!
}
```

### Nome de Função Incorreto

```rust
fn Main() {  // 'M' maiúsculo - Rust não reconhecerá como ponto de entrada
    println!("Olá, mundo!");
}
```

A função `main` deve ter 'm' minúsculo.

### Esquecendo o Ponto de Exclamação

```rust
fn main() {
    println("Olá, mundo!");  // Faltou '!' - println é uma macro, não uma função
}
```

## Pontos Principais

- Todo programa Rust começa com uma função `main`
- `println!` é usado para imprimir no console
- Código Rust deve ser compilado antes de executar
- Declarações terminam com ponto e vírgula
- Macros são chamadas com `!`

## Próximos Passos

Agora que você pode escrever e executar programas básicos Rust, vamos aprender sobre [Cargo]({{ '/pt/lessons/cargo/' | relative_url }}), o gerenciador de pacotes e sistema de build do Rust, que tornará o gerenciamento de projetos Rust muito mais fácil.

## Experimente Você Mesmo

Antes de prosseguir, tente estes exercícios:

1. Imprima seu nome em vez de "mundo"
2. Imprima múltiplas linhas com mensagens diferentes
3. Crie variáveis para sua linguagem de programação favorita e o ano atual, então imprima-as

Boa programação! 🦀