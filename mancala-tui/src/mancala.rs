use rand::prelude::*;
use core::ops::IndexMut;

const BOARD_LEN: usize = 14;

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Capture,
    Avalanche,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Normal,
    Random,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Top,
    Bottom,
}

// index works on the 0-13 system 
// 0-5 bottom row going right 
// 6 right bowl 
// 7-12 top row going Left
// 13 left bowl
pub trait GameState{
    fn play(&mut self);
    fn get_selected_index(&self) -> u8;
    fn get_value(&self, pos: usize) -> u8;
    fn move_right(&mut self);
    fn move_left(&mut self);
}

pub struct Capture{
    pub game_board: [u8; BOARD_LEN],
    pub in_play: Side,
    pub selected_cell: u8,
}

impl GameState for Capture {
    fn play(&mut self){
        let chosen_cell: usize = self.get_selected_index() as usize;
        let value: usize = self.game_board[chosen_cell] as usize;
        let end_cell: usize = (chosen_cell + value) % BOARD_LEN;
        self.game_board[chosen_cell] = 0;
        for p in 1..=value{
            self.game_board[(p+chosen_cell) % BOARD_LEN] += 1;
        }

        if end_cell != 6 && end_cell != 13{
            self.flip_sides();
        }
        // add the actual capture functionality
    }

    fn get_value(&self, pos: usize) -> u8{
        self.game_board[pos]
    }

    fn get_selected_index(&self) -> u8{
        match self.in_play {
            Side::Top => 12 - self.selected_cell,
            Side::Bottom => self.selected_cell,
        }
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

impl Capture{
    fn flip_sides(&mut self){
        match self.in_play {
            Side::Top => self.in_play = Side::Bottom,
            Side::Bottom => self.in_play = Side::Top,
        }
    }
}

pub struct Avalanche{
    pub game_board: [u8; BOARD_LEN],
    pub in_play: Side,
    pub selected_cell: u8,
}

impl GameState for Avalanche {
    fn play(self: &mut Self) -> () {
        
    }
    
    fn get_value(&self, pos: usize) -> u8{
        self.game_board[pos]
    }

    fn get_selected_index(&self) -> u8{
        match self.in_play {
            Side::Top => 13 - self.selected_cell,
            Side::Bottom => self.selected_cell,
        }
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