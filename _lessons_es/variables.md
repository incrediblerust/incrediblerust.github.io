---
title: "Variables y Mutabilidad"
difficulty: beginner
version: "1.85.0"
---

# Variables y Mutabilidad en Rust

En Rust, las variables tienen características únicas que garantizan seguridad de memoria y previenen errores comunes. Exploremos cómo funcionan.

## Variables Inmutables por Defecto

```rust
fn main() {
    let x = 5;
    println!("El valor de x es: {}", x);
    
    // Esto causaría un error de compilación:
    // x = 6;  // Error: no se puede asignar dos veces a una variable inmutable
}
```

## Variables Mutables

Para hacer una variable mutable, usa la palabra clave `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("El valor de x es: {}", x);
    
    x = 6;  // Ahora esto es válido
    println!("El valor de x es: {}", x);
}
```

## Shadowing (Sombreado)

Rust permite redeclarar variables con el mismo nombre:

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);
    
    let x = x + 1;  // Shadowing
    println!("x = {}", x);
    
    {
        let x = x * 2;  // Shadowing en scope interno
        println!("El valor de x en el scope interno es: {}", x);
    }
    
    println!("x = {}", x);  // Vuelve al valor del scope externo
}
```

## Diferencia entre `mut` y Shadowing

```rust
fn main() {
    // Con shadowing puedes cambiar el tipo
    let espacios = "   ";
    let espacios = espacios.len();  // String -> usize
    
    // Con mut no puedes cambiar el tipo
    let mut espacios_mut = "   ";
    // espacios_mut = espacios_mut.len();  // Error: tipos incompatibles
    espacios_mut = "    ";  // Esto sí está bien
    
    println!("Espacios: {}", espacios);
    println!("Espacios mut: {}", espacios_mut);
}
```

## Constantes

Las constantes son siempre inmutables y deben ser anotadas con tipo:

```rust
const TRES_HORAS_EN_SEGUNDOS: u32 = 60 * 60 * 3;

fn main() {
    println!("Tres horas son {} segundos", TRES_HORAS_EN_SEGUNDOS);
    
    const MAX_PUNTOS: u32 = 100_000;
    println!("Puntuación máxima: {}", MAX_PUNTOS);
}
```

## Convenciones de Nomenclatura

```rust
fn main() {
    // Variables: snake_case
    let mi_variable = 42;
    let edad_usuario = 25;
    let nombre_completo = "Juan Pérez";
    
    // Constantes: SCREAMING_SNAKE_CASE
    const PI: f64 = 3.14159;
    const VELOCIDAD_LUZ: u32 = 299_792_458;
    
    println!("Variable: {}", mi_variable);
    println!("Constante PI: {}", PI);
}
```

## Ámbito (Scope) de Variables

```rust
fn main() {
    let x = 1;  // x válida desde aquí
    
    {
        let y = 2;  // y válida desde aquí
        println!("x: {}, y: {}", x, y);
    }  // y sale del ámbito aquí
    
    println!("x: {}", x);
    // println!("y: {}", y);  // Error: y no está en ámbito
}
```

## Inicialización de Variables

```rust
fn main() {
    let x: i32;  // Declaración sin inicialización
    
    if true {
        x = 5;   // Inicialización condicional
    } else {
        x = 10;
    }
    
    println!("x: {}", x);  // Rust verifica que x esté inicializada
    
    // Error: usar antes de inicializar
    let y: i32;
    // println!("y: {}", y);  // Error de compilación
}
```

## Patrones de Asignación

```rust
fn main() {
    // Desestructuración de tuplas
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // Ignorar valores con _
    let (a, _, c) = (4, 5, 6);
    println!("a: {}, c: {}", a, c);
    
    // Variables con el mismo patrón
    let punto = (10, 20);
    let (x_coord, y_coord) = punto;
    println!("Coordenadas: ({}, {})", x_coord, y_coord);
}
```

## Mutabilidad con Referencias

```rust
fn main() {
    let mut x = 5;
    
    // Referencia inmutable
    let r1 = &x;
    let r2 = &x;
    println!("r1: {}, r2: {}", r1, r2);
    
    // Referencia mutable (solo una a la vez)
    let r3 = &mut x;
    *r3 = 10;
    println!("r3: {}", r3);
    
    // No puedes tener referencias mutables e inmutables al mismo tiempo
    // let r4 = &x;  // Error si r3 todavía está en uso
}
```

## Variables de Entorno

```rust
use std::env;

