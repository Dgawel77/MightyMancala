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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell{
    pub side: Side,
    pub pos: usize
}

impl Cell {
    fn to_index(&self) -> usize {
        match self.side {
            Side::Top => {
                match self.pos {
                    0..=5 => 12 - self.pos,
                    6 => 13,
                    _ => unreachable!(),
                }
            },
            Side::Bottom => self.pos,
        }
    }

    fn flip_sides(&mut self){
        match self.side {
            Side::Top => self.side = Side::Bottom,
            Side::Bottom => self.side = Side::Top,
        }
    }

    fn index_to_cell(index: usize) -> Cell{
        assert!((0..14).contains(&index));
        match index {
            0..=6 => Cell{side: Side::Bottom, pos: index},
            7..=13 => {
                match index{
                    7..=12 =>  Cell{side: Side::Top, pos: (-(index as isize) + 12) as usize},
                    13 => Cell{side: Side::Top, pos: 6}, 
                    _ => unreachable!(),
                }
               
            },
            _ => unreachable!(),
        }
    }

    fn move_right(&mut self){
        if self.pos < 5{
            self.pos += 1
        }
    }

    fn move_left(&mut self){
        if self.pos > 0{
            self.pos -= 1
        }
    }
}

pub trait GameState{
    fn play(&mut self);
    // fn has_won(&self) -> bool;
    fn get_selected_cell(&self) -> Cell;
    fn get_value(&self, cell: Cell) -> usize;
    fn move_right(&mut self);
    fn move_left(&mut self);
}

pub struct Capture{
    pub game_board: [usize; BOARD_LEN],
    pub selected_cell: Cell,
}

impl GameState for Capture {
    fn play(&mut self){
        let chosen_cell: Cell = self.get_selected_cell();
        let value: usize = self.game_board[chosen_cell.to_index()];
        if (value == 0){
            return
        }
        // last index it will land on
        let end_index: usize = (chosen_cell.to_index() + value) % BOARD_LEN;
        self.game_board[chosen_cell.to_index()] = 0;
        // update the board nums
        for p in 1..=value{
            self.game_board[(p+chosen_cell.to_index()) % BOARD_LEN] += 1;
        }
        
        let end_cell: Cell = Cell::index_to_cell(end_index);
        match end_cell{
            Cell{side: _, pos: 6} => {
                if (chosen_cell.side != end_cell.side){
                    self.selected_cell.flip_sides();
                }
            },
            Cell{side: Side::Bottom, pos: _} => {
                if (chosen_cell.side == end_cell.side){
                    let opposite_index: usize = Cell{side: Side::Top, pos: end_cell.pos}.to_index();
                    if (self.game_board[opposite_index] != 0){
                        self.game_board[6] += self.game_board[opposite_index] + 1;
                        self.game_board[end_cell.pos] = 0;
                        self.game_board[opposite_index] = 0;
                    }
                }
                self.selected_cell.flip_sides();
            },
            Cell{side: Side::Top, pos: _} => {
                if (chosen_cell.side == end_cell.side){
                    let opposite_index: usize = Cell{side: Side::Bottom, pos: end_cell.pos}.to_index();
                    if (self.game_board[opposite_index] != 0){
                        self.game_board[13] += self.game_board[opposite_index] + 1;
                        self.game_board[end_cell.pos] = 0;
                        self.game_board[opposite_index] = 0;
                    }
                }
                self.selected_cell.flip_sides();
            },
        }
    }

    fn get_value(&self, cell: Cell) -> usize{
        self.game_board[cell.to_index()]
    }

    fn get_selected_cell(&self) -> Cell{
        self.selected_cell
    }

    fn move_right(&mut self){
        self.selected_cell.move_right();
    }

    fn move_left(&mut self){
        self.selected_cell.move_left();
    }
}

pub struct Avalanche{
    pub game_board: [usize; BOARD_LEN],
    pub selected_cell: Cell,
}

impl GameState for Avalanche {
    fn play(self: &mut Self) -> () {
        let chosen_cell: Cell = self.get_selected_cell();
        let value: usize = self.game_board[chosen_cell.to_index()];
        if value == 0 {
            return
        }
        // last index it will land on
        let end_index: usize = (chosen_cell.to_index() + value) % BOARD_LEN;
        self.game_board[chosen_cell.to_index()] = 0;
        // update the board nums
        for p in 1..=value{
            self.game_board[(p+chosen_cell.to_index()) % BOARD_LEN] += 1;
        }
        
        let end_cell: Cell = Cell::index_to_cell(end_index);
        match end_cell{
            Cell{side: _, pos: 6} => {
                if (chosen_cell.side != end_cell.side){
                    self.selected_cell.flip_sides();
                }
            },
            Cell{side: Side::Bottom, pos: _} => {
                if (chosen_cell.side == end_cell.side){
                    let opposite_index: usize = Cell{side: Side::Top, pos: end_cell.pos}.to_index();
                    if (self.game_board[opposite_index] != 0){
                        self.game_board[6] += self.game_board[opposite_index] + 1;
                        self.game_board[end_cell.pos] = 0;
                        self.game_board[opposite_index] = 0;
                    }
                }
                self.selected_cell.flip_sides();
            },
            Cell{side: Side::Top, pos: _} => {
                if (chosen_cell.side == end_cell.side){
                    let opposite_index: usize = Cell{side: Side::Bottom, pos: end_cell.pos}.to_index();
                    if (self.game_board[opposite_index] != 0){
                        self.game_board[13] += self.game_board[opposite_index] + 1;
                        self.game_board[end_cell.pos] = 0;
                        self.game_board[opposite_index] = 0;
                    }
                }
                self.selected_cell.flip_sides();
            },
        }
    }
    
    fn get_value(&self, cell: Cell) -> usize{
        self.game_board[cell.to_index()]
    }

    fn get_selected_cell(&self) -> Cell{
        self.selected_cell
    }

    fn move_right(&mut self){
        self.selected_cell.move_right();
    }

    fn move_left(&mut self){
        self.selected_cell.move_left();
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