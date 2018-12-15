#[macro_use]
extern crate enumset;

use std::fs;
use enumset::EnumSet;
use std::fmt;
use std::collections::HashSet;

#[derive(EnumSetType, Debug)]
pub enum Dir { N, E, S, W }

impl Dir {
    fn get_next_pos(&self, x: usize, y: usize) -> (usize, usize) {
        match &self {
            Dir::N => (x, y-1),
            Dir::E => (x+1, y),
            Dir::S => (x, y+1),
            Dir::W => (x-1, y)
        }
    }

    fn left(&self) -> Dir {
        match &self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S
        }
    }

    fn right(&self) -> Dir {
        match &self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N
        }
    }

    fn to_char(&self) -> char {
        match &self {
            Dir::N => '^',
            Dir::E => '>',
            Dir::S => 'v',
            Dir::W => '<'
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    dir: Dir,
    turns: u32,
}

impl Cart {
    fn set_pos_turn(&self,
                    x: usize,
                    y: usize) -> Cart {
        let curr_dir = self.dir.clone();
        let turn = self.turns % 3;
        let new_dir;
        if turn == 0 {
            new_dir = curr_dir.left();
        } else if turn == 1 {
            new_dir = curr_dir;
        } else if turn == 2 {
            new_dir = curr_dir.right();
        } else {
            new_dir = curr_dir;
        }

        Cart {
            id: self.id,
            x,
            y,
            dir: new_dir,
            turns: self.turns + 1
        }
    }

    fn set_pos_dir(&self,
               x: usize,
               y: usize,
               dir: Dir) -> Cart {
        Cart {
            id: self.id,
            x,
            y,
            dir,
            turns: self.turns
        }
    }
}

fn step(carts: &Vec<Cart>,
        tracks: &Vec<Vec<EnumSet<Dir>>>) -> Option<Vec<Cart>>{

    let mut todo = carts.clone();
    todo.sort_by(
        |a, b|
            a.y.cmp(&b.y)
                .then(a.x.cmp(&b.x)));

    let mut done: Vec<Cart> = Vec::new();
    let mut done_ids: HashSet<usize> = HashSet::new();
    let mut crashed: HashSet<usize> = HashSet::new();

    for cart in todo.iter() {
        if crashed.contains(&cart.id) {
            continue;
        }

        let moved_cart = move_cart(cart, &tracks);
        println!(
            "cart {}, moved from {},{} {}, to {},{} {}",
            cart.id,
            cart.x,
            cart.y,
            cart.dir,
            moved_cart.x,
            moved_cart.y,
            moved_cart.dir);

        let x = moved_cart.x;
        let y = moved_cart.y;

        {
            let crash = done.iter()
                .find(|c| c.x == x && c.y == y);
            if crash.is_some() {
                crashed.insert(crash.unwrap().id);
                crashed.insert(moved_cart.id);
                println!("--- boom! {},{} ---", x, y);
                continue;
            }

            let crash = todo.iter()
                .find(|c| c.x == x && c.y == y && !done_ids.contains(&c.id));
            if crash.is_some() {
                crashed.insert(crash.unwrap().id);
                crashed.insert(moved_cart.id);
                println!("--- boom! {},{} ---", x, y);
                continue;
            }
        }
        done_ids.insert(moved_cart.id);
        done.push(moved_cart);
    }

    println!("crashed {}", crashed.len());
    Some(done.into_iter()
        .filter(|c| !crashed.contains(&c.id))
        .collect())
}

fn main() {
    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-13\\input.txt")
        .expect("peut");

    let mut carts: Vec<Cart> = Vec::new();
    let mut tracks: Vec<Vec<EnumSet<Dir>>> = Vec::new();

    let lines: Vec<&str> = contents.lines().collect();
    for i in 0..lines.len() {
        let row: Vec<EnumSet<Dir>> = get_tiles_from_string(i,lines[i], &mut carts);
        tracks.push(row);
    }

    let mut todo_carts_opt = Some(carts.clone());
    let mut i = 0;
    while todo_carts_opt.is_some() {
        let todo_carts = todo_carts_opt.unwrap();

        println!("\nstep {}, carts {}", i, &todo_carts.len());
        todo_carts_opt = step(&todo_carts, &tracks).clone();
        i += 1;

        if todo_carts.len() < 2 {
            break;
        }
    }
}

fn get_tiles_from_string(row: usize,
                         str: &str,
                         carts: &mut Vec<Cart>) -> Vec<EnumSet<Dir>> {
    let mut result: Vec<EnumSet<Dir>> = Vec::new();
    let mut prev_char = ' ';
    for i in str.char_indices() {
        let tc_dirs = get_possible_dirs(i.1, prev_char);
        if tc_dirs.1.is_some() {
            let id = carts.len();
            carts.push(
                Cart {
                    id,
                    x: i.0,
                    y: row,
                    dir: tc_dirs.1.unwrap(),
                    turns: 0
                }
            )
        }
        result.push(tc_dirs.0);

        prev_char = i.1;
    }
    result
}

fn move_cart(cart: &Cart,
            tracks: &Vec<Vec<EnumSet<Dir>>>) -> Cart {

    let new_pos = cart.dir.get_next_pos(cart.x, cart.y);
    let track = tracks[new_pos.1][new_pos.0];

    if track.is_empty() {
        println!("cart {} derail", cart.id);
        return cart.clone();
    } else if track.len() <=  2 {
        if !track.contains(cart.dir) {
            if track.contains(cart.dir.left()) {
                println!("cart {} left", cart.id);
                return cart.set_pos_dir(new_pos.0, new_pos.1, cart.dir.left());
            } else if track.contains(cart.dir.right()) {
                println!("cart {} right", cart.id);
                return cart.set_pos_dir(new_pos.0, new_pos.1, cart.dir.right());
            } else {
                println!("cart {} wut", cart.id);
                return cart.clone();
            }
        } else {
            println!("cart {} straight", cart.id);
            return cart.set_pos_dir(new_pos.0, new_pos.1, cart.dir);
        }
    } else {
        println!("cart {} crossroad", cart.id);
        return cart.set_pos_turn(new_pos.0, new_pos.1);
    }
}

fn get_possible_dirs(track: char, prev_track: char) -> (EnumSet<Dir>, Option<Dir>) {
    if track == ' ' {
        (EnumSet::empty(), None)
    } else if track == '-' {
        (Dir::E | Dir::W, None)
    } else if track ==  '<' {
        (Dir::E | Dir::W, Some(Dir::W))
    } else if track ==  '>' {
        (Dir::E | Dir::W, Some(Dir::E))
    } else if track == '|' {
        (Dir::N | Dir::S, None)
    } else if track == '^' {
        (Dir::N | Dir::S, Some(Dir::N))
    } else if track == 'v' {
        (Dir::N | Dir::S, Some(Dir::S))
    } else if track == '\\' {
        if prev_track == '-' || prev_track == '+' || prev_track == '<' || prev_track == '>' {
            (Dir::S | Dir::W, None)
        } else {
            (Dir::N | Dir::E, None)
        }
    } else if track  == '/' {
        if prev_track == '-' || prev_track == '+' || prev_track == '<' || prev_track == '>' {
            (Dir::N | Dir::W, None)
        } else {
            (Dir::S | Dir::E, None)
        }
    } else if track == '+' {
        (EnumSet::all(), None)
    } else {
        println!("nooooo broken track '{}'", track);
        (EnumSet::empty(), None)
    }
}
