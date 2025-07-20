---
layout: default
title: Learn Rust Programming
---

<div class="hero">
    <div class="hero-content">
        <div class="hero-logo">
            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/0f/Original_Ferris.svg/800px-Original_Ferris.svg.png" alt="Ferris the Rust Crab" width="120" height="120">
        </div>
        <h1>Master Rust Programming</h1>
        <p>The ultimate learning platform for Rust programming. From beginner-friendly tutorials to advanced systems programming - unlock the power of safe, fast, and concurrent programming.</p>
        <a href="{{ '/lessons/installation/' | relative_url }}" class="cta-button">
            Start Your Journey
        </a>
    </div>
</div>

<div class="features">
    <div class="feature animate-on-scroll">
        <h3>🚀 Zero to Hero</h3>
        <p>Complete learning path from installation to advanced topics. No prior systems programming experience needed - just curiosity and determination.</p>
    </div>
    
    <div class="feature animate-on-scroll">
        <h3>🛠️ Hands-On Learning</h3>
        <p>Interactive code examples you can run, modify, and experiment with. Every concept comes with practical, real-world demonstrations.</p>
    </div>
    
    <div class="feature animate-on-scroll">
        <h3>🎯 Progressive Mastery</h3>
        <p>Carefully structured curriculum building from fundamentals to advanced systems programming. Master ownership, borrowing, and fearless concurrency.</p>
    </div>
    
    <div class="feature animate-on-scroll">
        <h3>🌐 Production Ready</h3>
        <p>Learn industry best practices, testing strategies, and project organization. Build confidence with real-world patterns and architectures.</p>
    </div>
    
    <div class="feature animate-on-scroll">
        <h3>🔬 Advanced Topics</h3>
        <p>Dive deep into networking, distributed systems, blockchain development, and machine learning. Push the boundaries of what's possible with Rust.</p>
    </div>
    
    <div class="feature animate-on-scroll">
        <h3>🌟 Community Driven</h3>
        <p>Open source content that evolves with the community. Contribute lessons, suggest improvements, and help others on their Rust journey.</p>
    </div>
</div>

<div class="learning-path">
<h2>Your Learning Journey</h2>

<div class="path-grid">
    <div class="path-card animate-on-scroll">
        <h3>🌱 Getting Started</h3>
        <p>Perfect entry point for newcomers to Rust programming.</p>
        <ul class="path-lessons">
            <li><a href="{{ '/lessons/installation/' | relative_url }}">Installation & Setup</a></li>
            <li><a href="{{ '/lessons/hello-world/' | relative_url }}">Hello World</a></li>
            <li><a href="{{ '/lessons/cargo/' | relative_url }}">Cargo Package Manager</a></li>
        </ul>
    </div>

    <div class="path-card animate-on-scroll">
        <h3>🏗️ Rust Fundamentals</h3>
        <p>Core concepts that form the foundation of Rust mastery.</p>
        <ul class="path-lessons">
            <li><a href="{{ '/lessons/variables/' | relative_url }}">Variables & Mutability</a></li>
            <li><a href="{{ '/lessons/data-types/' | relative_url }}">Data Types</a></li>
            <li><em>Functions & Control Flow</em></li>
            <li><em>Structs & Enums</em></li>
        </ul>
    </div>

    <div class="path-card animate-on-scroll">
        <h3>🎯 Ownership System</h3>
        <p>Rust's revolutionary approach to memory safety without garbage collection.</p>
        <ul class="path-lessons">
            <li><em>Understanding Ownership</em></li>
            <li><em>References & Borrowing</em></li>
            <li><em>Lifetimes & Slices</em></li>
            <li><em>Smart Pointers</em></li>
        </ul>
    </div>

    <div class="path-card animate-on-scroll">
        <h3>🚀 Advanced Topics</h3>
        <p>Master advanced systems programming and specialized domains.</p>
        <ul class="path-lessons">
            <li><a href="{{ '/lessons/networking-fundamentals/' | relative_url }}">Network Programming</a></li>
            <li><a href="{{ '/lessons/distributed-systems/' | relative_url }}">Distributed Systems</a></li>
            <li><a href="{{ '/lessons/blockchain-development/' | relative_url }}">Blockchain Development</a></li>
            <li><a href="{{ '/lessons/machine-learning/' | relative_url }}">Machine Learning</a></li>
        </ul>
    </div>
