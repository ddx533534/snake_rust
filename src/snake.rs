// 贪吃蛇移动的方向

use std::error::Error;
use rand::Rng;
use crate::canvas::Canvas;
use crate::error::GameError;
use crate::food::Food;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn is_direction_opposite(d1: &Direction, d2: &Direction) -> bool {
        match (d1, d2) {
            (Direction::Right, Direction::Left)
            | (Direction::Left, Direction::Right)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up) => true,
            _ => false,
        }
    }
    pub fn is_direction_same(d1: &Direction, d2: &Direction) -> bool { d1 == d2 }
}

// 最大移动速率
const MAX_RATE: i32 = 5;
// 最小移动速率
const MIN_RATE: i32 = 1;
const BASE_RATE: i32 = 1;

#[derive(Debug)]
pub struct Snake {
    // 蛇身，由许多个点坐标构成
    pub body: Vec<(i32, i32)>,
    // 速率
    pub rate: i32,
    // 方向
    pub direction: Direction,
}

pub trait Move {
    fn moving(&mut self, canvas: &Canvas, food: &Food) -> Result<bool, Box<dyn Error>>;
    fn turn(&mut self, direction: Direction) -> Result<bool, Box<dyn Error>>;
    fn change_rate(&mut self, rate: i32) -> Result<bool, Box<dyn Error>>;
}

impl Snake {
    pub fn default(canvas: &Canvas, food: &Food) -> Snake {
        let mut body = Vec::new();
        let mut random = rand::thread_rng();
        let mut res = (random.gen_range(0..canvas.width), random.gen_range(0..canvas.height));
        while res == food.position {
            res = (random.gen_range(0..canvas.width), random.gen_range(0..canvas.height));
        }
        body.push(res);
        Snake { body, rate: MIN_RATE, direction: Direction::Right }
    }
    pub fn new(body: Vec<(i32, i32)>, rate: i32, direction: Direction) -> Result<Snake, Box<dyn Error>> {
        if body.is_empty() {
            return Err(Box::new(GameError::InitialError(String::from("snake body is empty!"))));
        }
        if rate > MAX_RATE || rate < MIN_RATE {
            return Err(Box::new(GameError::InitialError(String::from("snake rate is illegal!"))));
        }
        Ok(Snake { body, rate, direction })
    }
}

impl Move for Snake {
    fn moving(&mut self, canvas: &Canvas, food: &Food) -> Result<bool, Box<dyn Error>> {
        let rate = self.rate;
        let times = rate / BASE_RATE;
        let mut body = &mut self.body;
        let mut head = body[0];
        let mut tail = body[body.len() - 1].clone();
        for e in 0..times {
            if body.len() > 1 {
                for i in (1..body.len()).rev() {
                    body[i] = body[i - 1];
                }
            }
            match self.direction {
                Direction::Right => {
                    head.0 += rate;
                }
                Direction::Left => {
                    head.0 -= rate;
                }
                Direction::Up => {
                    head.1 -= rate;
                }
                Direction::Down => {
                    head.1 += rate;
                }
            }
            if body.contains(&head) {
                println!("oh no eat your self!");
                return Err(Box::new(GameError::CircleError(String::from("oh no eat your self!"))));
            }
            body[0] = head;
            if head == food.position {
                println!("eat food!");
                body.push(tail);
                return Ok(true);
            }
            if head.0 >= canvas.width || head.0 < 0 || head.1 >= canvas.height || head.1 < 0 {
                println!("oh no hit the wall!");
                return Err(Box::new(GameError::OutOfBounds(String::from("oh no hit the wall!"))));
            }
        }
        Ok(false)
    }


    fn turn(&mut self, direction: Direction) -> Result<bool, Box<dyn Error>> {
        return if Direction::is_direction_opposite(&self.direction, &direction) {
            // invalid, can't step back!
            Ok(false)
        } else {
            self.direction = direction;
            Ok(true)
        };
    }

    fn change_rate(&mut self, rate: i32) -> Result<bool, Box<dyn Error>> {
        if rate > MAX_RATE || rate < MIN_RATE {
            return Err(Box::new(GameError::CustomError(String::from("speed illegal!!!"))));
        }
        self.rate = rate;
        Ok(true)
    }
}


