use vid_kit_simple_lib::preset_manager::{ConversionPreset, PresetManager, Resolution};

fn setup_test_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("Failed to create temporary directory")
}

#[test]
fn test_preset_manager_init() {
    let temp_dir = setup_test_dir();
    let preset_dir = temp_dir.path().join("presets");
    
    PresetManager::new(&preset_dir).expect("Failed to create PresetManager");
    
    // Kiểm tra xem thư mục presets có được tạo không
    assert!(preset_dir.exists(), "Presets directory should be created");
}

#[test]
fn test_save_and_get_preset() {
    let temp_dir = setup_test_dir();
    let preset_dir = temp_dir.path().join("presets");
    
    let preset_manager = PresetManager::new(&preset_dir).expect("Failed to create PresetManager");
    
    // Tạo một preset mẫu
    let test_preset = ConversionPreset {
        id: "test_preset".to_string(),
        name: "Test Preset".to_string(),
        description: Some("Test description".to_string()),
        output_format: "mp4".to_string(),
        resolution: Resolution::Original,
        bitrate: Some(5000),
        fps: Some(30),
        codec: Some("libx264".to_string()),
        use_gpu: false,
        audio_codec: Some("aac".to_string()),
        created_at: "2023-05-15T10:00:00Z".to_string(),
        updated_at: "2023-05-15T10:00:00Z".to_string(),
    };
    
    // Lưu preset
    preset_manager.save_preset(&test_preset).expect("Failed to save preset");
    
    // Kiểm tra file có tồn tại không
    let preset_path = preset_dir.join("test_preset.json");
    assert!(preset_path.exists(), "Preset file should exist");
    
    // Lấy preset và kiểm tra
    let loaded_preset = preset_manager.get_preset("test_preset").expect("Failed to get preset");
    assert_eq!(loaded_preset.id, test_preset.id);
    assert_eq!(loaded_preset.name, test_preset.name);
    assert_eq!(loaded_preset.output_format, test_preset.output_format);
}

#[test]
fn test_list_presets() {
    let temp_dir = setup_test_dir();
    let preset_dir = temp_dir.path().join("presets");
    
    let preset_manager = PresetManager::new(&preset_dir).expect("Failed to create PresetManager");
    
    // Tạo một số preset mẫu
    let presets = vec![
        ConversionPreset {
            id: "preset1".to_string(),
            name: "Preset 1".to_string(),
            description: None,
            output_format: "mp4".to_string(),
            resolution: Resolution::Original,
            bitrate: None,
            fps: None,
            codec: None,
            use_gpu: false,
            audio_codec: None,
            created_at: "2023-05-15T10:00:00Z".to_string(),
            updated_at: "2023-05-15T10:00:00Z".to_string(),
        },
        ConversionPreset {
            id: "preset2".to_string(),
            name: "Preset 2".to_string(),
            description: None,
            output_format: "mkv".to_string(),
            resolution: Resolution::Original,
            bitrate: None,
            fps: None,
            codec: None,
            use_gpu: false,
            audio_codec: None,
            created_at: "2023-05-15T10:00:00Z".to_string(),
            updated_at: "2023-05-15T10:00:00Z".to_string(),
        },
    ];
    
    // Lưu các preset
    for preset in &presets {
        preset_manager.save_preset(preset).expect("Failed to save preset");
    }
    
    // Lấy danh sách và kiểm tra
    let loaded_presets = preset_manager.list_presets().expect("Failed to list presets");
    assert_eq!(loaded_presets.len(), 2, "Should have 2 presets");
    
    // Kiểm tra xem các id có tồn tại trong danh sách không
    let preset_ids: Vec<String> = loaded_presets.iter().map(|p| p.id.clone()).collect();
    assert!(preset_ids.contains(&"preset1".to_string()));
    assert!(preset_ids.contains(&"preset2".to_string()));
}

#[test]
fn test_delete_preset() {
    let temp_dir = setup_test_dir();
    let preset_dir = temp_dir.path().join("presets");
    
    let preset_manager = PresetManager::new(&preset_dir).expect("Failed to create PresetManager");
    
    // Tạo một preset mẫu
    let test_preset = ConversionPreset {
        id: "to_delete".to_string(),
        name: "Preset to Delete".to_string(),
        description: None,
        output_format: "mp4".to_string(),
        resolution: Resolution::Original,
        bitrate: None,
        fps: None,
        codec: None,
        use_gpu: false,
        audio_codec: None,
        created_at: "2023-05-15T10:00:00Z".to_string(),
        updated_at: "2023-05-15T10:00:00Z".to_string(),
    };
    
    // Lưu preset
    preset_manager.save_preset(&test_preset).expect("Failed to save preset");
    
    // Kiểm tra file có tồn tại không
    let preset_path = preset_dir.join("to_delete.json");
    assert!(preset_path.exists(), "Preset file should exist before deletion");
    
    // Xóa preset
    preset_manager.delete_preset("to_delete").expect("Failed to delete preset");
    
    // Kiểm tra file đã bị xóa chưa
    assert!(!preset_path.exists(), "Preset file should not exist after deletion");
}

#[test]
fn test_create_default_presets() {
    let temp_dir = setup_test_dir();
    let preset_dir = temp_dir.path().join("presets");
    
    let preset_manager = PresetManager::new(&preset_dir).expect("Failed to create PresetManager");
    
    // Tạo các preset mặc định
    preset_manager.create_default_presets().expect("Failed to create default presets");
    
    // Lấy danh sách và kiểm tra
    let loaded_presets = preset_manager.list_presets().expect("Failed to list presets");
    assert!(!loaded_presets.is_empty(), "Should have default presets");
    
    // Kiểm tra xem có các preset mặc định không
    let preset_ids: Vec<String> = loaded_presets.iter().map(|p| p.id.clone()).collect();
    assert!(preset_ids.contains(&"default_mp4".to_string()));
    assert!(preset_ids.contains(&"high_quality".to_string()));
    assert!(preset_ids.contains(&"web_optimized".to_string()));
    
    // Gọi lại create_default_presets không nên tạo thêm presets
    preset_manager.create_default_presets().expect("Failed to call create_default_presets again");
    
    let loaded_presets_after = preset_manager.list_presets().expect("Failed to list presets after second call");
    assert_eq!(loaded_presets.len(), loaded_presets_after.len(), 
               "Should not create more presets if they already exist");
} 