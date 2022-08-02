use rand::Rng;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::lib::types::{Block, Cell, Snake};

pub enum GameState {
    Over(u32),
    Going,
}

pub struct Game {
    snake: Snake,
    snake_direction: (i32, i32),
    snack: Block,
    nx_cells: u32,
    ny_cells: u32,
    cell_width: u32,
}

impl Game {
    pub fn new(nx_cells: u32, ny_cells: u32, cell_width: u32, direction: (i32, i32)) -> Self {
        Game {
            snake: Snake::init(2, 3),
            snack: Block {
                row: 9,
                col: 9,
                color: Cell::snack(),
            },
            nx_cells,
            ny_cells,
            cell_width,
            snake_direction: direction
        }
    }

    pub fn move_snake(&mut self, direction: (i32, i32)) -> GameState {

        

        let direction = if self.snake_direction.0 + direction.0 == 0 && self.snake_direction.1 + direction.1 == 0 {
            self.snake_direction
        } else {
            direction
        };

        self.snake_direction = direction;

        let new_row = (self.snake.body[0].row + direction.0).rem_euclid(self.ny_cells as i32); // % operator to wrap around screen
        let new_col = (self.snake.body[0].col + direction.1).rem_euclid(self.nx_cells as i32);



        for block in self.snake.body.iter().skip(1) {
            if block.row == self.snake.body[0].row && block.col == self.snake.body[0].col {
                return GameState::Over(self.snake.body.len() as u32);
            }
        }

        if new_col == self.snack.col && new_row == self.snack.row {
            let tail = self.snake.body.last().unwrap();
            let tail_pos = (tail.row, tail.col);

            self.snake.body.push(Block {
                row: tail_pos.0,
                col: tail_pos.1,
                color: Cell::snake(),
            });

            self.new_snack();
        }


        for i in (1..self.snake.body.len()).rev() {
            self.snake.body[i].row = self.snake.body[i - 1].row;
            self.snake.body[i].col = self.snake.body[i - 1].col;
        }

        // head movement
        self.snake.body[0].row = new_row;
        self.snake.body[0].col = new_col;

        GameState::Going
    }

    pub fn draw_snake(&self, renderer: &mut Canvas<Window>) {
        for block in self.snake.body.iter() {
            self.draw_block(block, renderer);
        }
    }

    pub fn draw_block(&self, block: &Block, renderer: &mut Canvas<Window>) {
        let row = block.row as u32;
        let col = block.col as u32;
        let color = &block.color;

        self.draw_cell(row, col, color, renderer);
    }

    pub fn draw_cell(&self, row: u32, col: u32, color: &Cell, renderer: &mut Canvas<Window>) {
        let x = self.cell_width * col;
        let y = self.cell_width * row;

        // For now, we want random colors, to see what happens.
        let drawing_color = Color::RGB(color.red, color.green, color.blue);

        renderer.set_draw_color(drawing_color);
        let square = renderer.fill_rect(Rect::new(
            x as i32,
            y as i32,
            self.cell_width,
            self.cell_width,
        ));

        if let Err(error) = square {
            println!("{}", error);
        }
    }

    pub fn draw_snack(&self, renderer: &mut Canvas<Window>) {
        self.draw_block(&self.snack, renderer);
    }

    pub fn new_snack(&mut self) {
        let mut rng = rand::thread_rng();

        let (row, col) = loop {
            let row = rng.gen_range(0, self.ny_cells) as i32;
            let col = rng.gen_range(0, self.nx_cells) as i32;

            if !self.snake.body.contains(&Block {
                row,
                col,
                color: Cell::snake(),
            }) {
                break (row, col);
            }
        };

        self.snack = Block {
            row,
            col,
            color: Cell::snack(),
        };
    }

    pub fn draw(&mut self, renderer: &mut Canvas<Window>) {
        let bg_color = Color::RGB(20, 20, 20);

        renderer.set_draw_color(bg_color);
        renderer.clear();

        self.draw_snake(renderer);
        self.draw_snack(renderer);

        renderer.present();
    }
}
