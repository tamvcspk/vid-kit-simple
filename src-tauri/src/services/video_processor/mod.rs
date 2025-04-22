mod error;
mod processor;

use serde::{Deserialize, Serialize};

pub use error::{VideoError, VideoResult};
pub use processor::VideoProcessor;

/// Video information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub path: String,
    pub format: String,
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub bitrate: u64,
    pub codec: String,
    pub framerate: f32,
}

/// Video processing options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingOptions {
    pub output_format: String,
    pub output_path: String,
    pub resolution: Option<(u32, u32)>,
    pub bitrate: Option<u64>,
    pub framerate: Option<f32>,
    pub use_gpu: bool,
    pub gpu_codec: Option<String>,
    pub cpu_codec: Option<String>,

    // Time options for splitting
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,

    // Edit options
    pub crop: Option<(u32, u32, u32, u32)>, // (x, y, width, height)
    pub rotate: Option<i32>,                // 90, 180, 270 degrees
    pub flip: Option<bool>,                 // horizontal flip
    pub flop: Option<bool>,                 // vertical flip

    // Sanitize options
    pub remove_metadata: Option<bool>,      // remove all metadata
    pub blur_regions: Option<Vec<(u32, u32, u32, u32)>>, // regions to blur (x, y, width, height)
    pub audio_volume: Option<f32>,          // adjust audio volume (1.0 = normal)
    pub denoise: Option<bool>,              // apply denoising filter
}
