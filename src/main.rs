extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;
// set frames per second limit
const LIMIT_FPS: i32 = 20; 

fn main() {
    let mut root: Root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Maze Game")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(1, 1, '@', BackgroundFlag::None);
        // BackgroundFlag::None => ignore default background color
        
        root.flush(); // draws everything on the screen at once
        root.wait_for_keypress(true);
    }
}
