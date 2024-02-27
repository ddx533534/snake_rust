use std::error::Error;
use std::{fmt, io};
use std::io::Write;
use crossterm::event::{EnableMouseCapture, Event, KeyCode, KeyEvent, read};
use crossterm::{execute, terminal};
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
        let mut direction = Direction::Right;
        loop {
            // 读取事件
            if let Ok(Event::Key(KeyEvent { code, modifiers: _modifiers })) = read() {
                // 检查是否按下了方向键
                match code {
                    KeyCode::Up => {
                        println!("Up arrow pressed");
                        direction = Direction::Up
                    }
                    KeyCode::Down => {
                        println!("Down arrow pressed");
                        direction = Direction::Down
                    }
                    KeyCode::Left => {
                        println!("Left arrow pressed");
                        direction = Direction::Left
                    }
                    KeyCode::Right => {
                        println!("Right arrow pressed");
                        direction = Direction::Right
                    }
                    _ => {
                        continue;
                    }
                }
                self.process_move_result(direction)?;
            }
        }
    }

    // 处理移动结果
    pub fn process_move_result(&mut self, direction: Direction) -> Result<bool, Box<dyn Error>> {
        // 调整方向
        let turn_res = self.snake.turn(direction)?;
        if turn_res {
            // 按照指定方向前进
            let got_food = self.snake.moving(&self.canvas, &self.food)?;
            if got_food {
                self.scores += 1;
                self.food.position = Food::generate_new_food(&self.canvas, &self.snake).position;
            }
            self.format_output();
            return Ok(true);
        } else {
            println!("opposite direction,no use!");
        }
        Ok(false)
    }

    pub fn format_output(&mut self) {
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).expect("clear error!");
        println!("游戏情况:蛇：{:?},食物：{:?},得分：{:?}", self.snake, self.food, self.scores);
        for i in 0..self.canvas.height {
            for j in 0..self.canvas.width {
                if self.food.position == (j, i) {
                    print!("*");
                } else if self.snake.body.contains(&(j, i)) {
                    if self.snake.body[0] == (j, i) {
                        match self.snake.direction {
                            Direction::Right => print!(">"),
                            Direction::Left => print!("<"),
                            Direction::Up => print!("^"),
                            Direction::Down => print!("v"),
                        }
                    } else {
                        print!("=");
                    }
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }
}
