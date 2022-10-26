use piston_window::{types::{Color}, *};
use rand::{thread_rng, Rng};
use crate::{snake, canvas};
use snake::{Snake, Direction};
use canvas::{draw_block, draw_rectangle};


const APPLE_COLOR: Color = [1.00, 0.20,0.00,1.00];
const BORDER_COLOR: Color = [0.5,0.5,0.5,1.0];
const GAME_OVER_COLOR: Color = [0.90,0.00,0.00,0.50];

const FPS: f64 = 0.06;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(3,3),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.01,
        }
    }

    pub fn key_pressed(&mut self, key:Key) {
        if self.game_over {
            return;
        }   

        let dir: Option<Direction> = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None, 
        };

        if dir.unwrap() == Direction::opposite(self.snake.head_direction()){
            return;
        }
        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(APPLE_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g)
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > FPS{
            self.update_snake(None); 
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) : (i32,i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    
    }
    fn check_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) : (i32,i32) = self.snake.next_head(dir);
        if self.snake.overlapping_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x: i32 = rng.gen_range(1..self.width - 1); 
        let mut new_y: i32 = rng.gen_range(1..self.height - 1);
        while self.snake.overlapping_tail(new_x,new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y= rng.gen_range(1..self.height - 1);
        } 

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self,dir: Option<Direction>) {
        if self.check_alive(dir) {
            self.snake.forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2,2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}