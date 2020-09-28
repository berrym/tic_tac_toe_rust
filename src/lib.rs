pub mod game {
    use rand::seq::SliceRandom;
    use std::io::{self, Write};
    use std::{thread, time};

    type Move = (usize, usize);

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Player {
        X,
        O,
    }

    impl Player {
        fn to_char(self) -> char {
            match self {
                Player::X => 'X',
                Player::O => 'O',
            }
        }

        fn other_player(self) -> Player {
            match self {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }

        fn switch_player(&mut self) {
            *self = self.other_player()
        }
    }

    pub fn empty_cells(cells: [[Option<Player>; 3]; 3]) -> Vec<Move> {
        let mut empty_cells: Vec<Move> = vec![];
        for (x, row) in cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                match cell {
                    Some(_cell) => {},
                    _ => empty_cells.push((x, y))
                }
            }
        }
        empty_cells
    }

    pub fn translate_to_coord(index: usize) -> Option<Move> {
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

    pub fn get_move() -> Result<usize, std::num::ParseIntError> {
        print!("\nEnter a number: ");
        io::stdout().flush().unwrap();
        let mut _move = String::new();
        io::stdin()
            .read_line(&mut _move)
            .expect("Failed to get input!");
        let _move = _move.trim().parse::<usize>();
        _move
    }
    
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct TicTacToe {
        current_player: Player,
        cells: [[Option<Player>; 3]; 3],
        empty_moves: Vec::<usize>
    }

    impl TicTacToe {
        pub fn new() -> TicTacToe {
            TicTacToe {
                cells: [[None, None, None], [None, None, None], [None, None, None]],
                current_player: Player::X,
                empty_moves: vec![],
            }
        }

        pub fn generate_ai_play(&mut self) -> Option<&usize> {
            thread::sleep(time::Duration::from_millis(500));
            self.empty_moves = (1..=empty_cells(self.cells).len()).map(usize::from).collect();
            self.empty_moves.choose(&mut rand::thread_rng())
        }

        pub fn current_player(&self) -> char {
            self.current_player.to_char()
        }

        fn ref_cell(&self, index: usize) -> Option<Player> {
            match index {
                1 => self.cells[0][0],
                2 => self.cells[0][1],
                3 => self.cells[0][2],
                4 => self.cells[1][0],
                5 => self.cells[1][1],
                6 => self.cells[1][2],
                7 => self.cells[2][0],
                8 => self.cells[2][1],
                9 => self.cells[2][2],
                _ => None,
            }
        }

        fn repr_cell(&self, cell: Option<Player>, none_char: char) -> char {
            match cell {
                Some(p) => p.to_char(),
                None => none_char,
            }
        }

        pub fn draw(&self) {
            println!(
                " {} | {} | {} ",
                self.repr_cell(self.ref_cell(1), '1'),
                self.repr_cell(self.ref_cell(2), '2'),
                self.repr_cell(self.ref_cell(3), '3')
            );
            println!("---|---|---");
            println!(
                " {} | {} | {} ",
                self.repr_cell(self.ref_cell(4), '4'),
                self.repr_cell(self.ref_cell(5), '5'),
                self.repr_cell(self.ref_cell(6), '6')
            );
            println!("---|---|---");
            println!(
                " {} | {} | {} ",
                self.repr_cell(self.ref_cell(7), '7'),
                self.repr_cell(self.ref_cell(8), '8'),
                self.repr_cell(self.ref_cell(9), '9')
            );
        }

        pub fn apply_move(&mut self, _move: Move) -> bool {
            if empty_cells(self.cells).contains(&_move) {
                self.cells[_move.0][_move.1] = Some(self.current_player);
                self.current_player.switch_player();
                true
            } else {
                false
            }
        }

        fn is_stalemate(&self) -> bool {
            if empty_cells(self.cells).is_empty() {
                println!();
                self.draw();
                println!("\nGame over, stalemate.\n");
                true
            } else {
                false
            }
        }

        fn has_winner(&self) -> bool {
            let mut winner: Option<Player> = None;

            for i in 0..3 {
                // Columns
                match (self.cells[i][0], self.cells[i][1], self.cells[i][2]) {
                    (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                    _ => {}
                }
                // Rows
                match (self.cells[0][i], self.cells[1][i], self.cells[2][i]) {
                    (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                    _ => {}
                }
            }
            // Diagonals
            match (self.cells[0][0], self.cells[1][1], self.cells[2][2]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                _ => {}
            }
            match (self.cells[2][0], self.cells[1][1], self.cells[0][2]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => winner = Some(x),
                _ => {}
            }

            if winner != None {
                println!();
                self.draw();
                println!("\nGame over! {} wins!\n", winner.unwrap().to_char());
                true
            } else {
                false
            }
        }

        pub fn is_game_over(&self) -> bool {
            if self.has_winner() {
                true
            } else if self.is_stalemate() {
                true
            } else {
                false
            }
        }
    }
}
