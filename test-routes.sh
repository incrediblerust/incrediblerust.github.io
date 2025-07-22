#!/bin/bash

echo "🔍 Testing Site Routes and Links"
echo "================================"

# Test main pages
echo -e "\n📄 Testing Main Pages:"
pages=(
    "_site/index.html"
    "_site/pt/index.html" 
    "_site/es/index.html"
    "_site/about/index.html"
    "_site/pt/about/index.html"
    "_site/es/about/index.html"
    "_site/lessons/index.html"
    "_site/pt/lessons/index.html"
    "_site/es/lessons/index.html"
)

for page in "${pages[@]}"; do
    if [[ -f "$page" ]]; then
        echo "✅ $page"
    else
        echo "❌ $page (missing)"
    fi
done

# Test lesson pages
echo -e "\n📚 Testing Lesson Pages:"
lessons=(
    "_site/lessons/hello-world/index.html|_site/pt/lessons/ola-mundo/index.html|_site/es/lessons/hola-mundo/index.html"
    "_site/lessons/installation/index.html|_site/pt/lessons/instalacao/index.html|_site/es/lessons/instalacion/index.html"
    "_site/lessons/variables/index.html|_site/pt/lessons/variaveis/index.html|_site/es/lessons/variables/index.html"
    "_site/lessons/cargo/index.html|_site/pt/lessons/cargo/index.html|_site/es/lessons/cargo/index.html"
)

for lesson_set in "${lessons[@]}"; do
    IFS='|' read -ra LESSON_PAGES <<< "$lesson_set"
    en_page="${LESSON_PAGES[0]}"
    pt_page="${LESSON_PAGES[1]}"
    es_page="${LESSON_PAGES[2]}"
    
    echo -n "📖 $(basename $(dirname $en_page)): "
    
    if [[ -f "$en_page" ]]; then
        echo -n "EN✅ "
    else
        echo -n "EN❌ "
    fi
    
    if [[ -f "$pt_page" ]]; then
        echo -n "PT✅ "
    else
        echo -n "PT❌ "
    fi
    
    if [[ -f "$es_page" ]]; then
        echo "ES✅"
    else
        echo "ES❌"
    fi
done

# Check for broken internal links
echo -e "\n🔗 Checking Language Menu URLs:"
echo "Checking if language menu links are properly generated..."

# Test a sample page for correct language URLs
if [[ -f "_site/lessons/hello-world/index.html" ]]; then
    echo "✅ English hello-world page exists"
    
    # Check if the page contains correct PT and ES links
    if grep -q "pt/lessons/ola-mundo" "_site/lessons/hello-world/index.html"; then
        echo "✅ Portuguese link in EN page is correct"
    else
        echo "❌ Portuguese link in EN page is incorrect"
    fi
    
    if grep -q "es/lessons/hola-mundo" "_site/lessons/hello-world/index.html"; then
        echo "✅ Spanish link in EN page is correct"
    else
        echo "❌ Spanish link in EN page is incorrect"
    fi
fi

# Summary
echo -e "\n📊 Summary:"
total_files=$(find _site -name "*.html" | wc -l)
echo "Generated HTML files: $total_files"

if [[ -f "_site/.nojekyll" ]]; then
    echo "✅ .nojekyll exists (Jekyll disabled)"
else
    echo "❌ .nojekyll missing"
fi

echo -e "\n🎯 Key URLs to test:"
echo "English: /"
echo "English lessons: /lessons/"
echo "English hello-world: /lessons/hello-world/"
echo "Portuguese: /pt/"
echo "Portuguese lessons: /pt/lessons/"
echo "Portuguese hello-world: /pt/lessons/ola-mundo/"
echo "Spanish: /es/"
echo "Spanish lessons: /es/lessons/"
echo "Spanish hello-world: /es/lessons/hola-mundo/"

echo -e "\n🚀 All tests completed!"