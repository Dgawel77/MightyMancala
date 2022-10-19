mod game;

#[derive(Debug)]
enum BoardState {
    Config,
    Playing,
    Finish,
}

pub struct MancalaBoard {
    ans: game::mancala,
    sudoku: Sudoku,
    focus: [usize; 2],
    history: Vec<([usize; 2], u8)>,
    redo: Vec<([usize; 2], u8)>,
    undos: usize,
    moves: usize,
    hints: usize,
    conflict: Option<[usize; 2]>,
    state: BoardState,
    stopwatch: Stopwatch,
}

// hold the board info 
impl MancalaBoard {
    pub fn new() -> Self{

    }
}

// holds how to draw the board on the screen
impl View for SudokuBoard {
    fn draw(&self, printer: &Printer) {
    }
}