fn main() {
    // Leer variable de entorno
    match env::var("HOME") {
        Ok(valor) => println!("HOME: {}", valor),
        Err(e) => println!("Error leyendo HOME: {}", e),
    }
    
    // Con valor por defecto
    let debug = env::var("DEBUG").unwrap_or("false".to_string());
    println!("Debug mode: {}", debug);
    
    // Establecer variable de entorno
    env::set_var("MI_VAR", "mi_valor");
    println!("MI_VAR: {}", env::var("MI_VAR").unwrap());
}
```

## Ejemplo Práctico: Contador

```rust
fn main() {
    let mut contador = 0;
    
    // Simular algunos eventos
    let eventos = vec!["click", "hover", "click", "scroll", "click"];
    
    for evento in eventos {
        match evento {
            "click" => {
                contador += 1;
                println!("Click detectado! Total: {}", contador);
            },
            "hover" => println!("Hover detectado"),
            "scroll" => println!("Scroll detectado"),
            _ => println!("Evento desconocido: {}", evento),
        }
    }
    
    println!("Total de clicks: {}", contador);
}
```

## Variables en Structs

```rust
struct Usuario {
    nombre: String,
    edad: u8,
    activo: bool,
}

fn main() {
    // Struct inmutable
    let usuario1 = Usuario {
        nombre: String::from("Alice"),
        edad: 30,
        activo: true,
    };
    
    // Struct mutable
    let mut usuario2 = Usuario {
        nombre: String::from("Bob"),
        edad: 25,
        activo: false,
    };
    
    // Modificar campo de struct mutable
    usuario2.edad = 26;
    usuario2.activo = true;
    
    println!("Usuario1: {} - {} años", usuario1.nombre, usuario1.edad);
    println!("Usuario2: {} - {} años", usuario2.nombre, usuario2.edad);
}
```

## Ejemplo Avanzado: Estado de Aplicación

```rust
use std::collections::HashMap;

struct AppState {
    usuarios_conectados: u32,
    configuracion: HashMap<String, String>,
    debug_mode: bool,
}

impl AppState {
    fn new() -> Self {
        let mut config = HashMap::new();
        config.insert("version".to_string(), "1.0.0".to_string());
        config.insert("tema".to_string(), "oscuro".to_string());
        
        AppState {
            usuarios_conectados: 0,
            configuracion: config,
            debug_mode: false,
        }
    }
    
    fn conectar_usuario(&mut self) {
        self.usuarios_conectados += 1;
        if self.debug_mode {
            println!("Usuario conectado. Total: {}", self.usuarios_conectados);
        }
    }
    
    fn activar_debug(&mut self) {
        self.debug_mode = true;
        println!("Modo debug activado");
    }
}

fn main() {
    let mut app = AppState::new();
    
    app.activar_debug();
    app.conectar_usuario();
    app.conectar_usuario();
    
    println!("Estado final: {} usuarios conectados", app.usuarios_conectados);
    println!("Configuración: {:?}", app.configuracion);
}
```

## Buenas Prácticas

1. **Preferir inmutabilidad**: Usar `mut` solo cuando sea necesario
2. **Nombres descriptivos**: `contador_usuarios` mejor que `c`
3. **Shadowing inteligente**: Para transformaciones de tipos
4. **Constantes para valores fijos**: Usar `const` para valores que no cambian

## Errores Comunes

```rust
fn main() {
    // Error: usar antes de inicializar
    let x: i32;
    // println!("{}", x);  // Error
    
    // Error: reasignar variable inmutable
    let y = 5;
    // y = 6;  // Error
    
    // Error: referencias simultáneas mutables
    let mut z = 10;
    let r1 = &mut z;
    // let r2 = &mut z;  // Error: solo una referencia mutable
    
    *r1 = 15;
    println!("z: {}", z);
}
```

Las variables en Rust pueden parecer restrictivas al principio, pero estas restricciones previenen errores comunes y hacen que tu código sea más seguro y predecible. ¡Practica con diferentes patrones para familiarizarte con el sistema!