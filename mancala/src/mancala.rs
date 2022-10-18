#[derive(Debug)]
enum GameMode {
    Capture,
    Avalanche
}

#[derive(Debug)]
enum Difficulty {
    Normal,
    Random
}

#[derive(Debug)]
pub struct Mancala{
    game: [u8; 14],
    mode: GameMode,
    difficulty: Difficulty
}

// holds game data
impl mancala{

}