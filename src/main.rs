extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;
// set frames per second limit
const LIMIT_FPS: i32 = 20; 

/* Function to handle keyboard input
 * Input: root of type tcod::console::Root mutable since we need to redraw @, player_x and
 * player_y of type i32 mutable since we need to modify it as the arrow keys are pressed.
 * Output: bool: true -> exit game, false -> continue
 */
fn keyboard_input(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool{
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = root.wait_for_keypress(true); //turn based otherwise real time
    match key {
        // code tells whether up or down etc.
        // .. => ignore other fields
        Key {code: Escape, ..} => return true,

        Key {code: Up, ..} => *player_y -= 1,
        Key {code: Down, ..} => *player_y += 1,
        Key {code: Left, ..} => *player_x -= 1,
        Key {code: Right, ..} => *player_x += 1,
        // Escape exits game

        _ => {}

    };
    return false;
}

fn main() {
    let mut root: Root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Maze Game")
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH/2;
    let mut player_y = SCREEN_HEIGHT/2;

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);

        // BackgroundFlag::None => ignore default background color
        root.put_char(player_x, player_y, '@', BackgroundFlag::None);
        root.flush(); // draws everything on the screen at once

        root.put_char(player_x, player_y, ' ', BackgroundFlag::None);

        let end_game = keyboard_input(&mut root, &mut player_x, &mut player_y);
        if end_game {
            break;
        }

    }
}