</div>
</div>

<div class="why-rust">
    <h2>Why Choose Rust?</h2>
    <div class="why-rust-grid">
        <div class="why-rust-item animate-on-scroll">
            <h3>⚡ Blazing Performance</h3>
            <p>Zero-cost abstractions, minimal runtime, and memory efficiency that rivals C and C++ without sacrificing safety.</p>
        </div>
        
        <div class="why-rust-item animate-on-scroll">
            <h3>🛡️ Memory Safety</h3>
            <p>Eliminate entire categories of bugs with Rust's ownership system. No null pointer dereferences, buffer overflows, or use-after-free errors.</p>
        </div>
        
        <div class="why-rust-item animate-on-scroll">
            <h3>🧵 Fearless Concurrency</h3>
            <p>Write concurrent code with confidence. Rust's type system prevents data races and ensures thread safety at compile time.</p>
        </div>
        
        <div class="why-rust-item animate-on-scroll">
            <h3>🏗️ Rich Ecosystem</h3>
            <p>Growing library ecosystem with excellent package management. From web servers to embedded systems, Rust has you covered.</p>
        </div>
        
        <div class="why-rust-item animate-on-scroll">
            <h3>🎯 Developer Experience</h3>
            <p>Helpful compiler messages, integrated testing, documentation generation, and a formatter that keeps your code clean.</p>
        </div>
        
        <div class="why-rust-item animate-on-scroll">
            <h3>💼 Industry Adoption</h3>
            <p>Used by tech giants like Microsoft, Google, Facebook, and Dropbox. From system programming to web development.</p>
        </div>
    </div>
</div>

## 🎯 Ready to Begin?

Whether you're a complete beginner or an experienced developer, our curriculum adapts to your learning style. Start with our [Installation Guide]({{ '/lessons/installation/' | relative_url }}) to set up your development environment, or dive straight into [Hello World]({{ '/lessons/hello-world/' | relative_url }}) if you're ready to code.

### 🌍 Global Community

Join developers worldwide who are building the future with Rust. Available in **English**, **Português**, and **Español** - because great programming knowledge should be accessible to everyone.

---

**Let's build something incredible together.** 🦀

<script>
// Modern scroll animations
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

// Observe all animation elements
document.addEventListener('DOMContentLoaded', () => {
    const animateElements = document.querySelectorAll('.animate-on-scroll');
    animateElements.forEach(el => observer.observe(el));
});

// Reading progress bar
window.addEventListener('scroll', () => {
    const winScroll = document.body.scrollTop || document.documentElement.scrollTop;
    const height = document.documentElement.scrollHeight - document.documentElement.clientHeight;
    const scrolled = (winScroll / height) * 100;
    
    let progressBar = document.querySelector('.reading-progress-fill');
    if (!progressBar) {
        const progressContainer = document.createElement('div');
        progressContainer.className = 'reading-progress';
        progressBar = document.createElement('div');
        progressBar.className = 'reading-progress-fill';
        progressContainer.appendChild(progressBar);
        document.body.appendChild(progressContainer);
    }
    
    progressBar.style.width = scrolled + '%';
});

// Enhanced CTA button interaction
document.addEventListener('DOMContentLoaded', () => {
    const ctaButton = document.querySelector('.cta-button');
    if (ctaButton) {
        ctaButton.addEventListener('mouseenter', () => {
            ctaButton.style.transform = 'translateY(-4px) scale(1.02)';
        });
        
        ctaButton.addEventListener('mouseleave', () => {
            ctaButton.style.transform = 'translateY(0) scale(1)';
        });
    }
});
</script>