extern crate tcod;
extern crate rand;

use tcod::console::*;
use tcod::colors;
use tcod::colors::Color;
use rand::Rng;

const SCREEN_HEIGHT: i32 = 70;
const SCREEN_WIDTH: i32 = 130;

const MAP_HEIGHT: i32 = 60;
const MAP_COLS: i32 = 15;

const MAP_WIDTH: i32 = 120;
const MAP_ROWS: i32 = 30;


const CELL_SIZE: i32 = 4;

// set frames per second limit
const LIMIT_FPS: i32 = 20; 

// derive bit automatically implements certain behaviours - traits
// Debug: lets us print tile's contents
// Clone and copy - let us copy values instead of moving it
#[derive(Clone, Copy, Debug)]
struct Tile{
    is_wall: bool,
}

impl Tile{
    pub fn empty() -> Self{
        Tile{is_wall: false}
    }

    pub fn make_wall(&mut self) ->(){
        self.is_wall = true;
    }
}

#[derive(Clone, Copy, Debug)]
struct Cell{
    top: bool,
    right: bool,
    left: bool,
    down: bool,
}

impl Cell{
    pub fn new() -> Self{
        Cell{
            top: true,
            right: true,
            left: true,
            down: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct KruskalCell{
    cell: Cell,
    value: i32,
}

impl KruskalCell {
    pub fn new() -> Self{
        KruskalCell{
            cell: Cell::new(),
            value: 0,
        }
    }
}

/* Function to merge 2 sets by setting their value to be 1 */
fn merge(ref mut set: &mut Vec<Vec<KruskalCell>>, val1: i32, val2: i32) -> () {
    for y in 0..MAP_COLS{
        for x in 0..MAP_ROWS{
            if set[x as usize][y as usize].value == val1 {
                set[x as usize][y as usize].value = val2;
            }
        }
    }
}

/* Function to check if all the elements of the set have been merged */
fn count_is_1(ref set: &Vec<Vec<KruskalCell>>) -> bool{
    let val: i32 = set[0][0].value;
    for y in 0..MAP_COLS{
        for x in 0..MAP_ROWS{
            if set[x as usize][y as usize].value != val{
                return false;
            }
        }
    }
    return true;
}

/* Function that generates the maze using kruskal's algorithm
 * Input: -
 * Output: 2D vector of objects of type Cell
 */
fn kruskal_map() -> Vec<Vec<Cell>> {
    let mut cell_set = vec![vec![KruskalCell::new(); MAP_COLS as usize]; MAP_ROWS as usize];
    let mut val: i32 = 0;

    // initilialize each as having unique number
    for x in 0..MAP_ROWS{
        for y in 0..MAP_COLS{
            cell_set[x as usize][y as usize].value = val;
            val += 1;
        }
    }

    // initialize list of walls
    let mut wall_list: Vec<(i32, i32, char)> = Vec::new();

    for x in 0..MAP_ROWS{
        for y in 0..MAP_COLS{
            wall_list.push((x, y, 'L'));
            wall_list.push((x, y, 'U'));
        }
    }

    while !count_is_1(&cell_set) {
        // pick a random wall from wall list
        let index = rand::thread_rng().gen_range(0, wall_list.len());
        let wall = wall_list[index as usize];
        wall_list.remove(index);
        let x = wall.0 as usize;
        let y = wall.1 as usize;
            
        if y > 0 && wall.2 == 'L' && cell_set[x][y].value != cell_set[x][y-1].value{
            cell_set[x][y].cell.left = false;
            cell_set[x][y-1].cell.right = false;
            let val1 = cell_set[x][y].value;
            let val2 = cell_set[x][y-1].value;
            merge(&mut cell_set, val1, val2);
        }

        if x > 0 && wall.2 == 'U' && cell_set[x][y].value != cell_set[x-1][y].value{
            cell_set[x][y].cell.top = false;
            cell_set[x-1][y].cell.down = false;
            let val1 = cell_set[x][y].value;
            let val2 = cell_set[x-1][y].value;
            merge(&mut cell_set, val1, val2);
        }
    }

    let mut cells = vec![vec![Cell::new(); MAP_COLS as usize]; MAP_ROWS as usize];

    for x in 0..MAP_ROWS{
        for y in 0..MAP_COLS{
            cells[x as usize][y as usize] = cell_set[x as usize][y as usize].cell;
        }
    }

    return cells;

}

type Map = Vec<Vec<Tile>>;

fn make_map(ref cells :&Vec<Vec<Cell>>) -> Map {
    // initialize with unblocked tiles using the vec! macro
    // syntax vec![val, no. of times]   no. of times is expected as usize not i32
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    for x in 0..MAP_ROWS{
        for y in 0..MAP_COLS{
            // println!("{}", x);
            map[(x * CELL_SIZE) as usize][(y * CELL_SIZE) as usize].make_wall();
            map[(x * CELL_SIZE) as usize][((y+1) * CELL_SIZE - 1) as usize].make_wall();
            map[((x+1) * CELL_SIZE - 1) as usize][(y * CELL_SIZE) as usize].make_wall();
            map[((x+1) * CELL_SIZE - 1) as usize][((y+1) * CELL_SIZE - 1) as usize].make_wall();

            for k in 1..(CELL_SIZE -1) {
                // println!("{}", k);
                map[(x * CELL_SIZE) as usize][(y * CELL_SIZE + k) as usize].is_wall = cells[x as usize][y as usize].top;
                map[(x * CELL_SIZE + k) as usize][(y * CELL_SIZE) as usize].is_wall = cells[x as usize][y as usize].left;
                map[(x * CELL_SIZE + k) as usize][((y+1) * CELL_SIZE - 1) as usize].is_wall = cells[x as usize][y as usize].right;
                map[((x+1) * CELL_SIZE - 1) as usize][(y * CELL_SIZE + k) as usize].is_wall = cells[x as usize][y as usize].down;
            }
        }
    }

    // map[10][10] = Tile::wall();
    // map[20][20] = Tile::wall();

    map
}

fn render_all(root: &mut Root, con: &mut Offscreen, player: &Object, map: &Map){
    player.draw(con);

    // draw map
    for j in 0..MAP_HEIGHT {
        for i in 0..MAP_WIDTH {
            let display_wall = map[i as usize][j as usize].is_wall;
            if display_wall {
                con.put_char_ex(i, j, '#', colors::WHITE, colors::BLACK);
                // con.set_char_background(i, j, COLOR_WALL_DARK, BackgroundFlag::Set);
            }
            else {
                // con.set_char_background(i, j, COLOR_FLOOR_DARK, BackgroundFlag::Set);
            }
        }
    }
    blit(con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), root, (0, 0), 1.0, 1.0);
}

#[derive(Debug)]
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

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].is_wall {
            self.x += dx;
            self.y += dy;
        }
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
fn keyboard_input(root: &mut Root, player: &mut Object, map: &Map) -> bool{
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = root.wait_for_keypress(true); //turn based otherwise real time
    match key {
        // code tells whether up or down etc.
        // .. => ignore other fields
        // Escape exits game
        Key {code: Escape, ..} => return true,

        Key {code: Up, ..} => player.move_by(0, -1, map),
        Key {code: Down, ..} => player.move_by(0, 1, map),
        Key {code: Left, ..} => player.move_by(-1, 0, map),
        Key {code: Right, ..} => player.move_by(1, 0, map),

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

    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    tcod::system::set_fps(LIMIT_FPS);

    let cells: Vec<Vec<Cell>>;
    cells = kruskal_map();
    let map = make_map(&cells);
    let mut player = Object::new(1, 1, '@', colors::RED);

    while !root.window_closed() {

        render_all(&mut root, &mut con, &player, &map);

        // BackgroundFlag::None => ignore default background color

        root.flush(); // draws everything on the screen at once

        player.erase(&mut con);

        let end_game = keyboard_input(&mut root, &mut player, &map);
        if end_game {
            break;
        }

    }
}
