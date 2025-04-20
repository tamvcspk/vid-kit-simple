use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Cấu trúc để parse config.toml
#[derive(Debug, Default)]
struct Config {
    ffmpeg: Option<FFmpegConfig>,
}

#[derive(Debug, Default)]
struct FFmpegConfig {
    // dll_path được lấy từ biến môi trường thay vì từ config
    dlls: Option<Vec<String>>,
}

// Hàm đọc và parse file config.toml
fn read_config() -> Config {
    let config_path = Path::new("config.toml");
    let mut config = Config::default();

    if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(content) => {
                match toml::from_str::<HashMap<String, toml::Value>>(&content) {
                    Ok(parsed) => {
                        if let Some(ffmpeg) = parsed.get("ffmpeg") {
                            if let Some(ffmpeg_table) = ffmpeg.as_table() {
                                let mut ffmpeg_config = FFmpegConfig::default();

                                // Đường dẫn DLL được lấy từ biến môi trường thay vì từ config.toml

                                if let Some(dlls) = ffmpeg_table.get("dlls") {
                                    if let Some(dlls_array) = dlls.as_array() {
                                        let dlls_vec: Vec<String> = dlls_array
                                            .iter()
                                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                            .collect();

                                        if !dlls_vec.is_empty() {
                                            ffmpeg_config.dlls = Some(dlls_vec);
                                        }
                                    }
                                }

                                config.ffmpeg = Some(ffmpeg_config);
                            }
                        }
                    }
                    Err(e) => println!("cargo:warning=Failed to parse config.toml: {}", e),
                }
            }
            Err(e) => println!("cargo:warning=Failed to read config.toml: {}", e),
        }
    } else {
        println!("cargo:warning=config.toml not found, using default values");
    }

    config
}

// Hàm sao chép DLL
fn copy_dlls(config: &Config) {
    // Chỉ thực hiện khi build cho Windows target
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs"); // Chạy lại build script nếu nó thay đổi
    println!("cargo:rerun-if-changed=config.toml"); // Chạy lại nếu config thay đổi

    // --- Xác định thư mục đích (nơi .exe sẽ được tạo) ---
    // Lấy profile (debug/release)
    let profile = env::var("PROFILE").unwrap();
    // Lấy thư mục gốc của target
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = manifest_dir.join("target").join(profile);

    // Lấy cấu hình FFmpeg từ config
    if let Some(ffmpeg_config) = &config.ffmpeg {
        // Lấy đường dẫn đến thư mục chứa DLL từ biến môi trường
        let dll_path = match env::var("FFMPEG_DLL_PATH") {
            Ok(path) => {
                // Chuẩn hóa đường dẫn (thay thế dấu \ bằng /)
                let normalized_path = path.replace("\\", "/");
                println!(
                    "cargo:warning=Using FFMPEG_DLL_PATH from environment: {}",
                    normalized_path
                );
                PathBuf::from(normalized_path)
            }
            Err(_) => {
                println!("cargo:warning=FFMPEG_DLL_PATH not set, using default path");
                PathBuf::from("C:/vcpkg/installed/x64-windows/bin")
            }
        };

        // Kiểm tra xem thư mục có tồn tại không
        if !dll_path.exists() {
            println!(
                "cargo:warning=FFmpeg DLL directory does not exist: {}",
                dll_path.display()
            );
            println!("cargo:warning=Please install FFmpeg or update FFMPEG_DLL_PATH in .cargo/config.toml");
        } else {
            println!(
                "cargo:warning=FFmpeg DLL directory found: {}",
                dll_path.display()
            );
            // Liệt kê các file trong thư mục để debug
            if let Ok(entries) = fs::read_dir(&dll_path) {
                println!("cargo:warning=Files in directory:");
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("cargo:warning=  {}", entry.path().display());
                    }
                }
            }
        }

        // Lấy danh sách DLL cần sao chép
        let dlls = if let Some(dlls) = &ffmpeg_config.dlls {
            dlls.clone()
        } else {
            // Danh sách mặc định nếu không có trong config
            vec![
                "avcodec-60.dll".to_string(),
                "avformat-60.dll".to_string(),
                "avutil-58.dll".to_string(),
                "swresample-4.dll".to_string(),
                "swscale-7.dll".to_string(),
            ]
        };

        if dll_path.exists() {
            for dll_name in &dlls {
                let src_path = dll_path.join(dll_name);
                let dest_path = target_dir.join(dll_name);

                if src_path.exists() {
                    // Tạo thư mục đích nếu chưa tồn tại
                    if let Some(parent) = dest_path.parent() {
                        if !parent.exists() {
                            if let Err(e) = fs::create_dir_all(parent) {
                                println!(
                                    "cargo:warning=Failed to create directory {}: {}",
                                    parent.display(),
                                    e
                                );
                            }
                        }
                    }

                    match fs::copy(&src_path, &dest_path) {
                        Ok(_) => println!(
                            "cargo:warning=Copied {} to {}",
                            src_path.display(),
                            dest_path.display()
                        ),
                        Err(e) => println!("cargo:warning=Failed to copy {}: {}", dll_name, e),
                    }
                } else {
                    println!("cargo:warning=DLL not found: {}", src_path.display());
                    // Kiểm tra xem file có tồn tại với tên khác không (ví dụ: avcodec-59.dll thay vì avcodec-60.dll)
                    let parent = src_path.parent().unwrap_or(Path::new(""));
                    let prefix = dll_name.split('-').next().unwrap_or("");
                    if !prefix.is_empty() && parent.exists() {
                        if let Ok(entries) = fs::read_dir(parent) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    let file_name = entry.file_name().to_string_lossy().to_string();
                                    if file_name.starts_with(prefix) && file_name.ends_with(".dll")
                                    {
                                        println!("cargo:warning=Found similar DLL: {}", file_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!(
                "cargo:warning=DLL directory not found: {}",
                dll_path.display()
            );
        }
    } else {
        println!("cargo:warning=No FFmpeg configuration found in config.toml");
    }
}

fn main() {
    // Đọc cấu hình
    let config = read_config();

    // Sao chép DLL
    copy_dlls(&config);

    // Tiếp tục với build Tauri
    tauri_build::build()
}
