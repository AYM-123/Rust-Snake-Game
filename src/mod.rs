use sdl2::{pixels::Color, render::Canvas, video::Window, EventPump};
pub mod game;
pub mod types;

// this function initializes the canvas
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}
