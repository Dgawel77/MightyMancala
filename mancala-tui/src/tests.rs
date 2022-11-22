#[cfg(test)]
mod capture_tests{
    use crate::mancala::{Side, GameState, Capture, Cell};
    
    #[test]
    fn capture_intialization() {
        let game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,4,0,4,4,4,4,4,4,0],
            selected_cell: Cell{side: Side::Bottom, pos: 0},
        });
        assert_eq!(game_state.game_board, [4,4,4,4,4,4,0,4,4,4,4,4,4,0]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Bottom, pos:0});
    }

    #[test]
    fn capture_play_bottom() {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,4,0,4,4,4,4,4,4,0],
            selected_cell: Cell{side: Side::Bottom, pos: 0},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [0,5,5,5,5,4,0,4,4,4,4,4,4,0]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Top, pos:0});
    }

    #[test]
    fn capture_play_top() {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,4,0,4,4,4,4,4,4,0],
            selected_cell: Cell{side: Side::Top, pos: 0},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [5,5,5,4,4,4,0,4,4,4,4,4,0,1]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Bottom, pos:0});
    }

    #[test]
    fn capture_keep_position_bottom() {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,4,0,4,4,4,4,4,4,0],
            selected_cell: Cell{side: Side::Bottom, pos: 2},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,0,5,5,5,1,4,4,4,4,4,4,0]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Bottom, pos:2});
    }

    #[test]
    fn capture_keep_position_top() {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,4,0,4,4,4,4,4,4,0],
            selected_cell: Cell{side: Side::Top, pos: 3},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,4,4,4,4,0,4,4,0,5,5,5,1]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Top, pos:3});
    }


    #[test]
    fn capture_capture_more_than_one_bottom () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,1,0,4,0,4,4,4,4,4,4,0],
            selected_cell: Cell{side: Side::Bottom, pos: 3},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,4,0,0,4,5,4,0,4,4,4,4,0]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Bottom, pos:3});
    }

    #[test]
    fn capture_capture_more_than_one_top () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,4,0,4,1,0,4,4,4,0],
            selected_cell: Cell{side: Side::Top, pos: 4},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,4,0,4,4,0,4,0,0,4,4,4,5]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Top, pos:4});
    }

    #[test]
    fn capture_capture_with_one_bottom () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,1,0,4,0,4,0,4,4,4,4,0],
            selected_cell: Cell{side: Side::Bottom, pos: 3},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,4,0,0,4,1,4,0,4,4,4,4,0]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Bottom, pos:3});
    }

    #[test]
    fn capture_capture_with_one_top () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,0,4,4,0,4,1,0,4,4,4,0],
            selected_cell: Cell{side: Side::Top, pos: 4},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,4,0,4,4,0,4,0,0,4,4,4,1]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Top, pos:4});
    }

    #[test]
    fn capture_capture_skip_opponent_bottom () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,10,0,4,4,4,4,4,4,0],
            selected_cell: Cell{side: Side::Bottom, pos: 5},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [5,5,5,4,4,0,1,5,5,5,5,5,5,0]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Top, pos:5});
    }

    #[test]
    fn capture_capture_skip_opponent_top () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,4,0,4,4,4,4,4,10,0],
            selected_cell: Cell{side: Side::Top, pos: 0},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [5,5,5,5,5,5,0,5,5,5,4,4,0,1]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Bottom, pos:0});
    }

    #[test]
    fn capture_capture_clause_in_bowl_bottom () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,1,0,4,4,4,4,4,1,4],
            selected_cell: Cell{side: Side::Bottom, pos: 5},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,4,4,4,0,1,4,4,4,4,4,1,4]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Bottom, pos:5});
    }

    #[test]
    fn capture_capture_clause_in_bowl_top () {
        let mut game_state = Box::new(Capture{
            game_board: [4,4,4,4,4,1,6,4,4,4,4,4,1,0],
            selected_cell: Cell{side: Side::Top, pos: 0},
        });
        game_state.play();
        assert_eq!(game_state.game_board, [4,4,4,4,4,1,6,4,4,4,4,4,0,1]);
        assert_eq!(game_state.selected_cell, Cell{side: Side::Top, pos:0});
    }

}