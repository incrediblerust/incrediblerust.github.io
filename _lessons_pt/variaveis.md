---
title: Variáveis e Mutabilidade
difficulty: beginner
version: 1.85.0
prev_lesson: /pt/lessons/cargo/
prev_lesson_title: Gerenciador de Pacotes Cargo
next_lesson: /pt/lessons/tipos-de-dados/
next_lesson_title: Tipos de Dados
lang: pt
---

# Variáveis e Mutabilidade

Em Rust, variáveis funcionam de forma diferente de muitas outras linguagens de programação. Por padrão, variáveis são **imutáveis**, o que significa que seus valores não podem ser alterados depois de definidos. Esta é uma das características principais do Rust que ajuda a escrever código seguro e concorrente.

## Criando Variáveis

Você cria variáveis usando a palavra-chave `let`:

```rust
fn main() {
    let x = 5;
    println!("O valor de x é: {}", x);
}
```

## Imutabilidade por Padrão

Uma vez que um valor é vinculado a uma variável, você não pode alterá-lo:

```rust
fn main() {
    let x = 5;
    println!("O valor de x é: {}", x);
    x = 6; // Isso causará um erro de compilação!
}
```

Se você tentar compilar este código, receberá um erro:

```
error[E0384]: cannot assign twice to immutable variable `x`
```

Isso pode parecer restritivo, mas a imutabilidade tem muitos benefícios:
- Previne alterações acidentais nos dados
- Torna o código mais fácil de entender
- Permite concorrência segura
- Ajuda o compilador a otimizar seu código

## Tornando Variáveis Mutáveis

Quando você precisa alterar o valor de uma variável, use a palavra-chave `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);
}
```

Saída:
```
O valor de x é: 5
O valor de x é: 6
```

## Sombreamento de Variáveis

Rust permite que você declare uma nova variável com o mesmo nome de uma variável anterior. Isso é chamado de **sombreamento**:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("O valor de x é: {}", x); // Imprime: 12
}
```

Sombreamento é diferente de mutabilidade:

### Sombreamento vs Mutabilidade

```rust
fn main() {
    // Sombreamento - criando novas variáveis
    let espacos = "   ";           // string
    let espacos = espacos.len();    // número
    
    // Isso é permitido porque estamos criando novas variáveis
    println!("Número de espaços: {}", espacos);
}
```

Mas isso não funcionará com `mut`:

```rust
fn main() {
    let mut espacos = "   ";
    espacos = espacos.len(); // Erro! Não pode mudar o tipo
}
```

Sombreamento permite que você:
- Mude o tipo de uma variável
- Reutilize nomes de variáveis
- Mantenha variáveis imutáveis

## Constantes

Constantes são valores que são vinculados a um nome e não podem mudar. Elas diferem de variáveis imutáveis:

```rust
const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;

fn main() {
    println!("Três horas em segundos: {}", TRES_HORAS_EM_SEGUNDOS);
}
```

Principais diferenças entre constantes e variáveis:
- Constantes usam `const` em vez de `let`
- Você deve sempre anotar o tipo
- Constantes podem ser declaradas em qualquer escopo, incluindo global
- Constantes só podem ser definidas com expressões constantes, não valores de runtime
- Constantes são escritas em SCREAMING_SNAKE_CASE por convenção

## Exemplos Práticos

### Exemplo de Contador

```rust
fn main() {
    let mut contador = 0;
    
    for i in 1..=5 {
        contador += i;
        println!("Contador agora é: {}", contador);
    }
    
    println!("Contador final: {}", contador);
}
```

### Exemplo de Processamento de Nome

```rust
fn main() {
    let nome = "joão silva";
    println!("Nome original: {}", nome);
    
    // Sombrear para transformar o nome
    let nome = nome.to_uppercase();
    println!("Nome em maiúsculas: {}", nome);
    
    // Sombrear novamente para obter o comprimento
    let nome = nome.len();
    println!("Comprimento do nome: {}", nome);
}
```

### Exemplo de Configuração

```rust
const USUARIOS_MAXIMOS: u32 = 100;
const TIMEOUT_SEGUNDOS: u32 = 30;

fn main() {
    let mut usuarios_atuais = 0;
    
    // Simular adição de usuários
    for _ in 0..5 {
        usuarios_atuais += 1;
        println!("Usuários atuais: {}/{}", usuarios_atuais, USUARIOS_MAXIMOS);
    }
    
    println!("Timeout definido para {} segundos", TIMEOUT_SEGUNDOS);
}
```

## Melhores Práticas

### Use Variáveis Imutáveis Quando Possível

```rust
fn main() {
    // Bom - valor não muda
    let mensagem = "Olá, Rust!";
    println!("{}", mensagem);
    
    // Bom - só torne mutável quando necessário
    let mut contador = 0;
    contador += 1;
}
```

### Use Nomes de Variáveis Descritivos

```rust
fn main() {
    // Bom
    let idade_usuario = 25;
    let esta_logado = true;
    let email_usuario = "usuario@exemplo.com";
    
    // Evite
    let x = 25;
    let flag = true;
    let s = "usuario@exemplo.com";
}
```

### Use Constantes para Valores Fixos

```rust
const PI: f64 = 3.14159;
const NOME_EMPRESA: &str = "Rust Corp";

fn main() {
    let raio = 5.0;
    let area = PI * raio * raio;
    println!("Área: {}", area);
    println!("Bem-vindo à {}!", NOME_EMPRESA);
}
```

## Padrões Comuns

### Inicialização Depois Atribuição

```rust
fn main() {
    let x; // Declarar sem inicializar
    
    if true {
        x = 5; // Inicializar em um ramo
    } else {
        x = 10;
    }
    
    println!("x é: {}", x); // x está garantidamente inicializado aqui
}
```

### Cálculos Temporários

```rust
fn main() {
    let entrada = "42";
    
    // Analisar e lidar em etapas separadas
    let numero = entrada.parse::<i32>().unwrap_or(0);
    let resultado = numero * 2;
    
    println!("Dobro de {} é {}", numero, resultado);
}
```

## Memória e Performance

Variáveis imutáveis não adicionam overhead de runtime. A imutabilidade é aplicada em tempo de compilação, então não há custo de performance comparado a variáveis mutáveis.

```rust
fn main() {
    let x = 5;          // Sem custo de runtime para imutabilidade
    let y = x + 1;      // Computação simples
    println!("{}", y);
}
```

## Pontos Principais

- Variáveis são imutáveis por padrão em Rust
- Use `mut` para tornar variáveis mutáveis quando necessário
- Sombreamento permite reutilizar nomes de variáveis e mudar tipos
- Constantes são sempre imutáveis e devem ter tipos explícitos
- Prefira variáveis imutáveis quando possível para código mais seguro
- Imutabilidade é aplicada em tempo de compilação sem custo de runtime

## Próximos Passos

Agora que você entende variáveis e mutabilidade, vamos explorar [Tipos de Dados]({{ '/pt/lessons/tipos-de-dados/' | relative_url }}) para aprender sobre os diferentes tipos de dados que você pode armazenar em variáveis.

## Experimente Você Mesmo

1. Crie um programa que calcula a área de um retângulo usando variáveis imutáveis
2. Escreva um contador que incrementa de 1 a 10 usando uma variável mutável
3. Use sombreamento para converter uma string em maiúsculas, depois obter seu comprimento
4. Defina constantes para seu número favorito e cor, depois imprima-os

Boa programação! 🦀