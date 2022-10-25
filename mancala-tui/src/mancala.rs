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

pub trait GameState{
    fn generate_game_board(difficulty: Difficulty) -> [u8; BOARD_LEN] where Self: Sized{
        let mut _game_board: [u8; BOARD_LEN] = [0; BOARD_LEN];
        let mut rng = rand::thread_rng();
        
        match difficulty{
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

    fn play(&mut self);

    fn move_right(&mut self);
    fn move_left(&mut self);
}

pub struct Capture{
    pub game_board: [u8; BOARD_LEN],
    pub in_play: Side,
    pub selected_cell: u8,
}

pub struct Avalanche{
    pub game_board: [u8; BOARD_LEN],
    pub in_play: Side,
    pub selected_cell: u8,
}

impl GameState for Capture {
    fn play(&mut self){
    
    }
    // fn play(&mut self){
    //     let chosen = self.get_selected_index() as usize;
    //     let value: usize = self.game_board[chosen] as usize;
    //     self.game_board[chosen] = 0;
    //     for p in 1..=value{
    //         self.game_board[(p+chosen) % BOARD_LEN] += 1;
    //     }
    //     self.flip_sides();
    // }

    // fn flip_sides(&mut self){
    //     match self.in_play {
    //         Side::Top => self.in_play = Side::Bottom,
    //         Side::Bottom => self.in_play = Side::Top,
    //     }
    //     //self.selected_cell = 0;
    // }

    // fn get_selected_index(&self) -> u8{
    //     match self.in_play{
    //         Side::Top => 12 - self.selected_cell,
    //         Side::Bottom => self.selected_cell,
    //     }
    // }

    fn move_right(&mut self){
        if self.selected_cell < 5{
            self.selected_cell += 1
        }
    }

    fn move_left(&mut self){
        if self.selected_cell > 0{
            self.selected_cell -= 1
        }
    }
}

impl GameState for Avalanche {
    fn play(self: &mut Self) -> () {
        
    }
    
    fn move_right(&mut self){
        if self.selected_cell < 5{
            self.selected_cell += 1
        }
    }

    fn move_left(&mut self){
        if self.selected_cell > 0{
            self.selected_cell -= 1
        }
    }
}


// #[derive(Debug)]
// pub struct GameSettings{
//     pub mode: GameMode,
//     pub difficulty: Difficulty
// }

// #[derive(Debug)]
// pub struct Mancala{
//     pub game_state: GameState,
// }

// holds game data
// impl Mancala{
//     pub fn new(settings : GameSettings) -> Self{
//         Self { 
//             game_state: 
//                 match settings.mode {
//                     GameMode::Capture => {
//                         Capture::new(settings.difficulty)
//                     }
//                     GameMode::Avalanche => {
//                         Avalanche::new(settings.difficulty)
//                     }
//                 }
//         }
//     }

//     pub fn play(&mut self){
//         let chosen = self.get_selected_index() as usize;
//         let value: usize = self.game_board[chosen] as usize;
//         self.game_board[chosen] = 0;
//         for p in 1..=value{
//             self.game_board[(p+chosen) % BOARD_LEN] += 1;
//         }
//         self.flip_sides();
//     }

//     fn flip_sides(&mut self){
//         match self.in_play {
//             Side::Top => self.in_play = Side::Bottom,
//             Side::Bottom => self.in_play = Side::Top,
//         }
//         //self.selected_cell = 0;
//     }

//     // pub fn get_selected_index(&self) -> u8{
//     //     match self.in_play{
//     //         Side::Top => 12 - self.selected_cell,
//     //         Side::Bottom => self.selected_cell,
//     //     }
//     // }
//     // // gonna have to change this later
//     // pub fn has_won(&self) -> bool {
//     //     match self.game_setting.mode{
//     //         GameMode::Avalanche =>{
//     //             self.game_board[0..6].into_iter().all(|x| *x == 0) || self.game_board[8..13].into_iter().all(|x| *x == 0)
//     //         },
//     //         GameMode::Capture => {
//     //             self.game_board[0..6].into_iter().all(|x| *x == 0) || self.game_board[8..13].into_iter().all(|x| *x == 0)
//     //         }
//     //     }    
//     // }
// }

// use std::fmt;
// impl fmt::Display for Mancala{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // very bad formatting code.
//         let mut s = String::with_capacity(180);
//         s.push_str(&format!("{:?}\n  ", self.game_board));
//         for x in (7..13).rev(){
//             s.push_str(&format!("|{:2}", x));
//         }
//         s.push_str(&format!("\n{:02}|", self.game_board[13]));
//         for x in (7..13).rev(){
//             s.push_str(&format!("{:02}|", self.game_board[x]));
//         }
//         s.push_str("\n  |");
//         for x in 0..6{
//             s.push_str(&format!("{:02}|", self.game_board[x]));
//         }
//         s.push_str(&format!("{:02}|\n  ", self.game_board[6]));
//         for x in 0..6{
//             s.push_str(&format!("|{} ", x));
//         }
//         write!(f, "{}", s)
//     }
// }