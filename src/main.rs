use tic_tac_toe::game;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = game::Board::new();

    println!("Tic-Tac-Toe");

    loop {
        let current_player = board.current_player();

        println!("\n{}'s turn\n", current_player);
        board.draw();

        if current_player == 'O' {
            let play = board.generate_ai_play();
            let p = match play {
                Some(p) => *p,
                None => {
                    println!("\nError generating AI move.");
                    continue;
                }
            };
            board.update(p);
        } else {
            let play = board.get_play();
            match play {
                Ok(p) => board.update(p),
                Err(_) => {
                    println!("\nEnter an number from the board.");
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
