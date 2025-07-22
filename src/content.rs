use anyhow::Result;
use gray_matter::{Matter, Pod};
use gray_matter::engine::YAML;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn pod_to_yaml_value(pod: Pod) -> Value {
    match pod {
        Pod::String(s) => Value::String(s),
        Pod::Integer(n) => Value::Number(serde_yaml::Number::from(n)),
        Pod::Float(f) => Value::Number(serde_yaml::Number::from(f as i64)), // Approximate conversion
        Pod::Boolean(b) => Value::Bool(b),
        Pod::Array(arr) => Value::Sequence(arr.into_iter().map(pod_to_yaml_value).collect()),
        Pod::Hash(map) => {
            let mut yaml_map = serde_yaml::Mapping::new();
            for (k, v) in map {
                yaml_map.insert(Value::String(k), pod_to_yaml_value(v));
            }
            Value::Mapping(yaml_map)
        }
        Pod::Null => Value::Null,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub difficulty: Option<String>,
    pub version: Option<String>,
    pub prev_lesson: Option<String>,
    pub prev_lesson_title: Option<String>,
    pub next_lesson: Option<String>,
    pub next_lesson_title: Option<String>,
    pub layout: Option<String>,
    pub lang: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContentFile {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub front_matter: FrontMatter,
    pub content: String,
    pub html_content: String,
    pub collection: Option<String>,
    pub language: String,
}

impl ContentFile {
    pub fn from_path(path: &Path, source_root: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let matter = Matter::<YAML>::new();
        let result = matter.parse(&content);

        let front_matter: FrontMatter = if let Some(data) = result.data {
            match data {
                Pod::Hash(map) => {
                    let mut fm = FrontMatter {
                        title: None,
                        difficulty: None,
                        version: None,
                        prev_lesson: None,
                        prev_lesson_title: None,
                        next_lesson: None,
                        next_lesson_title: None,
                        layout: None,
                        lang: None,
                        extra: HashMap::new(),
                    };
                    
                    for (key, value) in map {
                        match key.as_str() {
                            "title" => if let Pod::String(s) = value { fm.title = Some(s); }
                            "difficulty" => if let Pod::String(s) = value { fm.difficulty = Some(s); }
                            "version" => if let Pod::String(s) = value { fm.version = Some(s); }
                            "prev_lesson" => if let Pod::String(s) = value { fm.prev_lesson = Some(s); }
                            "prev_lesson_title" => if let Pod::String(s) = value { fm.prev_lesson_title = Some(s); }
                            "next_lesson" => if let Pod::String(s) = value { fm.next_lesson = Some(s); }
                            "next_lesson_title" => if let Pod::String(s) = value { fm.next_lesson_title = Some(s); }
                            "layout" => if let Pod::String(s) = value { fm.layout = Some(s); }
                            "lang" => if let Pod::String(s) = value { fm.lang = Some(s); }
                            _ => {
                                // Convert Pod to serde_yaml::Value for extra fields
                                let yaml_value = pod_to_yaml_value(value);
                                fm.extra.insert(key, yaml_value);
                            }
                        }
                    }
                    fm
                }
                _ => FrontMatter {
                    title: None,
                    difficulty: None,
                    version: None,
                    prev_lesson: None,
                    prev_lesson_title: None,
                    next_lesson: None,
                    next_lesson_title: None,
                    layout: None,
                    lang: None,
                    extra: HashMap::new(),
                }
            }
        } else {
            FrontMatter {
                title: None,
                difficulty: None,
                version: None,
                prev_lesson: None,
                prev_lesson_title: None,
                next_lesson: None,
                next_lesson_title: None,
                layout: None,
                lang: None,
                extra: HashMap::new(),
            }
        };

        // Convert markdown to HTML
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(&result.content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Determine collection and language from path
        let relative_path = path.strip_prefix(source_root)?.to_path_buf();
        let (collection, language) = Self::extract_collection_and_language(&relative_path);

        Ok(ContentFile {
            path: path.to_path_buf(),
            relative_path,
            front_matter,
            content: result.content,
            html_content: html_output,
            collection,
            language,
        })
    }

    fn extract_collection_and_language(path: &Path) -> (Option<String>, String) {
        let path_str = path.to_string_lossy();
        
        // Extract collection from path (e.g., _lessons, _lessons_pt, _lessons_es)
        if path_str.starts_with("_lessons_pt") {
            (Some("lessons".to_string()), "pt".to_string())
        } else if path_str.starts_with("_lessons_es") {
            (Some("lessons".to_string()), "es".to_string())
        } else if path_str.starts_with("_lessons") {
            (Some("lessons".to_string()), "en".to_string())
        } else if path_str.starts_with("pt/") {
            (None, "pt".to_string())
        } else if path_str.starts_with("es/") {
            (None, "es".to_string())
        } else {
            (None, "en".to_string())
        }
    }

    pub fn get_output_path(&self, _base_url: &str) -> String {
        let stem = self.path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("index");

        if let Some(collection) = &self.collection {
            match self.language.as_str() {
                "pt" => format!("/pt/{}/{}/", collection, stem),
                "es" => format!("/es/{}/{}/", collection, stem),
                _ => format!("/{}/{}/", collection, stem),
            }
        } else {
            match self.language.as_str() {
                "pt" => format!("/pt/{}/", stem),
                "es" => format!("/es/{}/", stem),
                _ => format!("/{}/", stem),
            }
        }
    }

    pub fn get_file_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        
        if let Some(collection) = &self.collection {
            match self.language.as_str() {
                "pt" => path.push(format!("pt/{}", collection)),
                "es" => path.push(format!("es/{}", collection)),
                _ => path.push(collection),
            }
        } else {
            match self.language.as_str() {
                "pt" => path.push("pt"),
                "es" => path.push("es"),
                _ => {}
            }
        }

        let stem = self.path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("index");
        
        path.push(stem);
        path.push("index.html");
        path
    }

    /// Get the equivalent URL for this page in other languages
    pub fn get_language_urls(&self) -> std::collections::HashMap<String, String> {
        let mut urls = std::collections::HashMap::new();
        let stem = self.path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("index");

        // Map common lesson names across languages
        let lesson_map = get_lesson_translation_map();
        
        if let Some(collection) = &self.collection {
            // For lessons, try to find equivalent content
            let en_stem = match self.language.as_str() {
                "pt" | "es" => lesson_map.get(stem).unwrap_or(&stem.to_string()).clone(),
                _ => stem.to_string(),
            };
            
            // Generate URLs for each language
            urls.insert("en".to_string(), format!("/{}/{}/", collection, en_stem));
            
            // For Portuguese, find PT equivalent or fallback to main page
            let pt_lesson = match en_stem.as_str() {
                "hello-world" => "ola-mundo",
                "installation" => "instalacao",
                "variables" => "variaveis", 
                "data-types" => "tipos-de-dados",
                "cargo" => "cargo",
                _ => {
                    // Try reverse lookup from lesson_map
                    lesson_map.iter()
                        .find(|(_, v)| *v == &en_stem)
                        .map(|(k, _)| k.as_str())
                        .unwrap_or("index")
                }
            };
            
            if pt_lesson != "index" {
                urls.insert("pt".to_string(), format!("/pt/{}/{}/", collection, pt_lesson));
            } else {
                urls.insert("pt".to_string(), "/pt/".to_string());
            }
            
            // For Spanish, find ES equivalent or fallback to main page
            let es_lesson = match en_stem.as_str() {
                "hello-world" => "hola-mundo",
                "installation" => "instalacion", 
                "variables" => "variables",
                "cargo" => "cargo",
                _ => {
                    // Try reverse lookup from lesson_map
                    lesson_map.iter()
                        .find(|(_, v)| *v == &en_stem)
                        .map(|(k, _)| k.as_str())
                        .unwrap_or("index")
                }
            };
            
            if es_lesson != "index" {
                urls.insert("es".to_string(), format!("/es/{}/{}/", collection, es_lesson));
            } else {
                urls.insert("es".to_string(), "/es/".to_string());
            }
        } else {
            // For regular pages
            if stem == "index" {
                urls.insert("en".to_string(), "/".to_string());
                urls.insert("pt".to_string(), "/pt/".to_string());
                urls.insert("es".to_string(), "/es/".to_string());
            } else {
                urls.insert("en".to_string(), format!("/{}/", stem));
                urls.insert("pt".to_string(), format!("/pt/{}/", stem));
                urls.insert("es".to_string(), format!("/es/{}/", stem));
            }
        }
        
        urls
    }
}

/// Map lesson names between languages  
fn get_lesson_translation_map() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    
    // Portuguese to English mappings
    map.insert("ola-mundo".to_string(), "hello-world".to_string());
    map.insert("instalacao".to_string(), "installation".to_string());
    map.insert("variaveis".to_string(), "variables".to_string());
    map.insert("tipos-de-dados".to_string(), "data-types".to_string());
    map.insert("cargo".to_string(), "cargo".to_string());
    
    // Spanish to English mappings  
    map.insert("hola-mundo".to_string(), "hello-world".to_string());
    map.insert("instalacion".to_string(), "installation".to_string());
    map.insert("variables".to_string(), "variables".to_string());
    map.insert("cargo".to_string(), "cargo".to_string());
    
    map
}