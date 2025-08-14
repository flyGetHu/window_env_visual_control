use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("Windows API error: {0}")]
    WindowsApiError(#[from] std::io::Error),
    
    #[error("Registry error: {0}")]
    RegistryError(String),
    
    #[error("Environment variable not found: {0}")]
    VariableNotFound(String),
    
    #[error("Invalid environment variable name: {0}")]
    InvalidVariableName(String),
    
    #[error("Invalid environment variable value: {0}")]
    InvalidVariableValue(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("YAML serialization error: {0}")]
    YamlSerializationError(#[from] serde_yaml::Error),
    
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    
    #[error("UTF-16 conversion error: {0}")]
    Utf16Error(#[from] std::string::FromUtf16Error),
    
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Refresh failed: {0}")]
    RefreshFailed(String),
    
    #[error("Generic error: {0}")]
    GenericError(#[from] Box<dyn std::error::Error>),
}

pub type EnvResult<T> = Result<T, EnvError>;