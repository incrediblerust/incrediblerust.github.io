---
title: "Tipos de Dados"
difficulty: beginner
version: "1.85.0"
---

# Tipos de Dados em Rust

Rust possui um sistema de tipos robusto que garante segurança de memória e previne muitos erros comuns. Vamos explorar os tipos fundamentais.

## Tipos Escalares

### Inteiros

```rust
fn main() {
    // Inteiros com sinal
    let num_i8: i8 = -128;
    let num_i16: i16 = -32_000;
    let num_i32: i32 = -2_000_000;
    let num_i64: i64 = -9_000_000_000;
    let num_i128: i128 = -170_000_000_000_000_000_000;
    
    // Inteiros sem sinal
    let num_u8: u8 = 255;
    let num_u16: u16 = 65_535;
    let num_u32: u32 = 4_000_000;
    let num_u64: u64 = 18_000_000_000;
    let num_u128: u128 = 340_000_000_000_000_000_000;
    
    // Tamanho dependente da arquitetura
    let num_isize: isize = -1000;
    let num_usize: usize = 1000;
    
    println!("i32: {}", num_i32);
    println!("u64: {}", num_u64);
}
```

### Literais Numéricos

```rust
fn main() {
    let decimal = 98_222;          // Decimal
    let hex = 0xff;                // Hexadecimal
    let octal = 0o77;              // Octal
    let binary = 0b1111_0000;      // Binário
    let byte = b'A';               // Byte (apenas u8)
    
    println!("Decimal: {}", decimal);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binário: {}", binary);
    println!("Byte: {}", byte);
}
```

### Ponto Flutuante

```rust
fn main() {
    let pi: f32 = 3.14159;         // 32-bit
    let e: f64 = 2.718281828;      // 64-bit (padrão)
    
    // Operações matemáticas
    let soma = pi + 1.0;
    let produto = e * 2.0;
    let divisao = pi / 2.0;
    
    println!("PI: {}", pi);
    println!("E: {}", e);
    println!("PI + 1: {}", soma);
}
```

### Booleano

```rust
fn main() {
    let verdadeiro: bool = true;
    let falso: bool = false;
    
    // Operações lógicas
    let e_logico = verdadeiro && falso;  // false
    let ou_logico = verdadeiro || falso; // true
    let negacao = !verdadeiro;           // false
    
    println!("Verdadeiro: {}", verdadeiro);
    println!("E lógico: {}", e_logico);
    println!("OU lógico: {}", ou_logico);
}
```

### Caractere

```rust
fn main() {
    let letra: char = 'R';
    let emoji: char = '🦀';
    let unicode: char = '\u{1F980}';   // Caranguejo em Unicode
    
    println!("Letra: {}", letra);
    println!("Emoji: {}", emoji);
    println!("Unicode: {}", unicode);
    
    // Caracteres especiais
    let nova_linha = '\n';
    let tab = '\t';
    let aspas = '\"';
    
    println!("Texto com{}tab{}e aspas{}", nova_linha, tab, aspas);
}
```

## Tipos Compostos

### Tuplas

```rust
fn main() {
    // Tupla com tipos mistos
    let pessoa: (String, i32, bool) = (
        String::from("Alice"), 
        30, 
        true
    );
    
    // Desestruturação
    let (nome, idade, ativo) = pessoa;
    
    println!("Nome: {}", nome);
    println!("Idade: {}", idade);
    println!("Ativo: {}", ativo);
    
    // Acesso por índice
    let coordenadas = (3.14, 2.71, 1.41);
    println!("X: {}", coordenadas.0);
    println!("Y: {}", coordenadas.1);
    println!("Z: {}", coordenadas.2);
}
```

### Arrays

```rust
fn main() {
    // Array com tamanho fixo
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Array inicializado com mesmo valor
    let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    
    // Acesso aos elementos
    println!("Primeiro: {}", numeros[0]);
    println!("Último: {}", numeros[4]);
    println!("Tamanho: {}", numeros.len());
    
    // Iteração
    for numero in numeros {
        println!("Número: {}", numero);
    }
    
    // Slice de array
    let fatia = &numeros[1..4];  // [2, 3, 4]
    println!("Fatia: {:?}", fatia);
}
```

## Strings

### String vs &str

```rust
fn main() {
    // String slice (imutável)
    let texto_literal: &str = "Olá, mundo!";
    
    // String (mutável, heap)
    let mut texto_string = String::from("Olá");
    texto_string.push_str(", Rust!");
    
    println!("Literal: {}", texto_literal);
    println!("String: {}", texto_string);
    
    // Conversões
    let de_literal = texto_literal.to_string();
    let para_literal = &texto_string;
    
    println!("Convertido: {}", de_literal);
    println!("Como slice: {}", para_literal);
}
```

### Operações com Strings

