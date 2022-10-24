use rand::prelude::*;

const BOARD_LEN: usize = 14;

#[derive(Debug)]
pub enum GameMode {
    Capture,
    Avalanche,
}

#[derive(Debug)]
pub enum Difficulty {
    Normal,
    Random,
}

#[derive(Debug)]
pub enum Side {
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct GameSettings{
    pub mode: GameMode,
    pub difficulty: Difficulty
}

#[derive(Debug)]
pub struct Mancala{
    pub game_board: [u8; BOARD_LEN],
    pub game_setting: GameSettings,
    pub in_play: Side,
    pub selected_cell: u8,
}

// holds game data
impl Mancala{
    pub fn new(settings : GameSettings) -> Self{
        Self { 
            game_board: Mancala::generate_game_board(&settings), 
            game_setting: settings,
            in_play: Side::Bottom,
            selected_cell: 0,
        }
    }

    fn generate_game_board(settings : &GameSettings) -> [u8; BOARD_LEN]{
        let mut _game_board: [u8; BOARD_LEN] = [0; BOARD_LEN];
        let mut rng = rand::thread_rng();
        
        match settings.difficulty{
            Difficulty::Normal => {
                for p in 0..7{
                    _game_board[p] = 4;
                    _game_board[p+7] = 4;
                }
                
            }
            Difficulty::Random =>{
                for p in 0..7{
                    let random: u8 = rng.gen_range(1..6);
                    _game_board[p] = random;
                    _game_board[p+7] = random;
                }
            }
        };
        // set the cups to 0
        _game_board[6] = 0;
        _game_board[13] = 0;
        _game_board
    }

    pub fn play(&mut self) -> (){
        let chosen = self.get_selected_index() as usize;
        let value: usize = self.game_board[chosen] as usize;
        self.game_board[chosen] = 0;
        for p in 1..=value{
            self.game_board[(p+chosen) % BOARD_LEN] += 1;
        }
    }

    pub fn get_selected_index(&self) -> u8{
        match self.in_play{
            Side::Top => 12 - self.selected_cell,
            Side::Bottom => self.selected_cell,
        }
    }

    pub fn move_right(&mut self){
        if self.selected_cell < 5{
            self.selected_cell += 1
        }
    }

    pub fn move_left(&mut self){
        if self.selected_cell > 0{
            self.selected_cell -= 1
        }
    }
    // // gonna have to change this later
    // pub fn has_won(&self) -> bool {
    //     match self.game_setting.mode{
    //         GameMode::Avalanche =>{
    //             self.game_board[0..6].into_iter().all(|x| *x == 0) || self.game_board[8..13].into_iter().all(|x| *x == 0)
    //         },
    //         GameMode::Capture => {
    //             self.game_board[0..6].into_iter().all(|x| *x == 0) || self.game_board[8..13].into_iter().all(|x| *x == 0)
    //         }
    //     }    
    // }
}

use std::fmt;
impl fmt::Display for Mancala{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // very bad formatting code.
        let mut s = String::with_capacity(180);
        s.push_str(&format!("{:?}\n  ", self.game_board));
        for x in (7..13).rev(){
            s.push_str(&format!("|{:2}", x));
        }
        s.push_str(&format!("\n{:02}|", self.game_board[13]));
        for x in (7..13).rev(){
            s.push_str(&format!("{:02}|", self.game_board[x]));
        }
        s.push_str("\n  |");
        for x in 0..6{
            s.push_str(&format!("{:02}|", self.game_board[x]));
        }
        s.push_str(&format!("{:02}|\n  ", self.game_board[6]));
        for x in 0..6{
            s.push_str(&format!("|{} ", x));
        }
        write!(f, "{}", s)
    }
}