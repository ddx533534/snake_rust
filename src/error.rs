use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    InitialError(String),
    InvalidInput(String),
    OutOfBounds(String),
    CircleError(String),
    CustomError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InitialError(msg) => write!(f, "Initial error: {}", msg),
            GameError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            GameError::OutOfBounds(msg) => write!(f, "Out of bounds:{}", msg),
            GameError::CircleError(msg) => write!(f, "Circle error:{}", msg),
            GameError::CustomError(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl Error for GameError {}