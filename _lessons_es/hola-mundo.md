---
title: Hola Mundo
difficulty: beginner
version: 1.85.0
prev_lesson: /es/lessons/instalacion/
prev_lesson_title: Instalación
next_lesson: /es/lessons/cargo/
next_lesson_title: Gestor de Paquetes Cargo
lang: es
---

# Hola Mundo

¡Escribamos tu primer programa Rust! Siguiendo la tradición de programación, haremos un programa simple que imprime "¡Hola, mundo!" en la pantalla.

## Creando un Archivo Rust

Primero, crea un nuevo directorio para tus proyectos Rust:

```bash
mkdir proyectos_rust
cd proyectos_rust
```

Ahora crea un archivo llamado `main.rs`:

```bash
touch main.rs
```

Abre `main.rs` en tu editor de texto favorito y agrega el siguiente código:

```rust
fn main() {
    println!("¡Hola, mundo!");
}
```

Analicemos qué hace este código:

1. `fn main()` - Esto define una función llamada `main`. La función `main` es especial: es el primer código que se ejecuta en todo programa ejecutable de Rust.
2. `println!("¡Hola, mundo!");` - Esto llama a la macro `println!` para imprimir texto en la pantalla. El `!` indica que es una macro, no una función.

## Compilando y Ejecutando

Para ejecutar este programa, necesitamos compilarlo primero. Rust es un lenguaje compilado, lo que significa que necesitamos traducir nuestro código fuente a código máquina antes de poder ejecutarlo.

Compila el programa:

```bash
rustc main.rs
```

Esto crea un archivo ejecutable. En Linux y macOS, se llama `main`. En Windows, es `main.exe`.

Ejecuta el programa:

```bash
./main        # En Linux/macOS
main.exe      # En Windows
```

Deberías ver:

```
¡Hola, mundo!
```

¡Felicidades! ¡Has escrito y ejecutado tu primer programa Rust!

## Entendiendo la Sintaxis

Examinemos el código más de cerca:

```rust
fn main() {
    println!("¡Hola, mundo!");
}
```

### Definición de Función

```rust
fn main() {
    // cuerpo de la función
}
```

- `fn` es la palabra clave para definir funciones
- `main` es el nombre de la función
- `()` indica que esta función no toma parámetros
- `{}` contiene el cuerpo de la función

### La Macro println!

```rust
println!("¡Hola, mundo!");
```

- `println!` es una macro que imprime texto en la consola
- El `!` nos dice que es una macro, no una función
- El texto dentro de las comillas se llama un literal de cadena
- El punto y coma `;` termina la declaración

## Más Ejemplos

Probemos algunas variaciones:

### Imprimiendo Múltiples Líneas

```rust
fn main() {
    println!("¡Hola, mundo!");
    println!("¡Estoy aprendiendo Rust!");
    println!("¡Esto es emocionante!");
}
```

### Usando Variables

```rust
fn main() {
    let nombre = "Rustáceo";
    println!("¡Hola, {}!", nombre);
}
```

Esto introduce:
- Palabra clave `let` para crear variables
- `{}` como marcador de posición en la cadena
- Interpolación de cadenas

### Múltiples Marcadores de Posición

```rust
fn main() {
    let lenguaje = "Rust";
    let año = 2025;
    println!("¡Estoy aprendiendo {} en {}!", lenguaje, año);
}
```

## Errores Comunes

Al comenzar con Rust, ten cuidado con estos problemas comunes:

### Olvidar Punto y Coma

```rust
fn main() {
    println!("¡Hola, mundo!")  // Falta punto y coma - ¡esto causará un error!
}
```

### Nombre de Función Incorrecto

```rust
fn Main() {  // 'M' mayúscula - Rust no reconocerá esto como punto de entrada
    println!("¡Hola, mundo!");
}
```

La función `main` debe tener una 'm' minúscula.

### Olvidar el Signo de Exclamación

```rust
fn main() {
    println("¡Hola, mundo!");  // Falta '!' - println es una macro, no una función
}
```

## Puntos Clave

- Todo programa Rust comienza con una función `main`
- `println!` se usa para imprimir en la consola
- El código Rust debe compilarse antes de ejecutarse
- Las declaraciones terminan con punto y coma
- Las macros se llaman con `!`

## Siguientes Pasos

Ahora que puedes escribir y ejecutar programas básicos de Rust, aprendamos sobre [Cargo]({{ '/es/lessons/cargo/' | relative_url }}), el gestor de paquetes y sistema de construcción de Rust, que hará que gestionar proyectos Rust sea mucho más fácil.

## Inténtalo Tú Mismo

Antes de continuar, prueba estos ejercicios:

1. Imprime tu nombre en lugar de "mundo"
2. Imprime múltiples líneas con mensajes diferentes
3. Crea variables para tu lenguaje de programación favorito y el año actual, luego imprímelas

¡Feliz programación! 🦀