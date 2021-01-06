#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() {
  let sdl = sdl2::init().expect("failed to initialize SDL");
  let video = sdl.video().expect("failed to initialize the SDL video subsystem");
  let mut canvas = {
    video.window("{{project-name}}", 800, 600)
      .resizable()
      .build()
      .expect("failed to create a window")
      .into_canvas()
      .accelerated()
      .present_vsync()
      .build()
      .expect("failed to create a canvas")
  };
  canvas.set_draw_color(Color::RGB(100,149,237));
  canvas.clear();
  canvas.present();

  let mut event_pump = sdl.event_pump().unwrap();
  'main_loop: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
        _ => {},
      }
    }

    // TODO: Render
  }
}