use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Runtime};

/// Defines a preset for video conversion with configurable parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionPreset {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub output_format: String,
    pub resolution: Resolution,
    pub bitrate: Option<u32>,
    pub fps: Option<u32>,
    pub codec: Option<String>,
    pub use_gpu: bool,
    pub audio_codec: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Defines resolution options for video conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Resolution {
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "preset")]
    Preset { width: u32, height: u32 },
    #[serde(rename = "custom")]
    Custom { width: u32, height: u32 },
}

/// Manages preset storage and retrieval operations
pub struct PresetManager {
    presets_dir: PathBuf,
}

impl PresetManager {
    /// Creates a new instance of PresetManager
    pub fn new<P: AsRef<Path>>(presets_dir: P) -> io::Result<Self> {
        let presets_dir = presets_dir.as_ref().to_path_buf();

        // Tạo thư mục nếu không tồn tại
        if !presets_dir.exists() {
            fs::create_dir_all(&presets_dir)?;
        }

        Ok(Self { presets_dir })
    }

    /// Creates a PresetManager using a fixed directory for tests
    pub fn from_app_handle<R: Runtime>(_app_handle: &AppHandle<R>) -> io::Result<Self> {
        // Trong Tauri 2, cấu trúc AppHandle đã thay đổi
        // Để đơn giản hóa, sử dụng đường dẫn cố định cho thử nghiệm
        let app_data_dir = std::env::temp_dir().join("vid-kit-simple");
        let presets_dir = app_data_dir.join("presets");
        Self::new(presets_dir)
    }

    /// Saves a preset to a JSON file
    pub fn save_preset(&self, preset: &ConversionPreset) -> io::Result<()> {
        let file_path = self.get_preset_path(&preset.id);
        let json = serde_json::to_string_pretty(preset)?;
        fs::write(file_path, json)
    }

    /// Gets a preset by ID
    pub fn get_preset(&self, id: &str) -> io::Result<ConversionPreset> {
        let file_path = self.get_preset_path(id);
        let content = fs::read_to_string(file_path)?;
        serde_json::from_str(&content).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
    }

    /// Lists all available presets
    pub fn list_presets(&self) -> io::Result<Vec<ConversionPreset>> {
        let mut presets = Vec::new();

        if !self.presets_dir.exists() {
            return Ok(presets);
        }

        for entry in fs::read_dir(&self.presets_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(preset) = serde_json::from_str::<ConversionPreset>(&content) {
                        presets.push(preset);
                    }
                }
            }
        }

        Ok(presets)
    }

    /// Deletes a preset by ID
    pub fn delete_preset(&self, id: &str) -> io::Result<()> {
        let file_path = self.get_preset_path(id);

        if file_path.exists() {
            fs::remove_file(file_path)?;
            Ok(())
        } else {
            Err(io::Error::new(ErrorKind::NotFound, "Preset not found"))
        }
    }

    /// Creates default presets if none exist
    pub fn create_default_presets(&self) -> io::Result<()> {
        let presets = self.list_presets()?;

        if !presets.is_empty() {
            return Ok(());
        }

        let default_presets = vec![
            ConversionPreset {
                id: "default_mp4".to_string(),
                name: "Default MP4".to_string(),
                description: Some("Standard MP4 conversion with H.264".to_string()),
                output_format: "mp4".to_string(),
                resolution: Resolution::Original,
                bitrate: Some(8000),
                fps: None,
                codec: Some("libx264".to_string()),
                use_gpu: false,
                audio_codec: Some("aac".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            ConversionPreset {
                id: "high_quality".to_string(),
                name: "High Quality".to_string(),
                description: Some("High quality conversion with H.264".to_string()),
                output_format: "mp4".to_string(),
                resolution: Resolution::Preset {
                    width: 1920,
                    height: 1080,
                },
                bitrate: Some(12000),
                fps: Some(60),
                codec: Some("libx264".to_string()),
                use_gpu: true,
                audio_codec: Some("aac".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            ConversionPreset {
                id: "web_optimized".to_string(),
                name: "Web Optimized".to_string(),
                description: Some("Optimized for web streaming".to_string()),
                output_format: "mp4".to_string(),
                resolution: Resolution::Preset {
                    width: 1280,
                    height: 720,
                },
                bitrate: Some(5000),
                fps: Some(30),
                codec: Some("libx264".to_string()),
                use_gpu: false,
                audio_codec: Some("aac".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
        ];

        for preset in default_presets {
            self.save_preset(&preset)?;
        }

        Ok(())
    }

    // Helper để lấy đường dẫn file của preset
    fn get_preset_path(&self, id: &str) -> PathBuf {
        self.presets_dir.join(format!("{}.json", id))
    }
}

// Tạo các hàm Tauri command

#[tauri::command]
pub async fn list_presets<R: Runtime>(
    app_handle: tauri::AppHandle<R>,
) -> Result<Vec<ConversionPreset>, String> {
    let preset_manager = PresetManager::from_app_handle(&app_handle)
        .map_err(|e| format!("Failed to initialize preset manager: {}", e))?;

    preset_manager
        .list_presets()
        .map_err(|e| format!("Failed to list presets: {}", e))
}

#[tauri::command]
pub async fn get_preset<R: Runtime>(
    id: String,
    app_handle: tauri::AppHandle<R>,
) -> Result<ConversionPreset, String> {
    let preset_manager = PresetManager::from_app_handle(&app_handle)
        .map_err(|e| format!("Failed to initialize preset manager: {}", e))?;

    preset_manager
        .get_preset(&id)
        .map_err(|e| format!("Failed to get preset: {}", e))
}

#[tauri::command]
pub async fn save_preset<R: Runtime>(
    preset: ConversionPreset,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    let preset_manager = PresetManager::from_app_handle(&app_handle)
        .map_err(|e| format!("Failed to initialize preset manager: {}", e))?;

    preset_manager
        .save_preset(&preset)
        .map_err(|e| format!("Failed to save preset: {}", e))
}

#[tauri::command]
pub async fn delete_preset<R: Runtime>(
    id: String,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    let preset_manager = PresetManager::from_app_handle(&app_handle)
        .map_err(|e| format!("Failed to initialize preset manager: {}", e))?;

    preset_manager
        .delete_preset(&id)
        .map_err(|e| format!("Failed to delete preset: {}", e))
}

#[tauri::command]
pub async fn create_default_presets<R: Runtime>(
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    let preset_manager = PresetManager::from_app_handle(&app_handle)
        .map_err(|e| format!("Failed to initialize preset manager: {}", e))?;

    preset_manager
        .create_default_presets()
        .map_err(|e| format!("Failed to create default presets: {}", e))
}
