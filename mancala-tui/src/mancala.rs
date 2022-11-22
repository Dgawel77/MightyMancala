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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn has_won(&self) -> bool;
    fn winner(&self) -> Side;
    fn get_selected_cell(&self) -> Cell;
    fn get_value(&self, cell: Cell) -> usize;
    fn move_right(&mut self);
    fn move_left(&mut self);
    fn set_selected(&mut self, cell: Cell);
}

pub struct Capture{
    pub game_board: [usize; BOARD_LEN],
    pub selected_cell: Cell,
}

impl GameState for Capture {
    fn play(&mut self){
        let chosen_cell: Cell = self.get_selected_cell();
        let chosen_cell_index = chosen_cell.to_index();
        let chosen_cell_value: usize = self.game_board[chosen_cell_index];
        if chosen_cell_value == 0 {
            return
        }
        self.game_board[chosen_cell_index] = 0;
        
        // update the board values, skip giving stone to the enemy
        let mut current_position = chosen_cell_index;
        let mut value = chosen_cell_value;
        while value != 0{
            current_position = (current_position + 1) % BOARD_LEN;
            if chosen_cell.side == Side::Bottom && current_position == 13 {
                continue;
            }
            if chosen_cell.side == Side::Top && current_position == 6 {
                continue;
            }
            self.game_board[current_position] += 1;
            value -= 1;
        }
        
        // capture mechanic
        let end_cell: Cell = Cell::index_to_cell(current_position);
        let end_cell_value: usize = self.game_board[current_position];
        let opposite_side = match end_cell.side {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
        };
        // we can capture
        if end_cell_value == 1 && end_cell.side == self.selected_cell.side && end_cell.pos != 6{
            let opposite_index: usize = Cell{side: opposite_side, pos: end_cell.pos}.to_index();
            let captured_stones =  self.game_board[opposite_index] + end_cell_value;
            self.game_board[opposite_index] = 0;
            self.game_board[end_cell.to_index()] = 0;
            match self.selected_cell.side{
                Side::Top => self.game_board[13] += captured_stones,
                Side::Bottom => self.game_board[6] += captured_stones,
            }
            self.selected_cell.flip_sides();
        }else{
            if end_cell.pos != 6 {
                self.selected_cell.flip_sides();
            }
        }
    }

    fn has_won(&self) -> bool{
        let bottom_done = self.game_board[0..=5].into_iter().all(|x| {*x == 0});
        let top_done = self.game_board[7..=12].into_iter().all(|x| {*x == 0});
        bottom_done || top_done
    }

    fn winner(&self) -> Side{
        let mut bottom_total: usize = 0;
        let mut top_total: usize = 0;
        for p in 0..6{
            bottom_total += self.game_board[Cell{side: Side::Bottom, pos: p}.to_index()];
            top_total += self.game_board[Cell{side: Side::Top, pos: p}.to_index()];
        }
        if bottom_total > top_total {
            Side::Bottom
        }else{
            Side::Top
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

    fn set_selected(&mut self, cell: Cell) {
        self.selected_cell = cell;
    }
}

pub struct Avalanche{
    pub game_board: [usize; BOARD_LEN],
    pub selected_cell: Cell,
}

impl GameState for Avalanche {
    fn play(self: &mut Self) -> () {
        let chosen_cell: Cell = self.get_selected_cell();
        let chosen_cell_index = chosen_cell.to_index();
        let chosen_cell_value: usize = self.game_board[chosen_cell_index];
        if chosen_cell_value == 0 {
            return
        }
        self.game_board[chosen_cell_index] = 0;
        
        // update the board values, skip giving stone to the enemy
        let mut in_play: bool = true;
        let mut current_position = chosen_cell_index;
        let mut value = chosen_cell_value;
        while in_play {
            while value != 0 {
                current_position = (current_position + 1) % BOARD_LEN;
                if chosen_cell.side == Side::Bottom && current_position == 13 {
                    continue;
                }
                if chosen_cell.side == Side::Top && current_position == 6 {
                    continue;
                }
                self.game_board[current_position] += 1;
                value -= 1;
            }
            let end_cell: Cell = Cell::index_to_cell(current_position);
            let end_cell_value: usize = self.game_board[current_position];
            //dbg!(end_cell);
            //dbg!(end_cell_value);
            //dbg!(self.game_board);
            if end_cell.pos == 6{
                return
            }
            if end_cell_value != 1{
                value = end_cell_value;
                self.game_board[end_cell.to_index()] = 0;
            } else {
                in_play = false;
            }
        }
        self.selected_cell.flip_sides();

    }
    
    fn has_won(&self) -> bool{
        let bottom_done = self.game_board[0..=5].into_iter().all(|x| {*x == 0});
        let top_done = self.game_board[7..=12].into_iter().all(|x| {*x == 0});
        bottom_done || top_done
    }

    fn winner(&self) -> Side{
        if self.game_board[6] == self.game_board[13] {
            return self.selected_cell.side
        }
        if self.game_board[6] > self.game_board[13] {
            Side::Bottom
        } else {
            Side::Top
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

    fn set_selected(&mut self, cell: Cell) {
        self.selected_cell = cell;
    }
}