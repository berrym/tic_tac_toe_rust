use tic_tac_toe::game::{self, TicTacToe};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = TicTacToe::new();

    println!("Tic-Tac-Toe");

    loop {
        let current_player = board.current_player();

        println!("\n{}'s turn\n", current_player);
        board.draw();

        if current_player == 'O' {
            let play = board.generate_ai_play().unwrap();
            if let Some(_move) = game::translate_to_coord(*play) {
                if !board.apply_move(_move) {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            let play = game::get_move();
            match play {
                Ok(p) => {
                    let _move = game::translate_to_coord(p).unwrap();
                    board.apply_move(_move);
                }
                Err(_) => {
                    eprintln!("\nEnter an number from the board.");
                    continue;
                }
            }
        }

        if board.is_game_over() {
            break;
        }
    }

    Ok(())
}
