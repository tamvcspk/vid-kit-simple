use vid_kit_simple_lib::gpu_detector::{
    check_ffmpeg_codec, check_gpu_availability, GpuInfo, GpuList,
};

// Test case for checking if GPU detector returns valid output format
#[test]
fn test_gpu_list_structure() {
    let result = check_gpu_availability();

    // Đảm bảo kết quả là Ok
    assert!(result.is_ok(), "GPU check should return valid result");

    let gpu_list = result.unwrap();

    // Đảm bảo danh sách GPU không rỗng
    assert!(!gpu_list.gpus.is_empty(), "GPU list should not be empty");

    // Kiểm tra mỗi GPU có thông tin hợp lệ
    for gpu in gpu_list.gpus.iter() {
        // Tên không được rỗng
        assert!(!gpu.name.is_empty(), "GPU name should not be empty");

        // Vendor không được rỗng
        assert!(!gpu.vendor.is_empty(), "GPU vendor should not be empty");

        // is_available phải là boolean (luôn đúng vì kiểu dữ liệu)
        // supported_codecs phải là Vec (luôn đúng vì kiểu dữ liệu)

        // Nếu GPU available thì phải có ít nhất 1 codec
        if gpu.is_available {
            assert!(
                !gpu.supported_codecs.is_empty(),
                "Available GPU should have at least one supported codec"
            );
        }
    }
}

// Test case cho hàm check_ffmpeg_codec
#[test]
fn test_check_ffmpeg_codec() {
    // Test với codec phổ biến
    let h264_result = check_ffmpeg_codec("libx264");

    // Đây chỉ là kiểm tra cơ bản, kết quả phụ thuộc vào cài đặt FFmpeg trên máy
    // Trong môi trường test thật, chúng ta nên mock kết quả này
    println!("libx264 codec available: {}", h264_result);

    // Test với codec không tồn tại
    let fake_result = check_ffmpeg_codec("this_codec_does_not_exist");
    assert!(!fake_result, "Non-existent codec should return false");
}

// Test mock cho hàm check_gpu_availability
#[test]
fn test_mock_gpu_list() {
    // Mô phỏng kết quả từ GPU nếu chúng ta không có GPU thật để test
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

    // Kiểm tra cấu trúc của mock data
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
