use std::error::Error;
use std::fmt;
use crossterm::event::{EnableMouseCapture, Event, KeyCode, KeyEvent, read};
use crate::canvas::Canvas;
use crate::food::Food;
use crate::snake::{Direction, Move, Snake};


#[derive(Debug)]
pub struct GameInfo {
    canvas: Canvas,
    food: Food,
    snake: Snake,
    scores: i32,
}

impl GameInfo {
    pub fn default() -> GameInfo {
        let canvas = Canvas::default();
        let food = Food::default(&canvas);
        let snake = Snake::default(&canvas, &food);
        let scores = 0;
        GameInfo { canvas, food, snake, scores }
    }

    pub fn start(&mut self) -> Result<bool, Box<dyn Error>> {
        // 启用鼠标捕获模式以便捕获键盘输入
        let _enable_mouse_capture = EnableMouseCapture;
        loop {
            let canvas = &mut self.canvas;
            let food = &mut self.food;
            let snake = &mut self.snake;
            let mut move_res = false;
            // 读取事件
            if let Ok(Event::Key(KeyEvent { code, modifiers: _modifiers })) = read() {
                // 检查是否按下了方向键
                match code {
                    KeyCode::Up => snake.turn(Direction::Up)?,
                    KeyCode::Down => snake.turn(Direction::Down)?,
                    KeyCode::Left => snake.turn(Direction::Left)?,
                    KeyCode::Right => snake.turn(Direction::Right)?,
                    KeyCode::Esc => {
                        println!("Esc pressed! exist snake game!");
                        break;
                    }
                    _ => {
                        println!("input illegal! input again!");
                        continue;
                    }
                };
            }
            move_res = snake.moving(canvas, food)?;
            if move_res {
                food.position = Food::generate_new_food(canvas, snake).position;
            }
            self.format_output();
        }
        Ok(true)
    }

    pub fn format_output(&mut self) {
        println!("蛇：{:?},食物：{:?}", self.snake, self.food);
        for i in 0..self.canvas.height {
            for j in 0..self.canvas.width {
                if self.food.position == (j, i) {
                    print!("*");
                } else if self.snake.body.contains(&(j, i)) {
                    print!("=");
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }
}


// impl fmt::Display for GameInfo {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         Ok()
//     }
// }