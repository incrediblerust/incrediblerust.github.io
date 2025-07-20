---
layout: default
title: Funções em Rust
lang: pt
---

# Funções em Rust

Funções são blocos fundamentais na programação Rust. Elas permitem organizar código em blocos reutilizáveis e criar programas mais limpos e sustentáveis.

## O que Você Vai Aprender

- Como definir e chamar funções
- Parâmetros de função e valores de retorno
- Sintaxe baseada em expressões do Rust
- Escopo de função e ownership
- Melhores práticas para design de funções

## Fundamentos das Funções

No Rust, funções são definidas usando a palavra-chave `fn`:

```rust
fn main() {
    println!("Olá, mundo!");
    cumprimentar();
}

fn cumprimentar() {
    println!("Bem-vindo às funções do Rust!");
}
```

### Regras Principais das Funções

1. **Nomenclatura snake_case**: Use `snake_case` para nomes de funções
2. **Ordem não importa**: Funções podem ser definidas antes ou depois de serem chamadas
3. **Função main**: Todo programa executável Rust deve ter uma função `main`

## Parâmetros de Função

Funções podem aceitar parâmetros para torná-las mais flexíveis:

```rust
fn cumprimentar_pessoa(nome: &str) {
    println!("Olá, {}!", nome);
}

fn somar_numeros(x: i32, y: i32) {
    let resultado = x + y;
    println!("{} + {} = {}", x, y, resultado);
}

fn main() {
    cumprimentar_pessoa("Alice");
    somar_numeros(5, 3);
}
```

### Requisitos dos Parâmetros

- **Anotações de tipo obrigatórias**: Você deve especificar o tipo de cada parâmetro
- **Múltiplos parâmetros**: Separe com vírgulas
- **Imutáveis por padrão**: Parâmetros são imutáveis a menos que especificado de outra forma

## Valores de Retorno

Funções podem retornar valores para seus chamadores:

```rust
fn somar(x: i32, y: i32) -> i32 {
    x + y  // Sem ponto e vírgula - isto é uma expressão
}

fn multiplicar(x: i32, y: i32) -> i32 {
    return x * y;  // Retorno explícito com ponto e vírgula
}

fn obter_cumprimento() -> String {
    String::from("Olá do Rust!")
}

fn main() {
    let soma = somar(5, 3);
    let produto = multiplicar(4, 7);
    let mensagem = obter_cumprimento();
    
    println!("Soma: {}", soma);
    println!("Produto: {}", produto);
    println!("Mensagem: {}", mensagem);
}
```

### Regras dos Valores de Retorno

1. **Anotação de tipo obrigatória**: Use `->` seguido pelo tipo de retorno
2. **Expressão vs Declaração**: 
   - Última expressão sem ponto e vírgula é retornada
   - Palavra-chave `return` para retornos antecipados
3. **Tipo unit**: Funções sem valor de retorno retornam `()` (tipo unit)

## Expressões vs Declarações

Entender a diferença é crucial no Rust:

```rust
fn exemplo_expressao() -> i32 {
    let x = 5;
    let y = {
        let interno = 3;
        interno + 1  // Expressão - sem ponto e vírgula
    };
    
    x + y  // Expressão retornada - sem ponto e vírgula
}

fn exemplo_declaracao() {
    let x = 5;  // Declaração
    let y = 6;  // Declaração
    
    // Isto seria um erro:
    // let z = (let a = 6);  // Declarações não retornam valores
}
```

## Escopo de Função e Ownership

Funções no Rust seguem as regras de ownership:

