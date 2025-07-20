---
layout: default
title: Lecciones
lang: es
---

<div class="hero lessons-hero">
    <div class="hero-content">
        <div class="hero-logo">
            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/0f/Original_Ferris.svg/800px-Original_Ferris.svg.png" alt="Ferris el Cangrejo de Rust" width="100" height="100">
        </div>
        <h1>Lecciones del Increíble Rust</h1>
        <p>Domina la programación Rust con nuestro currículo integral y progresivamente estructurado. Desde principiante absoluto hasta experto en programación de sistemas.</p>
    </div>
</div>

<div class="learning-path">
    <h2>Tu Viaje de Aprendizaje</h2>
    
    <div class="path-grid">
        <div class="path-card animate-on-scroll">
            <h3>🌱 Primeros Pasos</h3>
            <p>Punto de entrada perfecto para novatos en la programación Rust.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/es/lessons_es/instalacion/' | relative_url }}">Instalación & Configuración</a></li>
                <li><a href="{{ '/es/lessons_es/hola-mundo/' | relative_url }}">Hola Mundo</a></li>
                <li><em>Gestor de Paquetes Cargo</em></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🏗️ Fundamentos de Rust</h3>
            <p>Conceptos centrales que forman la base del dominio de Rust.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/es/lessons/variables/' | relative_url }}">Variables & Mutabilidad</a></li>
                <li><a href="{{ '/es/lessons/tipos-de-datos/' | relative_url }}">Tipos de Datos</a></li>
                <li><a href="{{ '/es/lessons/funciones/' | relative_url }}">Funciones</a></li>
                <li><a href="{{ '/es/lessons/control-de-flujo/' | relative_url }}">Control de Flujo</a></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🎯 Sistema de Ownership</h3>
            <p>El enfoque revolucionario de Rust para la seguridad de memoria sin garbage collection.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/es/lessons/ownership/' | relative_url }}">Entendiendo Ownership</a></li>
                <li><a href="{{ '/es/lessons/referencias/' | relative_url }}">Referencias & Borrowing</a></li>
                <li><a href="{{ '/es/lessons/slices/' | relative_url }}">Slices</a></li>
                <li><em>Smart Pointers</em></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🔧 Construyendo Programas Complejos</h3>
            <p>Organiza y estructura aplicaciones más grandes con confianza.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/es/lessons/structs/' | relative_url }}">Structs & Tipos Personalizados</a></li>
                <li><a href="{{ '/es/lessons/enums/' | relative_url }}">Enums & Pattern Matching</a></li>
                <li><a href="{{ '/es/lessons/modulos/' | relative_url }}">Módulos & Paquetes</a></li>
                <li><a href="{{ '/es/lessons/manejo-de-errores/' | relative_url }}">Manejo de Errores</a></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🚀 Temas Avanzados</h3>
            <p>Domina programación de sistemas avanzada y dominios especializados.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/lessons/networking-fundamentals/' | relative_url }}">Programación de Red</a></li>
                <li><a href="{{ '/lessons/distributed-systems/' | relative_url }}">Sistemas Distribuidos</a></li>
                <li><a href="{{ '/lessons/blockchain-development/' | relative_url }}">Desarrollo Blockchain</a></li>
                <li><a href="{{ '/lessons/machine-learning/' | relative_url }}">Machine Learning</a></li>
            </ul>
        </div>
    </div>
</div>

<div class="learning-tips">
    <h2>Estrategia de Aprendizaje</h2>
    <div class="tips-grid">
        <div class="tip-card animate-on-scroll">
            <h3>📈 Ruta Recomendada</h3>
            <ol>
                <li><strong>Primeros Pasos</strong> - Configuración del entorno y familiarización básica</li>
                <li><strong>Fundamentos</strong> - Domina sintaxis y conceptos centrales</li>
                <li><strong>Ownership</strong> - El corazón de la programación Rust</li>
                <li><strong>Programas Complejos</strong> - Construcción de aplicaciones del mundo real</li>
                <li><strong>Temas Avanzados</strong> - Experiencia en dominios especializados</li>
            </ol>
        </div>
        
        <div class="tip-card animate-on-scroll">
            <h3>💡 Consejos de Éxito</h3>
            <ul>
                <li><strong>Practica Diariamente</strong> - La consistencia construye maestría</li>
                <li><strong>Experimenta Libremente</strong> - Todos los ejemplos son interactivos</li>
                <li><strong>Tómate Tu Tiempo</strong> - Los conceptos de ownership necesitan paciencia</li>
                <li><strong>Únete a la Comunidad</strong> - Aprende juntos, crece más rápido</li>
            </ul>
        </div>
    </div>
</div>

<div class="cta-section">
    <h2>🚀 ¿Listo para Comenzar Tu Viaje?</h2>
    <p>Ya seas un principiante completo o un desarrollador experimentado, comienza con los fundamentos y progresa a tu propio ritmo.</p>
    <div class="cta-buttons">
        <a href="{{ '/es/lessons_es/instalacion/' | relative_url }}" class="cta-button primary">Comenzar con Instalación</a>
        <a href="{{ '/es/lessons_es/hola-mundo/' | relative_url }}" class="cta-button secondary">Saltar a Hola Mundo</a>
    </div>
</div>

<script>
// Animaciones de scroll mejoradas
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('animate');
        }
    });
}, observerOptions);

document.addEventListener('DOMContentLoaded', () => {
    const animateElements = document.querySelectorAll('.animate-on-scroll');
    animateElements.forEach(el => observer.observe(el));
});

// Interacciones mejoradas de botones CTA
document.addEventListener('DOMContentLoaded', () => {
    const ctaButtons = document.querySelectorAll('.cta-button');
    ctaButtons.forEach(button => {
        button.addEventListener('mouseenter', () => {
            button.style.transform = 'translateY(-3px) scale(1.02)';
        });
        
        button.addEventListener('mouseleave', () => {
            button.style.transform = 'translateY(0) scale(1)';
        });
    });
});
</script>