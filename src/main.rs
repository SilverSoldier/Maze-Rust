extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::colors::Color;

const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;
// set frames per second limit
const LIMIT_FPS: i32 = 20; 

struct Object{
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object{
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x : x,
            y: y,
            char : char,
            color : color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn erase(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

/* Function to handle keyboard input
 * Input: root of type tcod::console::Root mutable since we need to redraw @, player_x and
 * player_y of type i32 mutable since we need to modify it as the arrow keys are pressed.
 * Output: bool: true -> exit game, false -> continue
 */
fn keyboard_input(root: &mut Root, player: &mut Object) -> bool{
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = root.wait_for_keypress(true); //turn based otherwise real time
    match key {
        // code tells whether up or down etc.
        // .. => ignore other fields
        // Escape exits game
        Key {code: Escape, ..} => return true,

        Key {code: Up, ..} => player.move_by(0, -1),
        Key {code: Down, ..} => player.move_by(0, 1),
        Key {code: Left, ..} => player.move_by(-1, 0),
        Key {code: Right, ..} => player.move_by(1, 0),

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

    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Object::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, '@', colors::WHITE);

    while !root.window_closed() {
        // con.set_default_foreground(colors::WHITE);

        // BackgroundFlag::None => ignore default background color
        player.draw(&mut con);

        blit(&mut con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush(); // draws everything on the screen at once

        player.erase(&mut con);

        let end_game = keyboard_input(&mut root, &mut player);
        if end_game {
            break;
        }

    }
}
