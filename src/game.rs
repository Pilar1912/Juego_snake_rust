use piston_window::types::Color;
use piston_window::*;

use drawing::{draw_block, draw_rectange, draw_apple, draw_apple_with_color};
use rand::{thread_rng, Rng};
use snake::{Direction, Snake};

//const FOOD_COLOR: Color = [0.90, 0.49, 0.13, 1.0];
//const FOOD_COLOR: Color = [0.18, 0.80, 0.44, 1.0];   // Verde menta (el antiguo color de la serpiente)
const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.5];
const ENEMY_COLOR: Color = [0.91, 0.30, 0.24, 1.0];   // Rojo oscuro

const MOVING_PERIOD: f64 = 0.5; // in second
const RESTART_TIME: f64 = 1.0; // in second
const FOOD_MOVE_PERIOD: f64 = 2.0; // en segundos (ajústalo a 0.8 o 1.0 si quieres más lento)
const ENEMY_MOVING_PERIOD: f64 = 0.5; // Enemigo se mueve más rápido

#[derive(PartialEq)]
enum FoodType {
    Normal,
    Longer,
    SpeedUp,
    SlowDown,
    Moving,   // la que se mueve cada 0.6s
}

struct Enemy {
    x: i32,
    y: i32,
    move_timer: f64,
}

pub struct Game {
    snake: Snake,

    // Food
    food_exist: bool,
    food_x: i32,
    food_y: i32,

    // Enemy
    enemy: Enemy,

    // Game Space
    width: i32,
    height: i32,

    // Game state
    is_game_over: bool,
    // When the game is running, it represents the waiting time from the previous moving
    // When the game is over, it represents the waiting time from the end of the game
    waiting_time: f64,

    food_eaten_count: i32,    // contador de comidas normales consecutivas
    food_moving_mode: bool,   // true si la comida actual debe moverse
    food_move_timer: f64,
    current_food_type: FoodType,

    // temporizadores y efectos
    speed_up_timer: f64,
    slow_down_timer: f64,
    original_moving_period: f64,
    current_moving_period: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exist: true,
            food_x: 5,
            food_y: 3,
            enemy: Enemy {
                x: width - 5,
                y: height - 5,
                move_timer: 0.0,
            },
            width: width,
            height: height,
            is_game_over: false,
            food_eaten_count: 0,
            food_moving_mode: false,
            food_move_timer: 0.0,
            current_food_type: FoodType::Normal,
            speed_up_timer: 0.0,
            slow_down_timer: 0.0,
            original_moving_period: MOVING_PERIOD,
            current_moving_period: MOVING_PERIOD,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            // Ignore other keys
            _ => return,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        // Check if the snake hits the border
        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exist {
            match self.current_food_type {
                FoodType::Normal => {
                    draw_apple(self.food_x, self.food_y, con, g);
                }
                FoodType::Longer => {
                    // Manzana dorada
                    draw_apple_with_color(self.food_x, self.food_y, [1.0, 0.84, 0.0, 1.0], [1.0, 0.95, 0.3, 0.8], con, g);
                }
                FoodType::SpeedUp => {
                    // Manzana naranja
                    draw_apple_with_color(self.food_x, self.food_y, [1.0, 0.5, 0.0, 1.0], [1.0, 0.7, 0.2, 0.8], con, g);
                }
                FoodType::SlowDown => {
                    // Manzana azul
                    draw_apple_with_color(self.food_x, self.food_y, [0.0, 0.5, 1.0, 1.0], [0.3, 0.7, 1.0, 0.8], con, g);
                }
                FoodType::Moving => {
                    // Manzana morada
                    draw_apple_with_color(self.food_x, self.food_y, [0.8, 0.2, 0.8, 1.0], [1.0, 0.5, 1.0, 0.8], con, g);
                }
            }
        }

        // Draw the enemy
        draw_block(ENEMY_COLOR, self.enemy.x, self.enemy.y, con, g);

