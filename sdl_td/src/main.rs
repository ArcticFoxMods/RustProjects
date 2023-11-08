//https://blog.logrocket.com/using-sdl2-bindings-rust/#setting-up-rust-sdl2-project
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod game;
use game::constants;
use crate::game::context::GameContext;
use crate::game::renderer::Renderer;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
    .window("Snake Game",
     constants::GRID_X_SIZE * constants::DOT_SIZE_IN_PXS,
      constants::GRID_Y_SIZE * constants::DOT_SIZE_IN_PXS
    )
    .position_centered()
    .vulkan()
    .build()
    .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;

    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                   match keycode {
                    Keycode::W => context.move_up(),
                    Keycode::A => context.move_left(),
                    Keycode::S => context.move_down(),
                    Keycode::D => context.move_right(),
                    Keycode::Escape => context.toggle_pause(),
                    _ => {}
                   } 
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.next_tick();
            frame_counter = 0
        }
        renderer.draw(&context)?;

    }

    Ok(())

}