```rust
fn tomar_ownership(alguma_string: String) {
    println!("{}", alguma_string);
} // alguma_string sai de escopo e é descartada

fn fazer_copia(algum_inteiro: i32) {
    println!("{}", algum_inteiro);
} // algum_inteiro sai de escopo, mas é uma cópia

fn dar_ownership() -> String {
    let alguma_string = String::from("olá");
    alguma_string // Valor de retorno transfere ownership
}

fn pegar_e_devolver(uma_string: String) -> String {
    uma_string // Valor de retorno transfere ownership
}

fn main() {
    let s = String::from("olá");
    tomar_ownership(s); // valor de s move para a função
    // s não é mais válido aqui
    
    let x = 5;
    fazer_copia(x); // x ainda seria válido depois disso
    
    let s1 = dar_ownership();
    let s2 = String::from("olá");
    let s3 = pegar_e_devolver(s2);
    // s2 foi movido para pegar_e_devolver, s3 recebe o valor de retorno
}
```

## Exemplos Práticos

### Funções de Calculadora

```rust
fn somar(a: f64, b: f64) -> f64 {
    a + b
}

fn subtrair(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}

fn dividir(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        panic!("Não é possível dividir por zero!");
    }
}

fn main() {
    let x = 10.0;
    let y = 3.0;
    
    println!("{} + {} = {}", x, y, somar(x, y));
    println!("{} - {} = {}", x, y, subtrair(x, y));
    println!("{} * {} = {}", x, y, multiplicar(x, y));
    println!("{} / {} = {}", x, y, dividir(x, y));
}
```

### Funções de Processamento de Texto

```rust
fn contar_palavras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

fn inverter_string(texto: &str) -> String {
    texto.chars().rev().collect()
}

fn eh_palindromo(texto: &str) -> bool {
    let limpo: String = texto.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    limpo == limpo.chars().rev().collect::<String>()
}

fn main() {
    let frase = "Olá mundo Rust";
    println!("'{}' tem {} palavras", frase, contar_palavras(frase));
    println!("Invertido: '{}'", inverter_string(frase));
    
    let palavra = "osso";
    println!("'{}' é palíndromo: {}", palavra, eh_palindromo(palavra));
}
```

## Melhores Práticas

1. **Mantenha funções pequenas**: Cada função deve fazer uma coisa bem
2. **Use nomes descritivos**: Nomes de função devem indicar claramente seu propósito
3. **Minimize parâmetros**: Muitos parâmetros tornam funções difíceis de usar
4. **Considere borrowing**: Use referências (`&`) para evitar transferências desnecessárias de ownership
5. **Documente funções públicas**: Use comentários `///` para documentação de API pública

## Erros Comuns

1. **Esquecer anotação de tipo de retorno**:
```rust
// Errado
fn somar(x: i32, y: i32) { x + y }

// Correto
fn somar(x: i32, y: i32) -> i32 { x + y }
```

2. **Adicionar ponto e vírgula à expressão de retorno**:
```rust
// Errado - retorna (), não i32
fn somar(x: i32, y: i32) -> i32 { x + y; }

// Correto
fn somar(x: i32, y: i32) -> i32 { x + y }
```

3. **Confusão com ownership**:
```rust
// Errado - s foi movido e não pode ser usado depois
fn main() {
    let s = String::from("olá");
    tomar_ownership(s);
    println!("{}", s); // Erro!
}

// Correto - use borrowing
fn main() {
    let s = String::from("olá");
    imprimir_string(&s);
    println!("{}", s); // Funciona!
}

fn imprimir_string(s: &String) {
    println!("{}", s);
}
```

## Experimente Você Mesmo

Crie uma função que:
1. Receba um vetor de inteiros como parâmetro
2. Retorne o maior número do vetor
3. Lide com vetores vazios graciosamente

## Próximos Passos

Agora que você entende funções, está pronto para aprender sobre:
- [Controle de Fluxo]({{ '/pt/lessons/control-flow/' | relative_url }}) - Tomando decisões no seu código
- [Structs]({{ '/pt/lessons/structs/' | relative_url }}) - Agrupando dados e funções relacionados
- [Tratamento de Erros]({{ '/pt/lessons/error-handling/' | relative_url }}) - Lidando com falhas graciosamente

Funções são os blocos de construção que vão suportar todos os conceitos avançados que você aprenderá em Rust!