use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    pub name: String,
    pub title: String,
    pub avatar: String,
    pub summary: String,
    pub contact: Contact,
    pub skills: Vec<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub email: String,
    pub linkedin: String,
    pub github: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub period: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub degree: String,
    pub institution: String,
    pub period: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tech: Vec<String>,
    pub link: String,
}

impl Resume {
    pub async fn load_from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).await.unwrap();
        serde_json::from_str(&content).unwrap()
    }
}
