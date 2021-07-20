// struct
mod othello;
use regex::Regex;
use std::io;

fn main() {
    let mut game_othello = othello::init();


    // let a = [2,4];
    // println!("{:?}",a);

    loop {
        let turn_str = player_str(game_othello.turn);
        println!("{}のターン",&turn_str);
        println!("{}", print_board(&game_othello.board));
        let input_raw = &stdin();
        let input_r = parse_input(input_raw);
        let input;
        if (&input_r).is_ok() {
            input = input_r.unwrap()
        }else{
            println!("入力に誤りがあります");
            continue;
        }

        // debug
        // let ls = game_othello.get_placeable_list(game_othello.turn);
        // let input = &ls.get(8
        // ).unwrap_or(&ls.get(0).unwrap());

        let msg = game_othello.set_board(input[0] as i32, input[1] as i32);
        match msg{
            othello::Msg::NextTurn => (),
            othello::Msg::GameOver => break,
            othello::Msg::OutOfBoard => {println!("盤外です");continue;},
            othello::Msg::UnableToPlace => {println!("この位置には置けません");continue;},
            othello::Msg::Pass => {println!("パスです。");},
        }
        
    }

    let black_cell = game_othello.count_cell(othello::Player::Black);
    let white_cell = game_othello.count_cell(othello::Player::White);

    let winner_msg = 
    if black_cell > white_cell {
        "黒の勝ち"
    } else if black_cell < white_cell{
        "白の勝ち"
    } else {
        "引き分け"
    };
    println!("{}", print_board(&game_othello.board));
    println!("終了");
    println!("黒:{}、白:{}で{}",black_cell,white_cell,winner_msg);
}


fn print_board(board: &othello::Board) -> String {
    let x_list = ["A","B","C","D","E","F","G","H"];
    let y_list:Vec<String> = [1,2,3,4,5,6,7,8].iter().map(|x| x.to_string()).collect();
    let mut output = String::new();

    output += &format!("_|{}_\n",x_list.join("_|"));
    for x in 0..=7 {
        output += &y_list[x];

        for y in 0..=7 {
            output += "|";
            let cell = board.get(x, y).unwrap();
            let cell_char = match cell {
                othello::Player::Black => "O",
                othello::Player::White => "X",
                othello::Player::Empty => "_",
            };
            output += "_";
            output += cell_char;
        }
        output += "\n"
    }
    return output;
}

fn parse_input(input: &str) -> Result<Vec<usize>, &str> {
    let input = &input.trim().to_uppercase();
    if !Regex::new(r"^[A-H][1-8]$").unwrap().is_match(input) {
        return Err("入力に誤りがあります");
    }
    let splited_input: Vec<&str> = input.split("").collect();
    let y = splited_input.get(1).unwrap();
    let x = splited_input.get(2).unwrap();

    let usize_y: usize = y.as_bytes()[0] as usize - 64;
    let usize_x: usize = x.parse().unwrap();

    Ok(vec![usize_x - 1, usize_y - 1])
}

fn stdin() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    return input;
}

fn player_str(player:othello::Player) ->&'static str{
    match player{
        othello::Player::Black => "黒",
        othello::Player::White => "白",
        othello::Player::Empty => "空"
    } 
}
