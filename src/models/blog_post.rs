use crate::utils::markdown::parse_markdown;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub excerpt: String,
    pub content: String,
}

impl BlogPost {
    pub async fn load_all_from_dir(dir: &str) -> HashMap<String, Self> {
        let mut posts = HashMap::new();

        if let Ok(mut entries) = fs::read_dir(dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(extension) = entry.path().extension() {
                    if extension == "md" {
                        if let Ok(post) = Self::from_file(&entry.path()).await {
                            posts.insert(post.slug.clone(), post);
                        }
                    }
                }
            }
        }

        posts
    }

    async fn from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path).await?;
        let slug = path.file_stem().unwrap().to_string_lossy().to_string();

        let parts: Vec<&str> = content.splitn(3, "---").collect();

        if parts.len() >= 3 {
            let frontmatter: serde_json::Value = serde_yaml::from_str(parts[1])?;
            let markdown_content = parts[2].trim();

            Ok(Self {
                slug,
                title: frontmatter["title"]
                    .as_str()
                    .unwrap_or("Untitled")
                    .to_string(),
                date: frontmatter["date"].as_str().unwrap_or("").to_string(),
                excerpt: frontmatter["excerpt"].as_str().unwrap_or("").to_string(),
                content: parse_markdown(markdown_content),
            })
        } else {
            Ok(Self {
                slug,
                title: "Untitled".to_string(),
                date: "".to_string(),
                excerpt: "".to_string(),
                content: parse_markdown(&content),
            })
        }
    }
}
