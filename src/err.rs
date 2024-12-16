use bevy::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;
use thiserror::Error;

/// Custom Result type for the application
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Clone)]
pub enum ComponentError {
    #[error("Component not found: {0}")]
    NotFound(String),

    #[error("Component initialization failed: {0}")]
    InitFailed(String),

    #[error("Component validation failed: {0}")]
    ValidationFailed(String),

    #[error("Component update failed: {0}")]
    UpdateFailed(String),

    #[error("Component state error: {0}")]
    StateError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

#[derive(Debug, Error, Clone)]
pub enum ResourceError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource initialization failed: {0}")]
    InitFailed(String),

    #[error("Resource validation failed: {0}")]
    ValidationFailed(String),

    #[error("Resource state error: {0}")]
    StateError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Resource load failed: {0}")]
    LoadFailed(String),
}

#[derive(Debug, Error, Clone)]
pub enum SystemError {
    #[error("System initialization failed: {0}")]
    InitFailed(String),

    #[error("System execution failed: {0}")]
    ExecutionFailed(String),

    #[error("System state error: {0}")]
    StateError(String),
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Component error: {0}")]
    Component(#[from] ComponentError),

    #[error("Resource error: {0}")]
    Resource(#[from] ResourceError),

    #[error("System error: {0}")]
    System(#[from] SystemError),

    #[error("IO error: {0}")]
    Io(String),

    #[error("{0}")]
    Custom(String),
}

impl Error {
    pub fn custom<S: Into<String>>(msg: S) -> Self {
        Error::Custom(msg.into())
    }

    pub fn from_io(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

/// Centralized error manager that integrates with Bevy's systems
#[derive(Resource)]
pub struct ErrorManager {
    errors: Arc<RwLock<Vec<Error>>>,
}

impl Default for ErrorManager {
    fn default() -> Self {
        Self {
            errors: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl ErrorManager {
    /// Report an error and return () for Bevy system compatibility
    pub fn report_error(&self, error: Error) {
        error!("Error occurred: {}", error);
        self.errors.write().push(error);
    }

    /// Handle an error with recovery message
    pub fn report_with_recovery(&self, error: Error, recovery_msg: &str) {
        self.report_error(error);
        info!("Recovery: {}", recovery_msg);
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.read().is_empty()
    }

    /// Clear all accumulated errors
    pub fn clear(&self) {
        self.errors.write().clear();
    }

    /// Get all accumulated errors
    pub fn get_errors(&self) -> Vec<Error> {
        self.errors.read().clone()
    }
}

/// Extension trait for Result to integrate with ErrorManager
pub trait ErrorExt<T> {
    fn log_to_manager(self, manager: &ErrorManager) -> Option<T>;
}

impl<T> ErrorExt<T> for Result<T, Error> {
    fn log_to_manager(self, manager: &ErrorManager) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                manager.report_error(error);
                None
            }
        }
    }
}

/// Bevy system that periodically checks and handles accumulated errors
pub fn error_check_system(error_manager: Res<ErrorManager>, time: Res<Time>) {
    const ERROR_CHECK_INTERVAL: f32 = 1.0; // Check every second

    let current_time = time.elapsed_secs_f64() as f32;
    if current_time % ERROR_CHECK_INTERVAL < time.delta_secs() {
        let errors = error_manager.get_errors();
        if !errors.is_empty() {
            for error in errors {
                error!("Unhandled error: {}", error);
            }
            error_manager.clear();
        }
    }
}

