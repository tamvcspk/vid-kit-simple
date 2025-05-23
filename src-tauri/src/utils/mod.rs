//! # Utilities Module
//!
//! This module contains utility functions and helpers used throughout the application.
//! These utilities provide common functionality that is not specific to any particular
//! domain but is used by multiple components.
//!
//! ## Available Utilities
//!
//! - `gpu_detector`: Detects GPU capabilities and available hardware acceleration
//! - `app_info`: Provides application information including GPU and FFmpeg version
//! - `error`: Defines error types and error handling utilities
//! - `error_handler`: Provides error handling functions and macros
//! - `event_emitter`: Utilities for emitting events to the frontend
//! - `logger`: Provides utilities for accessing log files created by the Tauri Logging plugin
//! - `store_helper`: Utilities for working with the Tauri Store plugin

/// GPU detection utility that identifies available GPUs and their capabilities
/// for hardware-accelerated video processing
pub mod gpu_detector;

/// Application information provider including GPU and FFmpeg version
pub mod app_info;

/// Error types and utilities for standardized error handling across the application
pub mod error;

/// Error handling functions and macros to convert between different error types
/// and provide consistent error reporting
pub mod error_handler;

/// Event emitter utilities for sending notifications and errors to the frontend
pub mod event_emitter;

/// Utilities for accessing log files created by the Tauri Logging plugin
pub mod logger;

/// Utilities for working with the Tauri Store plugin
pub mod store_helper;
