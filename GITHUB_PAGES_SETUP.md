# 🚀 GitHub Pages Setup Instructions

## ⚠️ Critical Configuration Required

After pushing these changes, you **MUST** configure GitHub Pages settings correctly:

### Step 1: Go to Repository Settings
1. Navigate to your GitHub repository
2. Click on **Settings** tab  
3. Scroll down to **Pages** section (left sidebar)

### Step 2: Configure Source
**IMPORTANT**: Change the source from "Deploy from a branch" to "GitHub Actions"

1. Under **Source**, select: **GitHub Actions** 
2. Do NOT use "Deploy from a branch" - this will trigger Jekyll

### Step 3: Verify Workflow
The correct workflow should be:
- **Name**: "Deploy with Rust Generator"
- **File**: `.github/workflows/rust-pages.yml`
- **Trigger**: Push to `main` branch

### Step 4: Wait for Deployment
After the first push:
1. Go to **Actions** tab
2. Watch for "Deploy with Rust Generator" workflow
3. It should complete in ~2-5 seconds (much faster than Jekyll!)
4. Your site will be available at: `https://username.github.io/repository-name`

## 🔧 Troubleshooting

### If Jekyll is Still Running:
**Problem**: You see logs mentioning "jekyll", "gems", "bundle", etc.
**Solution**: GitHub Pages source is still set to "Deploy from a branch"
**Fix**: Follow Step 2 above to change to "GitHub Actions"

### If Workflow Fails:
1. Check **Actions** tab for error details
2. Common issues:
   - Rust compilation errors (check `Cargo.toml`)
   - Missing permissions (workflow has correct permissions)
   - Cache issues (clear cache and retry)

### If Site Doesn't Update:
1. Verify `.nojekyll` file exists in output
2. Check that `_site/` directory was created
3. Confirm HTML files exist in `_site/`

## ✅ Success Indicators

When everything is working correctly, you should see:

### In GitHub Actions:
```
🦀 Building site with custom Rust generator
Compiling Rust generator...
Running site generation...
✅ .nojekyll file exists - Jekyll is disabled
Site built successfully!
```

### Build Time:
- **Rust Generator**: ~2-5 seconds total
- **Jekyll (old)**: ~45-60 seconds

### File Structure:
```
_site/
├── .nojekyll          ← Disables Jekyll
├── index.html         ← Homepage
├── lessons/           ← English lessons
├── pt/               ← Portuguese content  
├── es/               ← Spanish content
├── assets/           ← CSS, images
├── manifest.json     ← PWA manifest
├── sw.js            ← Service worker
└── feed.xml         ← RSS feed
```

## 🆘 Emergency Fallback

If GitHub Actions deployment fails, you can manually deploy:

1. **Run locally**:
   ```bash
   ./build.sh
   ```

2. **Create new branch**:
   ```bash
   git checkout -b gh-pages
   cp -r _site/* .
   git add .
   git commit -m "Manual deployment"
   git push origin gh-pages
   ```

3. **Set GitHub Pages source** to "Deploy from a branch: gh-pages"

## 📞 Support

If you encounter issues:
1. Check the workflow logs in **Actions** tab
2. Verify repository settings match this guide
3. Ensure `.nojekyll` exists in the root directory
4. Confirm workflow file is at `.github/workflows/rust-pages.yml`

**Remember**: The key is setting GitHub Pages source to "GitHub Actions" instead of "Deploy from a branch"!