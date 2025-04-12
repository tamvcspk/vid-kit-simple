use std::env;
use std::path::PathBuf;

fn main() {
    // Đường dẫn đến thư mục chứa các tệp .lib của FFmpeg
    let ffmpeg_lib_path = PathBuf::from("../third_party/ffmpeg-built/lib");

    // Thêm đường dẫn thư viện FFmpeg vào linker search path
    println!(
        "cargo:rustc-link-search=native={}",
        ffmpeg_lib_path.display()
    );

    // Liệt kê các thư viện FFmpeg cần liên kết tĩnh
    let ffmpeg_libs = ["avcodec", "avformat", "avutil", "swresample", "swscale"];

    for lib in &ffmpeg_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    // Liên kết các thư viện hệ thống cần thiết
    let system_libs = [
        "bcrypt", "ws2_32", "user32", "gdi32", "advapi32", "ole32", "shell32",
    ];

    for lib in &system_libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    // Kiểm tra xem có cần liên kết với CUDA không
    if env::var("CARGO_FEATURE_CUDA").is_ok() {
        let cuda_libs = [
            "cudart", "cuda", "nppig", "nppicc", "nppidei", "nppitc", "npps", "nppc", "nppial",
            "nppim", "nppist", "nppisu", "nppit", "nppif", "nppicom",
        ];
        for lib in &cuda_libs {
            println!("cargo:rustc-link-lib=static={}", lib);
        }
    }
    // Đường dẫn đến thư mục chứa các tệp header của FFmpeg
    let ffmpeg_include_path = PathBuf::from("../third_party/ffmpeg-built/include");

    // Sử dụng bindgen để tạo binding từ các tệp header của FFmpeg
    let bindings = bindgen::Builder::default()
        .header(
            ffmpeg_include_path
                .join("libavcodec/avcodec.h")
                .to_string_lossy(),
        )
        // Thêm các tệp header khác nếu cần
        .clang_arg(format!("-I{}", ffmpeg_include_path.display()))
        .generate()
        .expect("Unable to generate bindings");

    // Lưu binding vào tệp src/ffmpeg_bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("ffmpeg_bindings.rs"))
        .expect("Couldn't write bindings!");
    tauri_build::build()
}
