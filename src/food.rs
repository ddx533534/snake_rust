use std::error::Error;
use rand::Rng;
use crate::canvas::Canvas;
use crate::error::GameError;
use crate::snake::Snake;
#[derive(Debug)]
pub struct Food {
    pub position: (i32, i32),
}

impl Food {
    pub fn default(canvas: &Canvas) -> Food {
        let mut random = rand::thread_rng();
        Food { position: (random.gen_range(0..=canvas.width - 1), random.gen_range(0..=canvas.height - 1)) }
    }
    pub fn new(canvas: &Canvas, x: i32, y: i32) -> Result<Food, Box<dyn Error>> {
        if x < 0 || x > canvas.width {
            return Err(Box::new(GameError::InitialError(String::from("food x is negative or more than canvas.width!"))));
        }
        if y < 0 || y > canvas.height {
            return Err(Box::new(GameError::InitialError(String::from("food y is negative or more than canvas.height"))));
        }
        Ok(Food { position: (x, y) })
    }

    pub fn generate_new_food(canvas: &Canvas, snake: &Snake) -> Food {
        let mut random = rand::thread_rng();
        let mut food = Food { position: (random.gen_range(0..=canvas.width), random.gen_range(0..=canvas.height)) };

        loop {
            if !snake.body.contains(&food.position) {
                return food;
            }
            food = Food { position: (random.gen_range(0..=canvas.width), random.gen_range(0..=canvas.height)) }
        }
    }
}