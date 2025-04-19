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

/// GPU detection utility that identifies available GPUs and their capabilities
/// for hardware-accelerated video processing
pub mod gpu_detector;

/// Error types and utilities for standardized error handling across the application
pub mod error;

/// Error handling functions and macros to convert between different error types
/// and provide consistent error reporting
pub mod error_handler;
