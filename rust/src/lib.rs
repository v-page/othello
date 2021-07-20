mod othello;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

static OTHELLO: OnceCell<Mutex<othello::OthelloMain>> = OnceCell::new();

#[wasm_bindgen]
pub fn othello_init() {
    OTHELLO.set(Mutex::new(othello::init())).unwrap();
}

#[wasm_bindgen]
pub fn othello_board() -> String {
    let mut output = String::new();
    for x in 0..=7 {
        for y in 0..=7 {
            output += &(get_othello().board.get(x, y).unwrap() as i32).to_string();
        }
        if x != 7 {
            output += "\n";
        }
    }
    return output;

    // get_othello().board.board.iter().map(|x| {
    //     x.iter().map(|y|{
    //         *y as i32
    //     }).collect()
    // }).collect()
}

#[wasm_bindgen]
pub fn othello_count_cell(player_num: i32) -> i32 {
    let player = match player_num {
        0 => othello::Player::Empty,
        1 => othello::Player::Black,
        2 => othello::Player::White,
        _ => return -1,
    };
    get_othello().count_cell(player) as i32
}

#[wasm_bindgen]
pub fn othello_turn() -> i32 {
    get_othello().turn as i32
}

#[wasm_bindgen]
pub fn othello_placeable_list() -> String {
    let mut output = String::new();
    let othello_tmp = &get_othello();
    let list = othello_tmp.get_placeable_list(othello_tmp.turn);
    for (x_i, x) in list.iter().enumerate() {
        for y in x {
            output += &format!("{}", *y as i32);
        }
        if x_i + 1 != list.len() {
            output += "\n";
        }
    }
    return output;
}

#[wasm_bindgen]
pub fn othello_set(x: i32, y: i32) -> i32 {
    let mut tmp = get_othello();
    let output = tmp.set_board(x, y);
    // OTHELLO.set(tmp).unwrap();
    return output as i32;
}

pub fn get_othello() -> std::sync::MutexGuard<'static, othello::OthelloMain> {
    if_no_inited_then_init();
    OTHELLO.get().unwrap().try_lock().unwrap()
}

fn if_no_inited_then_init() {
    if is_no_inited() {
        othello_init()
    }
}

fn is_no_inited() -> bool {
    !OTHELLO.get().is_some()
}
