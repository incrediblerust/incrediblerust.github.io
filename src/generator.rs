use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::config::SiteConfig;
use crate::content::ContentFile;
use crate::templates::TemplateEngine;
use crate::utils::{copy_dir_recursive, ensure_dir_exists, is_markdown_file, should_exclude};

pub struct SiteGenerator {
    source_dir: PathBuf,
    output_dir: PathBuf,
    config: SiteConfig,
    template_engine: TemplateEngine,
}

impl SiteGenerator {
    pub fn new(source_dir: &str, output_dir: &str, config: SiteConfig) -> Result<Self> {
        let source_path = PathBuf::from(source_dir);
        let output_path = PathBuf::from(output_dir);
        
        let template_engine = TemplateEngine::new(&source_path)?;

        Ok(SiteGenerator {
            source_dir: source_path,
            output_dir: output_path,
            config,
            template_engine,
        })
    }

    pub async fn build(&self) -> Result<()> {
        println!("🧹 Cleaning output directory...");
        if self.output_dir.exists() {
            fs::remove_dir_all(&self.output_dir)?;
        }
        fs::create_dir_all(&self.output_dir)?;

        println!("📁 Processing content files...");
        let content_files = self.collect_content_files()?;
        
        println!("📝 Rendering {} content files...", content_files.len());
        for content in &content_files {
            self.render_content_file(content).await?;
        }

        println!("📋 Generating index pages...");
        self.generate_index_pages(&content_files).await?;

        println!("📦 Copying static assets...");
        self.copy_static_assets()?;

        println!("🎨 Creating special files...");
        self.create_special_files().await?;

        Ok(())
    }

    fn collect_content_files(&self) -> Result<Vec<ContentFile>> {
        let mut content_files = Vec::new();
        let excludes = self.config.exclude.clone().unwrap_or_default();

        for entry in WalkDir::new(&self.source_dir)
            .into_iter()
            .filter_entry(|e| !should_exclude(e.path(), &excludes))
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && (is_markdown_file(path) || path.extension().map(|e| e == "html").unwrap_or(false)) {
                if let Ok(content) = ContentFile::from_path(path, &self.source_dir) {
                    content_files.push(content);
                }
            }
        }

