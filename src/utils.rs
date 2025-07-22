use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        return Ok(());
    }

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn slug_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| slug::slugify(s))
        .unwrap_or_else(|| "untitled".to_string())
}

pub fn is_markdown_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        extension == "md" || extension == "markdown"
    } else {
        false
    }
}

pub fn is_html_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        extension == "html" || extension == "htm"
    } else {
        false
    }
}

pub fn should_exclude(path: &Path, excludes: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    
    // Always exclude common Jekyll/build files
    if path_str.starts_with("_site") 
        || path_str.starts_with(".git")
        || path_str.starts_with("Gemfile")
        || path_str.starts_with("_config.yml")
        || path_str.contains(".lock")
        || path_str.contains("node_modules")
        || path_str.contains("vendor")
        || path_str.starts_with("src/")
        || path_str.starts_with("target/")
        || path_str.ends_with("Cargo.toml")
        || path_str.contains("CLAUDE.md")
        || path_str.contains("README.md")
        || path_str.contains("LICENSE")
        || path_str.contains("TAREFAS_")
    {
        return true;
    }

    // Check user-defined excludes
    excludes.iter().any(|exclude| {
        if exclude.contains("*") {
            // Simple wildcard matching
            let pattern = exclude.replace("*", "");
            path_str.contains(&pattern)
        } else {
            path_str.starts_with(exclude)
        }
    })
}