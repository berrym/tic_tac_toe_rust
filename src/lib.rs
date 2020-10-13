pub mod game {
    use std::fmt;
    use std::io::{self, Write};

    pub type Play = (usize, usize);

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Player {
        X,
        O,
    }

    pub type Board = [[Option<Player>; 3]; 3];

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct MiniMaxResult {
        pub play: Option<Play>,
        pub score: Option<i32>,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct TicTacToe {
        pub board: Board,
        pub current_player: Player,
        pub empty_plays: Vec<Play>,
    }

    // Implement the Player struct
    impl Player {
        // Represent a Player as a char
        fn to_char(self) -> char {
            match self {
                Player::X => 'X',
                Player::O => 'O',
            }
        }

        // Return the opposite player
        fn other_player(self) -> Player {
            match self {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }

        // Switch current player
        fn switch_player(&mut self) {
            *self = self.other_player()
        }

        // Get a play from human user
        pub fn get_play(&self) -> Option<Play> {
            print!("\nEnter a number: ");
            io::stdout().flush().unwrap();
            let mut play = String::new();
            io::stdin()
                .read_line(&mut play)
                .expect("Failed to get input!");
            let play = play.trim().parse::<usize>();
            translate_to_coord(play.unwrap())
        }

        // Evaluate score of board state
        fn evaluate(&self, state: &TicTacToe) -> Option<i32> {
            let result = state.has_winner();
            match result {
                Some(Player::X) => Some(1),
                Some(Player::O) => Some(-1),
                _ => Some(0),
            }
        }

        // Apply recursive minimax algorithm to determine AI move
        pub fn minimax(
            &self,
            state: &mut TicTacToe,
            depth: usize,
            player: Player,
        ) -> Option<MiniMaxResult> {
            let mut best: MiniMaxResult;

            // Initialize best score for either Player
            if player == Player::X {
                best = MiniMaxResult {
                    play: None,
                    score: Some(std::i32::MIN),
                };
            } else {
                best = MiniMaxResult {
                    play: None,
                    score: Some(std::i32::MAX),
                };
            }

            // Initialize the simulation results
            let mut sim_score: MiniMaxResult = MiniMaxResult {
                play: None,
                score: None,
            };

            // If depth is zero stop and return simulated score
            if depth == 0 {
                sim_score.score = self.evaluate(state);
                return Some(sim_score);
            }

            // If state has a winner stop and return score
            if let Some(_winner) = state.has_winner() {
                sim_score.score = self.evaluate(state);
                return Some(sim_score);
            }

            for cell in empty_plays(state.board) {
                // Create a copy of the board for restoration after simulation
                let state_copy = state.board.clone();

                // Simulate possible moves
                let x = cell.0;
                let y = cell.1;
                state.board[x][y] = Some(player);
                sim_score = self
                    .minimax(state, depth - 1, player.other_player())
                    .unwrap();

                // Undo simulation
                state.board = state_copy;

                // Determine best score for player
                sim_score.play = Some((x, y));

                if player == Player::X {
                    if sim_score.score > best.score {
                        best = sim_score; // max value
                    }
                } else {
                    if sim_score.score < best.score {
                        best = sim_score; // min value
                    }
                }
            }
            Some(best)
        }
    }

    // Create a vector of available plays
    pub fn empty_plays(board: Board) -> Vec<Play> {
        let mut empty_plays: Vec<Play> = vec![];
        for (x, row) in board.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                match cell {
                    Some(_cell) => (),
                    _ => empty_plays.push((x, y)),
                }
            }
        }
        empty_plays
    }

    // Translate user visible play numbers to valid coordinates
    pub fn translate_to_coord(index: usize) -> Option<Play> {
        match index {
            1 => Some((0, 0)),
            2 => Some((0, 1)),
            3 => Some((0, 2)),
            4 => Some((1, 0)),
            5 => Some((1, 1)),
            6 => Some((1, 2)),
            7 => Some((2, 0)),
            8 => Some((2, 1)),
            9 => Some((2, 2)),
            _ => None,
        }
    }

    // Implement display formatting for TicTacToe struct
    impl fmt::Display for TicTacToe {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(
                f,
                " {} | {} | {} ",
                self.repr_cell(self.ref_cell(1), '1'),
                self.repr_cell(self.ref_cell(2), '2'),
                self.repr_cell(self.ref_cell(3), '3')
            )?;
            writeln!(f, "---|---|---")?;
            writeln!(
                f,
                " {} | {} | {} ",
                self.repr_cell(self.ref_cell(4), '4'),
                self.repr_cell(self.ref_cell(5), '5'),
                self.repr_cell(self.ref_cell(6), '6')
            )?;
            writeln!(f, "---|---|---")?;
            writeln!(
                f,
                " {} | {} | {} ",
                self.repr_cell(self.ref_cell(7), '7'),
                self.repr_cell(self.ref_cell(8), '8'),
                self.repr_cell(self.ref_cell(9), '9')
            )?;
            Ok(())
        }
    }

    // Instantiate an implementation of TicTacToe
    impl TicTacToe {
        pub fn new() -> TicTacToe {
            TicTacToe {
                current_player: Player::X,
                board: [[None; 3]; 3],
                empty_plays: vec![
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (1, 0),
                    (1, 1),
                    (1, 2),
                    (2, 0),
                    (2, 1),
                    (2, 2),
                ],
            }
        }

        // Return current player
        pub fn current_player(&self) -> Player {
            self.current_player
        }

        // Reference avaialble plays (cells) by index 1-9
        fn ref_cell(&self, index: usize) -> Option<Player> {
            match index {
                1 => self.board[0][0],
                2 => self.board[0][1],
                3 => self.board[0][2],
                4 => self.board[1][0],
                5 => self.board[1][1],
                6 => self.board[1][2],
                7 => self.board[2][0],
                8 => self.board[2][1],
                9 => self.board[2][2],
                _ => None,
            }
        }

        // Reverse functionality of ref cell, take a coordinate and return Some index
        pub fn cell_index(&self, play: Play) -> Option<usize> {
            match play {
                (0, 0) => Some(0),
                (0, 1) => Some(1),
                (0, 2) => Some(2),
                (1, 0) => Some(3),
                (1, 1) => Some(4),
                (1, 2) => Some(5),
                (2, 0) => Some(6),
                (2, 1) => Some(7),
                (2, 2) => Some(8),
                _ => None,
            }
        }

        // Represemt a play (cell) as a character
        fn repr_cell(&self, cell: Option<Player>, none_char: char) -> char {
            match cell {
                Some(p) => p.to_char(),
                None => none_char,
            }
        }

        // Apply a play to the board
        pub fn apply_play(&mut self, play: Option<Play>) -> bool {
            if let Some(p) = play {
                self.board[p.0][p.1] = Some(self.current_player);
                self.empty_plays = empty_plays(self.board);
                self.current_player.switch_player();
                true
            } else {
                false
            }
        }

        // Check for a winner
        fn has_winner(&self) -> Option<Player> {
            let mut winner: Option<Player> = None;

            for i in 0..3 {
                // Columns
                match (self.board[i][0], self.board[i][1], self.board[i][2]) {
                    (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                    _ => (),
                }
                // Rows
                match (self.board[0][i], self.board[1][i], self.board[2][i]) {
                    (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                    _ => (),
                }
            }
            // Diagonals
            match (self.board[0][0], self.board[1][1], self.board[2][2]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                _ => (),
            }
            match (self.board[2][0], self.board[1][1], self.board[0][2]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                _ => (),
            }
            winner
        }

        // Helper function for has_winner and game_over
        fn is_win(&self) -> bool {
            let winner = self.has_winner();
            if winner != None {
                println!();
                println!("{}", self);
                println!("\nGame over! {:?} wins!\n", winner.unwrap());
                true
            } else {
                false
            }
        }

        // Helper function for game_over
        fn is_stalemate(&self) -> bool {
            if empty_plays(self.board).is_empty() {
                println!();
                println!("{}", self);
                println!("\nGame over, stalemate.\n");
                true
            } else {
                false
            }
        }

        // Check if the game is over
        pub fn game_over(&self) -> bool {
            if self.is_win() {
                true
            } else if self.is_stalemate() {
                true
            } else {
                false
            }
        }
    }
}
