mod game;
mod renderer;

use game::Game;
use wasm_bindgen::prelude::*;

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut GAME: Option<Game> = None;

// TODO enum Sounds {}

#[wasm_bindgen]
extern "C" {
    fn js_play_sound(id: i32);
}

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    unsafe { GAME = Some(Game::new()) }
}

#[wasm_bindgen]
pub fn frame() {
    unsafe {
        GAME.as_mut().unwrap().frame();
    }
}

#[wasm_bindgen]
pub fn send_key_down(code: u32) {
    unsafe {
        GAME.as_mut().unwrap().send_key_down(code);
    }
}

#[wasm_bindgen]
pub fn send_key_up(code: u32) {
    unsafe {
        GAME.as_mut().unwrap().send_key_up(code);
    }
}
