use crate::renderer::Renderer;
use crate::{console_log, js_play_sound};
use web_sys::window;

pub struct Game {
    renderer: Renderer,
    last_timestamp: f32,
    t: f32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            renderer: Renderer::new(),
            last_timestamp: window().unwrap().performance().unwrap().now() as f32,
            t: 0.0,
        }
    }

    pub fn frame(&mut self) {
        let new_timestamp = window().unwrap().performance().unwrap().now() as f32;
        let dt = new_timestamp - self.last_timestamp;
        self.last_timestamp = new_timestamp;

        self.t += dt;

        self.renderer.draw_frame(self.t);
    }

    pub fn send_key_down(&mut self, code: u32) {
        js_play_sound((100f64 * js_sys::Math::random()) as i32);
        console_log!("Key down: {:?}", code);
    }

    pub fn send_key_up(&mut self, code: u32) {
        console_log!("Key up: {:?}", code);
    }
}
