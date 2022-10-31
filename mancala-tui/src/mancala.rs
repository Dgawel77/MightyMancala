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
        //let chosen_cell: usize = self.get_selected_cell() as usize;
        // let value: usize = self.game_board[chosen_cell] as usize;
        // let end_cell: usize = (chosen_cell + value) % BOARD_LEN;
        // self.game_board[chosen_cell] = 0;
        // // update the board nums
        // for p in 1..=value{
        //     self.game_board[(p+chosen_cell) % BOARD_LEN] += 1;
        // }

        // logic for capture 
        // if self.is_bowl(end_cell){ return }
        // self.capture_pieces(end_cell);
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

// impl Capture{
//     fn flip_sides(&mut self){
//         match self.in_play {
//             Side::Top => self.in_play = Side::Bottom,
//             Side::Bottom => self.in_play = Side::Top,
//         }
//     }

//     fn get_opposite_index(&self, index: usize) -> isize{
//         match index {
//             0..=5 => 12 - index as isize,
//             7..=12 => -((index - 12 ) as isize),
//             _ => unreachable!()
//         }
//     }

//     fn capture_pieces(&mut self, index: usize){
//         let opposite_side: usize = self.get_opposite_index(index) as usize;
//         match self.in_play{
//             Side::Top => {
//                 if (7..=12).contains(&index){
//                     self.game_board[13] += self.game_board[opposite_side];
//                     self.game_board[opposite_side] = 0;
//                 }
//             }
//             Side::Bottom => if (0..=5).contains(&index){
//                 self.game_board[6] += self.game_board[opposite_side];
//                 self.game_board[opposite_side] = 0;
//             }
//         }
//         self.flip_sides();
//     }

//     fn is_bowl(&self, index: usize) -> bool{
//         index == 6 || index == 13
//     }
// }

pub struct Avalanche{
    pub game_board: [usize; BOARD_LEN],
    pub selected_cell: Cell,
}

impl GameState for Avalanche {
    fn play(self: &mut Self) -> () {
        
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