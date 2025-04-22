//! # Services Module
//!
//! This module contains the core business logic services of the application.
//! Each service is responsible for a specific domain of functionality and
//! provides a clean API for the rest of the application to use.
//!
//! ## Available Services
//!
//! - `video_processor`: Handles video processing operations using FFmpeg
//! - `preset_manager`: Manages conversion presets for video processing

/// Video processing service that handles video conversion, information extraction,
/// and task management using FFmpeg
pub mod video_processor;

// preset_manager has been moved to frontend