        Ok(content_files)
    }

    async fn render_content_file(&self, content: &ContentFile) -> Result<()> {
        let output_path = self.output_dir.join(content.get_file_path());
        ensure_dir_exists(&output_path)?;

        let rendered = self.template_engine.render_content(content, &self.config)?;
        fs::write(output_path, rendered)?;

        Ok(())
    }

    async fn generate_index_pages(&self, content_files: &[ContentFile]) -> Result<()> {
        // Generate main index pages for each language
        for lang in self.config.get_languages() {
            self.generate_language_index(&lang, content_files).await?;
            self.generate_lessons_index(&lang, content_files).await?;
        }

        Ok(())
    }

    async fn generate_language_index(&self, lang: &str, _content_files: &[ContentFile]) -> Result<()> {
        use tera::Context;
        
        let mut context = Context::new();
        context.insert("lang", lang);
        context.insert("site", &self.config);

        // Load data files for context
        if let Ok(data_dir) = fs::read_dir(self.source_dir.join("_data")) {
            for entry in data_dir.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".yml") || name.ends_with(".yaml") {
                        let content = fs::read_to_string(entry.path())?;
                        let yaml_data: serde_yaml::Value = serde_yaml::from_str(&content)?;
                        let key = name.trim_end_matches(".yml").trim_end_matches(".yaml");
                        context.insert(key, &yaml_data);
                        
                        // Add language-specific translations
                        if key == "translations" {
                            if let Some(translations) = yaml_data.get(lang) {
                                context.insert("t", translations);
                            }
                        }
                    }
                }
            }
        }

        // Note: We don't need to insert lessons here as they're handled by templates

        // Read index template from source
        let index_template_path = match lang {
            "pt" => self.source_dir.join("pt").join("index.md"),
            "es" => self.source_dir.join("es").join("index.md"),
            _ => self.source_dir.join("index.md"),
        };

        if index_template_path.exists() {
            let index_content = ContentFile::from_path(&index_template_path, &self.source_dir)?;
            let rendered = self.template_engine.render_content(&index_content, &self.config)?;
            
            let output_path = match lang {
                "pt" => self.output_dir.join("pt").join("index.html"),
                "es" => self.output_dir.join("es").join("index.html"),
                _ => self.output_dir.join("index.html"),
            };

            ensure_dir_exists(&output_path)?;
            fs::write(output_path, rendered)?;
        }

        Ok(())
    }

    async fn generate_lessons_index(&self, lang: &str, _content_files: &[ContentFile]) -> Result<()> {
        
        let lessons_template_path = match lang {
            "pt" => self.source_dir.join("pt").join("lessons").join("index.md"),
            "es" => self.source_dir.join("es").join("lessons").join("index.md"),
            _ => self.source_dir.join("lessons").join("index.md"),
        };

        if lessons_template_path.exists() {
            let lessons_content = ContentFile::from_path(&lessons_template_path, &self.source_dir)?;
            let rendered = self.template_engine.render_content(&lessons_content, &self.config)?;
            
            let output_path = match lang {
                "pt" => self.output_dir.join("pt").join("lessons").join("index.html"),
                "es" => self.output_dir.join("es").join("lessons").join("index.html"),
                _ => self.output_dir.join("lessons").join("index.html"),
            };

            ensure_dir_exists(&output_path)?;
            fs::write(output_path, rendered)?;
        }

        Ok(())
    }

    fn copy_static_assets(&self) -> Result<()> {
        // Copy assets directory
        let assets_src = self.source_dir.join("assets");
        if assets_src.exists() {
            let assets_dst = self.output_dir.join("assets");
            copy_dir_recursive(&assets_src, &assets_dst)?;
        }

        // Copy special files
        let special_files = [
            "manifest.json",
            "sw.js",
            "robots.txt",
            "sitemap.xml",
            "offline.html",
            ".nojekyll",
        ];

        for file in &special_files {
            let src = self.source_dir.join(file);
            let dst = self.output_dir.join(file);
            
            if src.exists() {
                fs::copy(src, dst)?;
            }
        }

        // Copy about pages
        for lang in self.config.get_languages() {
            let about_src = match lang.as_str() {
                "pt" => self.source_dir.join("pt").join("about.md"),
                "es" => self.source_dir.join("es").join("about.md"),
                _ => self.source_dir.join("about.md"),
            };

            if about_src.exists() {
                let about_content = ContentFile::from_path(&about_src, &self.source_dir)?;
                let rendered = self.template_engine.render_content(&about_content, &self.config)?;
                
                let output_path = match lang.as_str() {
                    "pt" => self.output_dir.join("pt").join("about").join("index.html"),
                    "es" => self.output_dir.join("es").join("about").join("index.html"),
                    _ => self.output_dir.join("about").join("index.html"),
                };

                ensure_dir_exists(&output_path)?;
                fs::write(output_path, rendered)?;
            }
        }

        Ok(())
    }

    async fn create_special_files(&self) -> Result<()> {
        // Create .nojekyll file to disable Jekyll on GitHub Pages with timestamp for cache busting
        let nojekyll_path = self.output_dir.join(".nojekyll");
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        fs::write(nojekyll_path, format!("# Generated by Rust Static Site Generator\n# Build time: {}\n", timestamp))?;

        // Generate feed.xml (basic implementation)
        self.generate_feed().await?;

        Ok(())
    }

    async fn generate_feed(&self) -> Result<()> {
        let feed_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{}</title>
    <description>{}</description>
    <link>{}</link>
    <atom:link href="{}/feed.xml" rel="self" type="application/rss+xml"/>
    <pubDate>{}</pubDate>
    <lastBuildDate>{}</lastBuildDate>
    <generator>Incredible Rust Generator</generator>
  </channel>
</rss>"#,
            self.config.title.as_deref().unwrap_or("The Incredible Rust"),
            self.config.description.as_deref().unwrap_or("Learn Rust Programming"),
            self.config.url.as_deref().unwrap_or("https://incrediblerust.github.io"),
            self.config.url.as_deref().unwrap_or("https://incrediblerust.github.io"),
            chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S %z"),
            chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S %z"),
        );

        let feed_path = self.output_dir.join("feed.xml");
        fs::write(feed_path, feed_content)?;

        Ok(())
    }
}