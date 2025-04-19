//! # Utilities Module
//!
//! This module contains utility functions and helpers used throughout the application.
//! These utilities provide common functionality that is not specific to any particular
//! domain but is used by multiple components.
//!
//! ## Available Utilities
//!
//! - `gpu_detector`: Detects GPU capabilities and available hardware acceleration
//! - `error`: Defines error types and error handling utilities
//! - `error_handler`: Provides error handling functions and macros
//! - `event_emitter`: Utilities for emitting events to the frontend
//! - `logger`: Provides file-based logging functionality with daily rotation

/// GPU detection utility that identifies available GPUs and their capabilities
/// for hardware-accelerated video processing
pub mod gpu_detector;

/// Error types and utilities for standardized error handling across the application
pub mod error;

/// Error handling functions and macros to convert between different error types
/// and provide consistent error reporting
pub mod error_handler;

/// Event emitter utilities for sending notifications and errors to the frontend
pub mod event_emitter;

/// Logger utilities for file-based logging with daily rotation
pub mod logger;
