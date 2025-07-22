use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: Option<String>,
    pub description: Option<String>,
    pub baseurl: Option<String>,
    pub url: Option<String>,
    pub languages: Option<Vec<String>>,
    pub default_lang: Option<String>,
    pub exclude_from_localization: Option<Vec<String>>,
    pub markdown: Option<String>,
    pub highlighter: Option<String>,
    pub permalink: Option<String>,
    pub plugins: Option<Vec<String>>,
    pub collections: Option<HashMap<String, CollectionConfig>>,
    pub defaults: Option<Vec<DefaultConfig>>,
    pub exclude: Option<Vec<String>>,
    pub kramdown: Option<KramdownConfig>,
    pub version: Option<String>,
    pub rust_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub output: Option<bool>,
    pub permalink: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub scope: DefaultScope,
    pub values: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultScope {
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub scope_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KramdownConfig {
    pub input: Option<String>,
    pub syntax_highlighter: Option<String>,
    pub syntax_highlighter_opts: Option<HashMap<String, serde_yaml::Value>>,
}

impl SiteConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: SiteConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_languages(&self) -> Vec<String> {
        self.languages
            .clone()
            .unwrap_or_else(|| vec!["en".to_string()])
    }

    pub fn get_default_lang(&self) -> String {
        self.default_lang
            .clone()
            .unwrap_or_else(|| "en".to_string())
    }

    pub fn is_collection(&self, name: &str) -> bool {
        self.collections
            .as_ref()
            .map(|c| c.contains_key(name))
            .unwrap_or(false)
    }

    pub fn get_collection_config(&self, name: &str) -> Option<&CollectionConfig> {
        self.collections.as_ref()?.get(name)
    }
}