use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct DockerEndpoint {
    #[serde(rename = "Host")]
    host: String,
    #[serde(rename = "SkipTLSVerify", skip_serializing_if = "Option::is_none")]
    skip_tls_verify: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct Endpoints {
    #[serde(rename = "docker")]
    docker: DockerEndpoint,
}

#[derive(Serialize, Deserialize)]
struct ContextMetadata {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
    metadata: Option<serde_json::Value>,
    #[serde(rename = "Endpoints")]
    endpoints: Endpoints,
}

pub fn docker_config_dir() -> PathBuf {
    let home = if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string())
    } else {
        std::env::var("HOME").unwrap_or_else(|_| "/root".to_string())
    };
    let mut path = PathBuf::from(home);
    path.push(".docker");
    path
}

fn read_config_json() -> Result<serde_json::Value, String> {
    let config_path = docker_config_dir().join("config.json");
    if !config_path.exists() {
        return Ok(serde_json::json!({}));
    }
    let content =
        std::fs::read_to_string(&config_path).map_err(|e| format!("Failed to read Docker config: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse Docker config: {}", e))
}

fn write_config_json(config: &serde_json::Value) -> Result<(), String> {
    let config_path = docker_config_dir().join("config.json");
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize Docker config: {}", e))?;
    std::fs::write(&config_path, content)
        .map_err(|e| format!("Failed to write Docker config: {}", e))?;
    Ok(())
}

pub fn read_current_context() -> Option<String> {
    let config = read_config_json().ok()?;
    config.get("currentContext")?.as_str().map(|s| s.to_string())
}

pub fn set_current_context(name: &str) -> Result<(), String> {
    let mut config = read_config_json()?;
    if let Some(obj) = config.as_object_mut() {
        obj.insert("currentContext".to_string(), serde_json::json!(name));
    }
    write_config_json(&config)
}

pub fn clear_current_context() -> Result<(), String> {
    let mut config = read_config_json()?;
    if let Some(obj) = config.as_object_mut() {
        obj.remove("currentContext");
    }
    write_config_json(&config)
}

pub fn get_current_context_endpoint() -> Option<String> {
    let name = read_current_context()?;
    let meta_path = docker_config_dir()
        .join("contexts")
        .join("meta")
        .join(&name)
        .join("meta.json");

    let content = std::fs::read_to_string(meta_path).ok()?;
    let meta: ContextMetadata = serde_json::from_str(&content).ok()?;
    Some(meta.endpoints.docker.host)
}

pub fn list_contexts() -> Result<Vec<crate::models::DockerContextInfo>, String> {
    let meta_dir = docker_config_dir().join("contexts").join("meta");
    let mut contexts = Vec::new();
    let current = read_current_context();

    if !meta_dir.exists() {
        return Ok(contexts);
    }

    let entries =
        std::fs::read_dir(&meta_dir).map_err(|e| format!("Failed to read Docker contexts directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let meta_path = entry.path().join("meta.json");

        if !meta_path.exists() {
            continue;
        }

        let content =
            std::fs::read_to_string(&meta_path).map_err(|e| format!("Failed to read context meta: {}", e))?;

        if let Ok(meta) = serde_json::from_str::<ContextMetadata>(&content) {
            let is_active = current.as_deref() == Some(&meta.name);
            contexts.push(crate::models::DockerContextInfo {
                name: meta.name,
                description: String::new(),
                docker_endpoint: meta.endpoints.docker.host,
                is_active,
            });
        }
    }

    Ok(contexts)
}

pub fn create_context(name: &str, host: &str) -> Result<(), String> {
    let meta_dir = docker_config_dir().join("contexts").join("meta").join(name);

    if name.is_empty() {
        return Err("Context name cannot be empty".to_string());
    }

    if meta_dir.join("meta.json").exists() {
        return Err(format!("Context '{}' already exists", name));
    }

    std::fs::create_dir_all(&meta_dir).map_err(|e| format!("Failed to create context directory: {}", e))?;

    let meta = ContextMetadata {
        name: name.to_string(),
        metadata: None,
        endpoints: Endpoints {
            docker: DockerEndpoint {
                host: host.to_string(),
                skip_tls_verify: None,
            },
        },
    };

    let content = serde_json::to_string_pretty(&meta)
        .map_err(|e| format!("Failed to serialize context metadata: {}", e))?;

    std::fs::write(meta_dir.join("meta.json"), content)
        .map_err(|e| format!("Failed to write context metadata: {}", e))?;

    Ok(())
}

pub fn remove_context(name: &str) -> Result<(), String> {
    let meta_dir = docker_config_dir().join("contexts").join("meta").join(name);

    if !meta_dir.exists() {
        return Err(format!("Context '{}' not found", name));
    }

    std::fs::remove_dir_all(&meta_dir).map_err(|e| format!("Failed to remove context directory: {}", e))?;

    if read_current_context().as_deref() == Some(name) {
        clear_current_context()?;
    }

    Ok(())
}
