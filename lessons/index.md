---
layout: default
title: Lessons
---

<div class="hero lessons-hero">
    <div class="hero-content">
        <div class="hero-logo">
            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/0f/Original_Ferris.svg/800px-Original_Ferris.svg.png" alt="Ferris the Rust Crab" width="100" height="100">
        </div>
        <h1>The Incredible Rust Lessons</h1>
        <p>Master Rust programming with our comprehensive, progressively structured curriculum. From absolute beginner to systems programming expert.</p>
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
            <h3>🔧 Building Complex Programs</h3>
            <p>Organize and structure larger applications with confidence.</p>
            <ul class="path-lessons">
                <li><em>Structs & Custom Types</em></li>
                <li><em>Enums & Pattern Matching</em></li>
                <li><em>Modules & Packages</em></li>
                <li><em>Error Handling</em></li>
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

<div class="learning-tips">
    <h2>Learning Strategy</h2>
    <div class="tips-grid">
        <div class="tip-card animate-on-scroll">
            <h3>📈 Recommended Path</h3>
            <ol>
                <li><strong>Getting Started</strong> - Environment setup and basic familiarity</li>
                <li><strong>Fundamentals</strong> - Master syntax and core concepts</li>
                <li><strong>Ownership</strong> - The heart of Rust programming</li>
                <li><strong>Complex Programs</strong> - Real-world application building</li>
                <li><strong>Advanced Topics</strong> - Specialized domain expertise</li>
            </ol>
        </div>
        
        <div class="tip-card animate-on-scroll">
            <h3>💡 Success Tips</h3>
            <ul>
                <li><strong>Practice Daily</strong> - Consistency builds mastery</li>
                <li><strong>Experiment Freely</strong> - All examples are interactive</li>
                <li><strong>Take Your Time</strong> - Ownership concepts need patience</li>
                <li><strong>Join Community</strong> - Learn together, grow faster</li>
            </ul>
        </div>
    </div>
</div>

<div class="cta-section">
    <h2>🚀 Ready to Begin Your Journey?</h2>
    <p>Whether you're a complete beginner or an experienced developer, start with the fundamentals and progress at your own pace.</p>
    <div class="cta-buttons">
        <a href="{{ '/lessons/installation/' | relative_url }}" class="cta-button primary">Start with Installation</a>
        <a href="{{ '/lessons/hello-world/' | relative_url }}" class="cta-button secondary">Jump to Hello World</a>
    </div>
</div>

<script>
// Enhanced scroll animations
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

// Enhanced CTA button interactions
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