        // Draw the border
        draw_rectange(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectange(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // Draw a game-over rectangle
        if self.is_game_over {
            draw_rectange(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    /*ub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // If the game is over
        if self.is_game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        // Check if the food still exists
        if !self.food_exist {
            self.add_food();
        }

        // Move the snake
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }

        // Si la comida existe y está en modo móvil, la movemos a otro lugar
        if self.food_moving_mode && self.food_exist {
            self.move_food();
        }
        self.waiting_time = 0.0;
    }*/

    pub fn update(&mut self, delta_time: f64) {
    self.waiting_time += delta_time;
    self.update_powerups(delta_time);  // nueva línea

    if self.is_game_over {
        if self.waiting_time > RESTART_TIME {
            self.restart();
        }
        return;
    }

    if !self.food_exist {
        self.add_food();
    }

    // Usar el período dinámico
    if self.waiting_time > self.current_moving_period {
        self.update_snake(None);
        self.waiting_time = 0.0;
    }

    if self.food_moving_mode && self.food_exist {
        self.food_move_timer += delta_time;
        if self.food_move_timer > FOOD_MOVE_PERIOD {
            self.move_food();
            self.food_move_timer = 0.0;
        }
    }

    // Update enemy movement
    self.enemy.move_timer += delta_time;
    if self.enemy.move_timer > ENEMY_MOVING_PERIOD {
        self.update_enemy();
        self.enemy.move_timer = 0.0;
    }
}

    /*fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_last_removed();
        }
    }*/

    fn check_eating(&mut self) {
    let (head_x, head_y) = self.snake.head_position();
    if self.food_exist && self.food_x == head_x && self.food_y == head_y {
        self.food_exist = false;

        // Aplicar efecto según el tipo de comida
        match self.current_food_type {
            FoodType::Normal => {
                self.snake.restore_last_removed();
                if !self.food_moving_mode {
                    self.food_eaten_count += 1;
                }
            }
            FoodType::Longer => {
                self.snake.grow_by(3);
                if !self.food_moving_mode {
                    self.food_eaten_count += 1;
                }
            }
            FoodType::SpeedUp => {
                self.snake.restore_last_removed();
                self.activate_speed_up();
                if !self.food_moving_mode {
                    self.food_eaten_count += 1;
                }
            }
            FoodType::SlowDown => {
                self.snake.restore_last_removed();
                self.activate_slow_down();
                if !self.food_moving_mode {
                    self.food_eaten_count += 1;
                }
            }
            FoodType::Moving => {
                self.snake.restore_last_removed();
                self.food_moving_mode = false;
                self.food_eaten_count = 0;
                // No incrementa contador, solo reinicia ciclo
            }
        }

        // Control del modo móvil (solo para comidas no móviles)
            if !self.food_moving_mode {
                if self.food_eaten_count >= 3 {
                    self.food_moving_mode = true;
                    self.food_eaten_count = 0;
                }
            } else {
                // Si estamos en modo móvil, reiniciamos después de comer una
                self.food_moving_mode = false;
                self.food_eaten_count = 0;
            }
        }
    }

    fn check_if_the_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head_position(dir);

        // Check if the snake hits itself
        if self.snake.is_overlap_except_tail(next_x, next_y) {
            return false;
        }

        // Check if the snake overlaps with the border
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_the_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
            self.check_enemy_collision();
        } else {
            self.is_game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn update_enemy(&mut self) {
        let (snake_head_x, snake_head_y) = self.snake.head_position();
        let (enemy_x, enemy_y) = (self.enemy.x, self.enemy.y);

        // Calculate direction towards snake's head
        let dx = snake_head_x - enemy_x;
        let dy = snake_head_y - enemy_y;

        // Move towards the snake (Manhattan distance approach)
        if dx.abs() > dy.abs() {
            // Move horizontally
            if dx > 0 {
                self.enemy.x += 1;
            } else {
                self.enemy.x -= 1;
            }
        } else {
            // Move vertically
            if dy > 0 {
                self.enemy.y += 1;
            } else {
                self.enemy.y -= 1;
            }
        }

        // Keep enemy within borders
        if self.enemy.x <= 1 {
            self.enemy.x = 1;
        } else if self.enemy.x >= self.width - 2 {
            self.enemy.x = self.width - 2;
        }

        if self.enemy.y <= 1 {
            self.enemy.y = 1;
        } else if self.enemy.y >= self.height - 2 {
            self.enemy.y = self.height - 2;
        }

        self.check_enemy_collision();
    }

    fn check_enemy_collision(&mut self) {
        // Check if the enemy touches any part of the snake body (head or tail)
        if self.snake.is_touching_point(self.enemy.x, self.enemy.y) {
            self.is_game_over = true;
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

    // Si debe ser comida móvil
        if self.food_moving_mode {
            // Cuando está en modo móvil, selecciona aleatoriamente entre diferentes tipos
            let r: f64 = rng.gen();
            self.current_food_type = if r < 0.25 {
                FoodType::Normal
            } else if r < 0.5 {
                FoodType::Longer
            } else if r < 0.75 {
                FoodType::SpeedUp
            } else {
                FoodType::SlowDown
            };
        } else {
        // Probabilidades: 60% normal, 15% longer, 15% speedup, 10% slowdown
            let r: f64 = rng.gen();
            self.current_food_type = if r < 0.6 {
                FoodType::Normal
            } else if r < 0.75 {
                FoodType::Longer
            } else if r < 0.9 {
                FoodType::SpeedUp
            } else {
                FoodType::SlowDown
            };
        }

    // Elegir posición libre (igual que antes)
        let mut new_x = rng.gen_range(1..(self.width - 1));
        let mut new_y = rng.gen_range(1..(self.height - 1));
        while self.snake.is_overlap_except_tail(new_x, new_y) {
            new_x = rng.gen_range(1..(self.width - 1));
            new_y = rng.gen_range(1..(self.height - 1));
        }


        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exist = true;
        self.food_move_timer = 0.0;
    }

    fn restart(&mut self) {
    self.snake = Snake::new(2, 2);
    self.waiting_time = 0.0;
    self.food_exist = true;
    self.food_x = 5;
    self.food_y = 3;
    self.is_game_over = false;
    self.food_move_timer = 0.0;
    self.food_eaten_count = 0;
    self.food_moving_mode = false;
    self.speed_up_timer = 0.0;
    self.slow_down_timer = 0.0;
    self.current_moving_period = self.original_moving_period;
    self.current_food_type = FoodType::Normal;
    // Restart enemy
    self.enemy.x = self.width - 5;
    self.enemy.y = self.height - 5;
    self.enemy.move_timer = 0.0;
}

    fn move_food(&mut self) {
    let mut rng = thread_rng();
    let mut new_x = rng.gen_range(1..(self.width - 1));
    let mut new_y = rng.gen_range(1..(self.height - 1));
    while self.snake.is_overlap_except_tail(new_x, new_y) {
        new_x = rng.gen_range(1..(self.width - 1));
        new_y = rng.gen_range(1..(self.height - 1));
    }
    self.food_x = new_x;
    self.food_y = new_y;
    }

    fn activate_speed_up(&mut self) {
        // Cancela ralentización si existe
        self.slow_down_timer = 0.0;
        self.speed_up_timer = 5.0;
        self.current_moving_period = 0.1;
    }

    fn activate_slow_down(&mut self) {
        self.speed_up_timer = 0.0;
        self.slow_down_timer = 5.0;
        self.current_moving_period = 0.4;
    }

    fn update_powerups(&mut self, delta_time: f64) {
        if self.speed_up_timer > 0.0 {
            self.speed_up_timer -= delta_time;
            if self.speed_up_timer <= 0.0 {
                self.current_moving_period = self.original_moving_period;
            }
        }
        if self.slow_down_timer > 0.0 {
            self.slow_down_timer -= delta_time;
            if self.slow_down_timer <= 0.0 {
                self.current_moving_period = self.original_moving_period;
            }
        }
    }
}
