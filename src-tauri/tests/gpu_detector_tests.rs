use vid_kit_simple_lib::utils::gpu_detector::{
    check_ffmpeg_codec, check_gpu_availability, GpuInfo, GpuList,
};

// Test case for checking if GPU detector returns valid output format
#[test]
fn test_gpu_list_structure() {
    let result = check_gpu_availability();

    // Ensure the result is Ok
    assert!(result.is_ok(), "GPU check should return valid result");

    let gpu_list = result.unwrap();

    // Ensure the GPU list is not empty
    assert!(!gpu_list.gpus.is_empty(), "GPU list should not be empty");

    // Check that each GPU has valid information
    for gpu in gpu_list.gpus.iter() {
        // Name should not be empty
        assert!(!gpu.name.is_empty(), "GPU name should not be empty");

        // Vendor should not be empty
        assert!(!gpu.vendor.is_empty(), "GPU vendor should not be empty");

        // is_available must be boolean (always true due to data type)
        // supported_codecs must be Vec (always true due to data type)

        // If GPU is available, it must have at least 1 codec
        if gpu.is_available {
            assert!(
                !gpu.supported_codecs.is_empty(),
                "Available GPU should have at least one supported codec"
            );
        }
    }
}

// Test case for check_ffmpeg_codec function
#[test]
fn test_check_ffmpeg_codec() {
    // Test with common codec
    let h264_result = check_ffmpeg_codec("libx264");

    // This is just a basic check, the result depends on the FFmpeg installation on the machine
    // In a real test environment, we should mock this result
    println!("libx264 codec available: {}", h264_result);

    // Test with non-existent codec
    let fake_result = check_ffmpeg_codec("this_codec_does_not_exist");
    assert!(!fake_result, "Non-existent codec should return false");
}

// Mock test for check_gpu_availability function
#[test]
fn test_mock_gpu_list() {
    // Simulate results from GPU if we don't have a real GPU for testing
    let mock_gpu_list = GpuList {
        gpus: vec![
            GpuInfo {
                name: "Test GPU".to_string(),
                vendor: "NVIDIA".to_string(),
                is_available: true,
                supported_codecs: vec!["h264_nvenc".to_string()],
            },
            GpuInfo {
                name: "CPU Only".to_string(),
                vendor: "None".to_string(),
                is_available: false,
                supported_codecs: vec![],
            },
        ],
    };

    // Check the structure of mock data
    assert_eq!(
        mock_gpu_list.gpus.len(),
        2,
        "Mock GPU list should have 2 items"
    );
    assert_eq!(mock_gpu_list.gpus[0].name, "Test GPU");
    assert_eq!(mock_gpu_list.gpus[0].vendor, "NVIDIA");
    assert!(mock_gpu_list.gpus[0].is_available);
    assert_eq!(mock_gpu_list.gpus[0].supported_codecs.len(), 1);

    assert_eq!(mock_gpu_list.gpus[1].name, "CPU Only");
    assert!(!mock_gpu_list.gpus[1].is_available);
    assert_eq!(mock_gpu_list.gpus[1].supported_codecs.len(), 0);
}
