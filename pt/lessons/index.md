---
layout: default
title: Lições
lang: pt
---

<div class="hero lessons-hero">
    <div class="hero-content">
        <div class="hero-logo">
            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/0f/Original_Ferris.svg/800px-Original_Ferris.svg.png" alt="Ferris o Caranguejo Rust" width="100" height="100">
        </div>
        <h1>Lições do Incrível Rust</h1>
        <p>Domine a programação Rust com nosso currículo abrangente e progressivamente estruturado. De iniciante absoluto a especialista em programação de sistemas.</p>
    </div>
</div>

<div class="learning-path">
    <h2>Sua Jornada de Aprendizado</h2>
    
    <div class="path-grid">
        <div class="path-card animate-on-scroll">
            <h3>🌱 Primeiros Passos</h3>
            <p>Ponto de entrada perfeito para novatos na programação Rust.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/pt/lessons_pt/instalacao/' | relative_url }}">Instalação & Configuração</a></li>
                <li><a href="{{ '/pt/lessons_pt/ola-mundo/' | relative_url }}">Olá Mundo</a></li>
                <li><em>Gerenciador de Pacotes Cargo</em></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🏗️ Fundamentos do Rust</h3>
            <p>Conceitos centrais que formam a base do domínio do Rust.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/pt/lessons_pt/variaveis/' | relative_url }}">Variáveis & Mutabilidade</a></li>
                <li><em>Tipos de Dados</em></li>
                <li><em>Funções & Controle de Fluxo</em></li>
                <li><em>Structs & Enums</em></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🎯 Sistema de Ownership</h3>
            <p>A abordagem revolucionária do Rust para segurança de memória sem garbage collection.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/pt/lessons/ownership/' | relative_url }}">Entendendo Ownership</a></li>
                <li><a href="{{ '/pt/lessons/referencias/' | relative_url }}">Referências & Borrowing</a></li>
                <li><a href="{{ '/pt/lessons/slices/' | relative_url }}">Slices</a></li>
                <li><em>Smart Pointers</em></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🔧 Construindo Programas Complexos</h3>
            <p>Organize e estruture aplicações maiores com confiança.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/pt/lessons/structs/' | relative_url }}">Structs & Tipos Customizados</a></li>
                <li><a href="{{ '/pt/lessons/enums/' | relative_url }}">Enums & Pattern Matching</a></li>
                <li><a href="{{ '/pt/lessons/modulos/' | relative_url }}">Módulos & Pacotes</a></li>
                <li><a href="{{ '/pt/lessons/tratamento-de-erros/' | relative_url }}">Tratamento de Erros</a></li>
            </ul>
        </div>

        <div class="path-card animate-on-scroll">
            <h3>🚀 Tópicos Avançados</h3>
            <p>Domine programação de sistemas avançada e domínios especializados.</p>
            <ul class="path-lessons">
                <li><a href="{{ '/lessons/networking-fundamentals/' | relative_url }}">Programação de Rede</a></li>
                <li><a href="{{ '/lessons/distributed-systems/' | relative_url }}">Sistemas Distribuídos</a></li>
                <li><a href="{{ '/lessons/blockchain-development/' | relative_url }}">Desenvolvimento Blockchain</a></li>
                <li><a href="{{ '/lessons/machine-learning/' | relative_url }}">Machine Learning</a></li>
            </ul>
        </div>
    </div>
</div>

<div class="learning-tips">
    <h2>Estratégia de Aprendizado</h2>
    <div class="tips-grid">
        <div class="tip-card animate-on-scroll">
            <h3>📈 Caminho Recomendado</h3>
            <ol>
                <li><strong>Primeiros Passos</strong> - Configuração do ambiente e familiarização básica</li>
                <li><strong>Fundamentos</strong> - Domine sintaxe e conceitos centrais</li>
                <li><strong>Ownership</strong> - O coração da programação Rust</li>
                <li><strong>Programas Complexos</strong> - Construção de aplicações do mundo real</li>
                <li><strong>Tópicos Avançados</strong> - Expertise em domínios especializados</li>
            </ol>
        </div>
        
        <div class="tip-card animate-on-scroll">
            <h3>💡 Dicas de Sucesso</h3>
            <ul>
                <li><strong>Pratique Diariamente</strong> - Consistência constrói maestria</li>
                <li><strong>Experimente Livremente</strong> - Todos os exemplos são interativos</li>
                <li><strong>Tome Seu Tempo</strong> - Conceitos de ownership precisam de paciência</li>
                <li><strong>Participe da Comunidade</strong> - Aprenda junto, cresça mais rápido</li>
            </ul>
        </div>
    </div>
</div>

<div class="cta-section">
    <h2>🚀 Pronto para Começar Sua Jornada?</h2>
    <p>Seja você um iniciante completo ou um desenvolvedor experiente, comece com os fundamentos e progrida no seu próprio ritmo.</p>
    <div class="cta-buttons">
        <a href="{{ '/pt/lessons_pt/instalacao/' | relative_url }}" class="cta-button primary">Começar com Instalação</a>
        <a href="{{ '/pt/lessons_pt/ola-mundo/' | relative_url }}" class="cta-button secondary">Pular para Olá Mundo</a>
    </div>
</div>

<script>
// Animações de scroll aprimoradas
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

// Interações aprimoradas dos botões CTA
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