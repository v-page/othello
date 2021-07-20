#[derive(Copy, Clone, PartialEq,Debug)]
pub enum Player {
    Empty = 0,
    Black,
    White,
}

fn enermy_player(player: Player) -> Player {
    if player == Player::Black {
        Player::White
    } else if player == Player::White {
        Player::Black
    } else {
        panic!("emptyは指定できません")
    }
}

pub type OthelloBoard = Vec<Vec<Player>>;

pub fn init() -> OthelloMain {
    let board: OthelloBoard = default_board();
    return OthelloMain {
        board: Board { board },
        turn: Player::Black,
    };
}

fn default_board() -> OthelloBoard {
    let mut output = vec![vec![Player::Empty; 8]; 8];
    output[3][3] = Player::White;
    output[3][4] = Player::Black;
    output[4][3] = Player::Black;
    output[4][4] = Player::White;
    return output;
}

#[derive(Debug,Clone)]
pub struct OthelloMain {
    pub board: Board,
    pub turn: Player,
}

#[derive(Debug)]
pub enum Msg {
    UnableToPlace = 0,
    OutOfBoard,
    GameOver,
    Pass,
    NextTurn,
}

impl OthelloMain {
    fn is_placeable_to(&self, x: usize, y: usize, turn: Player) -> Result<bool, &str> {
        if self.board.get(x, y).expect("") != Player::Empty {
            return Ok(false);
        }

        for arround_x in -1..=1 {
            for arround_y in -1..=1 {
                if arround_x == 0 && arround_y == 0 {
                    continue;
                }

                let mut i = 0;
                loop {
                    i += 1;

                    let checking_x = (x as i32) + arround_x * i;
                    let checking_y = (y as i32) + arround_y * i;

                    if self.is_in_board(checking_x as isize, checking_y as isize) {
                        break;
                    }

                    let checking_x = checking_x as usize;
                    let checking_y = checking_y as usize;

                    let checking_cell = self
                        .board
                        .get(checking_x, checking_y)
                        .expect("内部エラー:範囲外アクセス");

                    if i == 1 {
                        if checking_cell == enermy_player(turn) {
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        if checking_cell == enermy_player(turn) {
                            continue;
                        } else if checking_cell == turn {
                            return Ok(true);
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    fn is_placeable_to_somewhere(&self, turn: Player) -> bool {
        for x in 0..=7 {
            for y in 0..=7 {
                if self.is_placeable_to(x, y, turn).unwrap_or(false) {
                    return true;
                }
            }
        }
        return false;
    }

    fn reverse(&mut self, x: usize, y: usize, turn: Player) -> Result<(), &str> {
        if !self.is_placeable_to(x, y, turn).unwrap_or(false) {
            return Err("");
        }

        let mut place_to_reverse = vec![vec![x, y]];

        for arround_x in -1..=1 {
            for arround_y in -1..=1 {
                if arround_x == 0 && arround_y == 0 {
                    continue;
                }

                let mut tmp_place_to_reverse = vec![];

                let mut i = 0;
                loop {
                    i += 1;
                    let checking_x = (x as i32) + arround_x * i;
                    let checking_y = (y as i32) + arround_y * i;

                    if self.is_in_board(checking_x as isize, checking_y as isize) {
                        break;
                    }

                    let checking_x = checking_x as usize;
                    let checking_y = checking_y as usize;

                    let checking_cell = self
                        .board
                        .get(checking_x, checking_y)
                        .expect("内部エラー:範囲外アクセス");

                    if i == 1 {
                        //周囲一つ目の駒が敵の駒の場合
                        if checking_cell == enermy_player(turn) {
                            tmp_place_to_reverse.push(vec![checking_x, checking_y]);
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        //周囲iつ目の駒が敵の駒の場合
                        if checking_cell == enermy_player(turn) {
                            tmp_place_to_reverse.push(vec![checking_x, checking_y]);
                            continue;

                        //周囲iつ目の駒が味方の駒の場合
                        } else if checking_cell == turn {
                            place_to_reverse.extend(tmp_place_to_reverse);
                            break;

                        //周囲iつ目の駒が置かれていないとき
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        for x in place_to_reverse{
            self.board.set(x[0], x[1], turn).unwrap();
        }

        Ok(())
    }

    // fn is_in_board(x:usize,y:usize) -> bool{
    //     return x < 0 || y < 0 || x > 7 || y > 7
    // }

    fn is_in_board(&self, x: isize, y: isize) -> bool {
        return x < 0 || y < 0 || x > 7 || y > 7;
    }

    fn change_turn(&mut self) {
        self.turn = enermy_player(self.turn);
    }

    // [[x,y],[x,y]..=.[x,y]]
    pub fn get_placeable_list(&self, turn: Player) -> Vec<Vec<usize>> {
        let mut output = vec![];
        for x in 0..=7 {
            for y in 0..=7 {
                if self.is_placeable_to(x, y, turn).unwrap_or(false) {
                    output.push(vec![x, y]);
                }
            }
        }
        return output;
    }

    pub fn set_board(&mut self, x: i32, y: i32) -> Msg {
        let x = x as usize;
        let y = y as usize;

        if self.is_in_board(x as isize, y as isize) {
            return Msg::OutOfBoard;
        }

        if !self.is_placeable_to(x, y, self.turn).unwrap_or(false) {
            return Msg::UnableToPlace;
        }

        self.reverse(x, y, self.turn).unwrap();

        let player_placeable = self.is_placeable_to_somewhere(self.turn);
        let enermy_placeable = self.is_placeable_to_somewhere(enermy_player(self.turn));

        if !player_placeable && !enermy_placeable {
            return Msg::GameOver;
        } else if !enermy_placeable {
            return Msg::Pass;
        } else {
            self.change_turn();
            return Msg::NextTurn;
        }
    }

    pub fn count_cell(&self,player:Player) -> u32{
        let mut cell = 0;
        for x in 0..=7 {
           for y in 0..=7 {
                if self.board.get(x, y).unwrap() == player{
                    cell += 1;
                }
           } 
        }
        return cell;
    }
}

#[derive(Debug,Clone)]
pub struct Board {
    pub board: OthelloBoard,
}

impl Board {
    pub fn get(&self, x: usize, y: usize) -> Result<Player, &str> {
        let cell = self.board.get(x).ok_or("")?.get(y).ok_or("")?;
        Ok(*cell)
    }

    fn set(&mut self, x: usize, y: usize, val: Player) -> Result<(), &str> {
        if self.board.len() < x {
            return Err("");
        }
        if self.board[0].len() < y {
            return Err("");
        }

        self.board[x][y] = val;
        Ok(())
    }
}
