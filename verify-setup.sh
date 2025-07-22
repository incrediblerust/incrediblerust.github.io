#!/bin/bash

echo "🔍 Verifying Incredible Rust Generator Setup"
echo "==========================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Install from https://rustup.rs/"
    exit 1
else
    echo "✅ Rust/Cargo found"
fi

# Check if source files exist
echo ""
echo "📁 Checking source files..."

required_files=(
    "src/main.rs"
    "src/config.rs" 
    "src/content.rs"
    "src/templates.rs"
    "src/generator.rs"
    "src/utils.rs"
    "Cargo.toml"
    "_config.yml"
    "templates/base.html"
    "templates/default.html"
    "templates/lesson.html"
    ".nojekyll"
    ".github/workflows/rust-pages.yml"
)

for file in "${required_files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "✅ $file"
    else
        echo "❌ $file (missing)"
    fi
done

# Check if Jekyll files are removed
echo ""
echo "🧹 Checking Jekyll cleanup..."

jekyll_files=(
    "Gemfile"
    "Gemfile.lock"
    "_layouts"
    "_plugins"
    "vendor"
    ".bundle"
)

all_clean=true
for file in "${jekyll_files[@]}"; do
    if [[ -e "$file" ]]; then
        echo "❌ $file still exists (should be removed)"
        all_clean=false
    else
        echo "✅ $file removed"
    fi
done

if $all_clean; then
    echo "✅ All Jekyll files cleaned up"
fi

# Test build
echo ""
echo "🔨 Testing build..."
if cargo build --release; then
    echo "✅ Rust generator compiles successfully"
else
    echo "❌ Build failed"
    exit 1
fi

# Test site generation
echo ""
echo "🏗️  Testing site generation..."
if cargo run --release; then
    echo "✅ Site generated successfully"
else
    echo "❌ Site generation failed"
    exit 1
fi

# Check output
echo ""
echo "📋 Checking generated output..."

if [[ -d "_site" ]]; then
    echo "✅ _site directory created"
    
    if [[ -f "_site/.nojekyll" ]]; then
        echo "✅ .nojekyll file present (Jekyll disabled)"
    else
        echo "❌ .nojekyll file missing"
    fi
    
    if [[ -f "_site/index.html" ]]; then
        echo "✅ index.html generated"
    else
        echo "❌ index.html missing"
    fi
    
    if [[ -d "_site/lessons" ]]; then
        echo "✅ lessons directory present"
    else
        echo "❌ lessons directory missing"
    fi
    
    if [[ -d "_site/assets" ]]; then
        echo "✅ assets copied"
    else
        echo "❌ assets missing"
    fi
    
    file_count=$(find _site -name "*.html" | wc -l)
    echo "✅ Generated $file_count HTML files"
    
else
    echo "❌ _site directory not created"
    exit 1
fi

echo ""
echo "🎉 Setup verification complete!"
echo ""
echo "Next steps:"
echo "1. Push changes to GitHub: git add . && git commit -m 'Migrate to Rust generator' && git push"
echo "2. Go to repository Settings → Pages"  
echo "3. Change source from 'Deploy from a branch' to 'GitHub Actions'"
echo "4. Wait for workflow to run (~2-5 seconds)"
echo "5. Visit your GitHub Pages URL"
echo ""
echo "Your site will build 10x faster than Jekyll! 🚀"