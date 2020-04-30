pub mod game {
    use rand::seq::SliceRandom;
    use std::io::{self, Write};
    use std::{thread, time};

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Player {
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

	// Switch Player variant in place
        fn switch_player(&mut self) {
            if *self == Player::X {
                *self = Player::O;
            } else {
                *self = Player::X
            }
        }
    }

    pub struct Board {
        current_player: Player,
        cells: [[Option<Player>; 3]; 3],
        remaining_plays: Vec<usize>,
    }

    impl Board {
        pub fn new() -> Board {
            Board {
                cells: [[None, None, None], [None, None, None], [None, None, None]],
                current_player: Player::X,
                remaining_plays: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            }
        }

        pub fn generate_ai_play(&self) -> Option<&usize> {
            thread::sleep(time::Duration::from_millis(500));
            self.remaining_plays.choose(&mut rand::thread_rng())
        }

        pub fn get_play(&self) -> Result<usize, std::num::ParseIntError> {
            print!("\nEnter a number: ");
            io::stdout().flush().unwrap();
            let mut play = String::new();
            io::stdin()
                .read_line(&mut play)
                .expect("Failed to get input!");

            let play = play.trim().parse::<usize>();
            play
        }

        pub fn current_player(&self) -> char {
            self.current_player.to_char()
        }

        fn cell_index(&self, index: usize) -> Option<(usize, usize)> {
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

        pub fn update(&mut self, play: usize) {
            if play < 1 || play > 9 {
                println!("\nInvalid move, try again.");
                return;
            }

            if !self.remaining_plays.contains(&play) {
                println!("\nMove already taken, try again!");
                return;
            }

            if let Some((row, col)) = self.cell_index(play) {
                self.cells[row][col] = Some(self.current_player);
                let mut index: usize = 0;
                for (i, v) in self.remaining_plays.iter().enumerate() {
                    if v == &play {
                        index = i;
                        break;
                    }
                }
                self.remaining_plays.remove(index);
                self.current_player.switch_player();
            }
        }

        fn is_stalemate(&self) -> bool {
            if self.remaining_plays.is_empty() {
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

            macro_rules! has {
                ($player:expr, $x:expr, $y:expr) => {
                    self.cells[$x][$y] == Some(*$player)
                };
            }

            for player in &[Player::X, Player::O] {
                // Horizontal wins
                for row in 0..=2 {
                    if has!(player, row, 0) && has!(player, row, 1) && has!(player, row, 2) {
                        winner = Some(*player);
                    }
                }
                // Vertical wins
                for col in 0..=2 {
                    if has!(player, 0, col) && has!(player, 1, col) && has!(player, 2, col) {
                        winner = Some(*player);
                    }
                }
                // Diagonal wins
                if has!(player, 0, 0) && has!(player, 1, 1) && has!(player, 2, 2) {
                    winner = Some(*player);
                }
                if has!(player, 2, 0) && has!(player, 1, 1) && has!(player, 0, 2) {
                    winner = Some(*player);
                }
            }

            if winner != None {
                println!();
                self.draw();
                println!(
                    "\nGame over! {:?} wins!\n",
                    self.current_player.other_player()
                );
                true
            } else {
                false
            }
        }

        pub fn is_game_over(&self) -> bool {
            if self.is_stalemate() {
                true
            } else if self.has_winner() {
                true
            } else {
                false
            }
        }
    }
}
