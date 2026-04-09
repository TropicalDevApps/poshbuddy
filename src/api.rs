use std::fs;
use std::path::PathBuf;
use tokio::sync::mpsc;
use futures::future::join_all;
use crate::app::{AppMessage, FontAsset};

pub async fn fetch_theme_names() -> Result<Vec<String>, String> {
    let url = "https://api.github.com/repos/JanDeDobbeleer/oh-my-posh/contents/themes";
    let client = reqwest::Client::builder()
        .user_agent("PoshBuddy-Rust")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(|e| e.to_string())?;

    let themes = resp
        .into_iter()
        .filter_map(|v| v["name"].as_str().map(|s| s.to_string()))
        .filter(|s| s.ends_with(".omp.json"))
        .collect();

    Ok(themes)
}

pub async fn fetch_font_names() -> Result<Vec<FontAsset>, String> {
    let url = "https://api.github.com/repos/ryanoasis/nerd-fonts/releases/latest";
    let client = reqwest::Client::builder()
        .user_agent("PoshBuddy-Rust")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    let assets = resp["assets"]
        .as_array()
        .ok_or("No se pudieron encontrar assets en la release")?;

    let fonts = assets
        .iter()
        .filter_map(|a| {
            let raw_name = a["name"].as_str()?;
            if raw_name.ends_with(".zip") {
                Some(FontAsset {
                    name: raw_name.replace(".zip", ""),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(fonts)
}

pub async fn setup_app_task(tx: mpsc::Sender<AppMessage>, themes_dir: PathBuf) {
    // 1. Fetch de fuentes
    match fetch_font_names().await {
        Ok(fonts) => {
            let _ = tx.send(AppMessage::FontsLoaded(fonts)).await;
        }
        Err(e) => {
            let _ = tx.send(AppMessage::Error(format!("Error fuentes: {}", e))).await;
        }
    }

    // 2. Fetch de temas
    let theme_names = match fetch_theme_names().await {
        Ok(names) => names,
        Err(e) => {
            let _ = tx.send(AppMessage::Error(format!("Error temas: {}", e))).await;
            return;
        }
    };

    if !themes_dir.exists() {
        if let Err(e) = fs::create_dir_all(&themes_dir) {
            let _ = tx.send(AppMessage::Error(format!("Error creando carpeta: {}", e))).await;
            return;
        }
    }

    let client = reqwest::Client::builder()
        .user_agent("PoshBuddy-Rust")
        .build()
        .unwrap_or_default();

    let download_futures = theme_names.iter().map(|name| {
        let client = client.clone();
        let name = name.clone();
        let path = themes_dir.join(&name);
        async move {
            if path.exists() {
                return Ok(());
            }
            let url = format!(
                "https://raw.githubusercontent.com/JanDeDobbeleer/oh-my-posh/main/themes/{}",
                name
            );
            let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
            let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
            fs::write(path, bytes).map_err(|e| e.to_string())
        }
    });

    let _results = join_all(download_futures).await;
    let _ = tx.send(AppMessage::ThemesLoaded(theme_names)).await;
}
