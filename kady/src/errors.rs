use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
     #[error("Error from the file system: {0}")]
     Io(#[from] std::io::Error),
     #[error("The config file is invalid")]
     ConfigParsing(#[from] toml::de::Error)
}