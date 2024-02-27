use std::error::Error;
use crate::error::GameError;

const DEFAULT_WIDTH: i32 = 32;
const DEFAULT_HEIGHT: i32 = 32;

const MAX_SIZE: i32 = 64;
const MIN_SIZE: i32 = 0;

#[derive(Debug)]
pub struct Canvas {
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Canvas {
    pub fn default() -> Canvas {
        Canvas { width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT / 4 }
    }
    pub fn new(width: i32, height: i32) -> Result<Canvas, Box<dyn Error>> {
        if width > MAX_SIZE || width < MIN_SIZE {
            return Err(Box::new(GameError::InitialError(String::from("width is illegal"))));
        }

        if height > MAX_SIZE || height < MIN_SIZE {
            return Err(Box::new(GameError::InitialError(String::from("height is illegal!"))));
        }
        Ok(Canvas { width, height })
    }
}