```rust
fn main() {
    let mut saudacao = String::new();
    
    // Adicionar texto
    saudacao.push_str("Olá");
    saudacao.push(' ');
    saudacao.push_str("mundo");
    saudacao.push('!');
    
    println!("Saudação: {}", saudacao);
    
    // Concatenação
    let nome = "Alice";
    let mensagem = format!("Bem-vinda, {}!", nome);
    println!("{}", mensagem);
    
    // Métodos úteis
    println!("Tamanho: {}", saudacao.len());
    println!("Vazia? {}", saudacao.is_empty());
    println!("Contém 'mundo'? {}", saudacao.contains("mundo"));
    
    // Iteração por caracteres
    for c in saudacao.chars() {
        println!("Char: {}", c);
    }
}
```

## Vetores (Vec)

```rust
fn main() {
    // Criar vetor
    let mut numeros = Vec::new();
    
    // Adicionar elementos
    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    
    // Macro vec!
    let frutas = vec!["maçã", "banana", "laranja"];
    
    println!("Números: {:?}", numeros);
    println!("Frutas: {:?}", frutas);
    
    // Acesso e modificação
    if let Some(primeiro) = numeros.get(0) {
        println!("Primeiro número: {}", primeiro);
    }
    
    // Iteração
    for fruta in &frutas {
        println!("Fruta: {}", fruta);
    }
    
    // Operações
    println!("Tamanho: {}", numeros.len());
    numeros.pop(); // Remove último
    println!("Após pop: {:?}", numeros);
}
```

## HashMap (Mapa)

```rust
use std::collections::HashMap;

fn main() {
    let mut pontuacoes = HashMap::new();
    
    // Inserir valores
    pontuacoes.insert(String::from("Alice"), 95);
    pontuacoes.insert(String::from("Bob"), 87);
    pontuacoes.insert(String::from("Carol"), 92);
    
    // Acessar valores
    let nome = String::from("Alice");
    match pontuacoes.get(&nome) {
        Some(pontuacao) => println!("{}: {}", nome, pontuacao),
        None => println!("{} não encontrado", nome),
    }
    
    // Iteração
    for (nome, pontuacao) in &pontuacoes {
        println!("{}: {}", nome, pontuacao);
    }
    
    // Atualizar ou inserir
    pontuacoes.entry(String::from("David")).or_insert(0);
    
    // Incrementar valor
    let contador = pontuacoes.entry(String::from("Alice")).or_insert(0);
    *contador += 5;
    
    println!("Pontuações finais: {:?}", pontuacoes);
}
```

## Option e Result

### Option<T>

```rust
fn encontrar_numero(lista: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in lista.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numeros = [1, 3, 5, 7, 9];
    
    match encontrar_numero(&numeros, 5) {
        Some(index) => println!("Encontrado no índice: {}", index),
        None => println!("Não encontrado"),
    }
    
    // Métodos úteis
    let valor: Option<i32> = Some(42);
    println!("Valor padrão: {}", valor.unwrap_or(0));
    
    if let Some(v) = valor {
        println!("Valor: {}", v);
    }
}
```

### Result<T, E>

```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Divisão por zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match dividir(10.0, 2.0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(erro) => println!("Erro: {}", erro),
    }
    
    // Operador ?
    let resultado = dividir(15.0, 3.0)?; // Propaga erro se houver
    println!("Resultado: {}", resultado);
}
```

## Conversão de Tipos

```rust
fn main() {
    // Casting explícito
    let inteiro: i32 = 42;
    let flutuante = inteiro as f64;
    let byte = inteiro as u8;
    
    println!("Inteiro: {}", inteiro);
    println!("Float: {}", flutuante);
    println!("Byte: {}", byte);
    
    // Parse de string
    let texto = "42";
    let numero: Result<i32, _> = texto.parse();
    
    match numero {
        Ok(n) => println!("Número parseado: {}", n),
        Err(e) => println!("Erro ao parsear: {}", e),
    }
    
    // From/Into traits
    let string_num = String::from("123");
    let vec_chars: Vec<char> = string_num.chars().collect();
    println!("Caracteres: {:?}", vec_chars);
}
```

## Exemplo Prático: Sistema de Inventário

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Item {
    nome: String,
    quantidade: u32,
    preco: f64,
}

fn main() {
    let mut inventario: HashMap<u32, Item> = HashMap::new();
    
    // Adicionar itens
    inventario.insert(1, Item {
        nome: String::from("Notebook"),
        quantidade: 5,
        preco: 2500.00,
    });
    
    inventario.insert(2, Item {
        nome: String::from("Mouse"),
        quantidade: 20,
        preco: 45.99,
    });
    
    // Calcular valor total
    let mut valor_total = 0.0;
    for item in inventario.values() {
        valor_total += item.quantidade as f64 * item.preco;
    }
    
    println!("Inventário: {:#?}", inventario);
    println!("Valor total: R$ {:.2}", valor_total);
    
    // Buscar item
    if let Some(item) = inventario.get(&1) {
        println!("Item encontrado: {}", item.nome);
    }
}
```

## Dicas Importantes

1. **Inferência de tipos**: Rust pode inferir tipos na maioria dos casos
2. **Imutabilidade**: Variáveis são imutáveis por padrão
3. **Shadowing**: Você pode redeclarar variáveis com o mesmo nome
4. **Overflow**: Rust verifica overflow em modo debug

O sistema de tipos do Rust é uma de suas características mais poderosas. Pratique com diferentes tipos para se familiarizar com suas nuances!