use serde::{Serialize, Deserialize};
use gfx_hal::Instance;
use gfx_backend_vulkan as back;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub is_available: bool,
    pub supported_codecs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuList {
    pub gpus: Vec<GpuInfo>,
}

#[tauri::command]
pub fn check_gpu_availability() -> Result<GpuList, String> {
    // Tạo instance Vulkan
    let instance = back::Instance::create("VidKitSimple", 1)
        .map_err(|e| format!("Failed to create Vulkan instance: {}", e))?;

    // Lấy danh sách adapter
    let adapters = instance.enumerate_adapters();
    
    if adapters.is_empty() {
        return Ok(GpuList {
            gpus: vec![GpuInfo {
                name: "CPU Only".to_string(),
                vendor: "None".to_string(),
                is_available: false,
                supported_codecs: vec![],
            }],
        });
    }

    let mut gpu_list = Vec::new();

    // Lấy thông tin từ tất cả các adapter
    for adapter in adapters.iter() {
        let name = adapter.info.name.to_string();
        let vendor = match adapter.info.vendor {
            0x10DE => "NVIDIA",
            0x8086 => "Intel",
            0x1002 => "AMD",
            _ => "Unknown",
        };

        // Kiểm tra các codec được hỗ trợ
        let mut supported_codecs = Vec::new();

        match vendor {
            "NVIDIA" => {
                if check_ffmpeg_codec("h264_nvenc") {
                    supported_codecs.push("h264_nvenc".to_string());
                }
                if check_ffmpeg_codec("hevc_nvenc") {
                    supported_codecs.push("hevc_nvenc".to_string());
                }
                if check_ffmpeg_codec("scale_cuda") {
                    supported_codecs.push("scale_cuda".to_string());
                }
            },
            "Intel" => {
                if check_ffmpeg_codec("h264_qsv") {
                    supported_codecs.push("h264_qsv".to_string());
                }
            },
            "AMD" => {
                if check_ffmpeg_codec("h264_amf") {
                    supported_codecs.push("h264_amf".to_string());
                }
            },
            _ => {}
        }

        gpu_list.push(GpuInfo {
            name,
            vendor: vendor.to_string(),
            is_available: !supported_codecs.is_empty(),
            supported_codecs,
        });
    }

    Ok(GpuList { gpus: gpu_list })
}

pub fn check_ffmpeg_codec(codec: &str) -> bool {
    let output = Command::new("ffmpeg")
        .args(["-hide_banner", "-encoders"])
        .output();

    match output {
        Ok(output) => {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                output_str.contains(codec)
            } else {
                false
            }
        }
        Err(_) => false,
    }
} 