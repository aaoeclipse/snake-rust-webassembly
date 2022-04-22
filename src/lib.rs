use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    apple: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        let mut rng = thread_rng();

        World {
            width: width,
            size: (width * width),
            snake: Snake::new(snake_idx),
            apple: rng.gen_range(0..=(width * width)),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_state(&mut self, new_state: usize) {
        self.snake.change_state(new_state);
    }

    pub fn get_apple(&self) -> usize {
        self.apple
    }

    pub fn check_collision(&mut self) {
        if self.collide_with_apple() {
            let mut rng = thread_rng();
            self.snake.add_size();
            self.apple = rng.gen_range(0..=(self.size));
        }
    }

    pub fn collide_with_apple(&self) -> bool {
        if self.snake_head_idx() == self.apple {
            return true;
        }

        false
    }

    pub fn update(&mut self) {
        self.check_collision();
        let snake_idx = self.snake_head_idx();
        let left = 0;
        let up = 1;
        let right = 2;
        let down = 3;

        if self.snake.state == right {
            self.snake.body[0].0 = (snake_idx + 1) % self.size;
            // if it goes to the right it has to come on the same row to the left
            if self.snake.body[0].0 % self.width == 0 && self.snake.body[0].0 != 0 {
                self.snake.body[0].0 = self.snake.body[0].0 - self.width();
            }
        } else if self.snake.state == up {
            let result = snake_idx - self.width();
            self.snake.body[0].0 = result % self.size;
        } else if self.snake.state == down {
            let result = snake_idx + self.width();
            self.snake.body[0].0 = result % self.size;
        } else if self.snake.state == left {
            let result = snake_idx - 1;
            self.snake.body[0].0 = result % self.size;

            // if it goes to the left it has to come on the same row to the right
            if self.snake.body[0].0 % self.width == (self.width - 1) {
                self.snake.body[0].0 = self.snake.body[0].0 + self.width();
            }
        } else {
            self.snake.body[0].0 = self.snake.body[0].0;
        }
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    state: usize,
    size: usize,
}

impl Snake {
    fn new(spwan_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spwan_index)],
            state: 5,
            size: 1,
        }
    }

    fn change_state(&mut self, new_state: usize) {
        self.state = new_state;
    }

    fn add_size(&mut self) {
        self.size += 1
    }
}

// wasm-pack build --target web
