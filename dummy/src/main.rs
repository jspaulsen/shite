use std::slice::Iter;

use sdl2::pixels::Color;

use shite::game::GameEngine;

fn main() {
    let mut game: GameEngine = GameEngine::new(
        "Poopster",
        800,
        600,
    ).unwrap();

    loop {
        let mut window = game.window_handler.get_mut_window().unwrap().get_canvas_mut();

        window.clear();

        window.set_draw_color(
            Color::RGB(255, 0, 0),
        );

        window.present();
    }
}
