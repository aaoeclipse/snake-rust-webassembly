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
        World {
            width: width,
            size: (width * width),
            snake: Snake::new(snake_idx),
            apple: 10,
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

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let left = 0;
        let up = 1;
        let right = 2;
        let down = 3;

        if self.snake.state == right {
            self.snake.body[0].0 = (snake_idx + 1) % self.size;
        } else if self.snake.state == up {
            let result = snake_idx - self.width();
            self.snake.body[0].0 = result % self.size;
        } else if self.snake.state == down {
            let result = snake_idx + self.width();
            self.snake.body[0].0 = result % self.size;
        } else if self.snake.state == left {
            let result = snake_idx - 1;
            self.snake.body[0].0 = result % self.size;
        }
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    state: usize,
}

impl Snake {
    fn new(spwan_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spwan_index)],
            state: 2,
        }
    }

    fn change_state(&mut self, new_state: usize) {
        self.state = new_state;
    }
}

// wasm-pack build --target web
