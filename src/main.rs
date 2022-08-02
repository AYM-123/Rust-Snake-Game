pub mod lib;

use lib::game::{Game, GameState};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time;

// this is main
fn main() {
    
    let canvas_width = 720u32;
    let canvas_height = 720u32;
    
    let rows = 18;
    let columns = 18;
    
    let cell_width = 40;
    
    let (mut canvas, mut events) = lib::init(canvas_height, canvas_width);
    
    let mut game = Game::new(rows, columns, cell_width, (1, 0));
    let mut highscore = 0;

    let mut direction = (1, 0);
    
    thread::spawn(move || {});

    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    direction = (-1, 0);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    direction = (1, 0);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    direction = (0, -1);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    direction = (0, 1);
                }

                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                _ => (),
            }
        }

        if let GameState::Over(score) = game.move_snake(direction) {
            highscore = std::cmp::max(highscore, score);
            break 'game;
        }
        game.draw(&mut canvas);

        thread::sleep(time::Duration::from_millis(100));
    }

    println!("you got a score of {}", highscore);

}
