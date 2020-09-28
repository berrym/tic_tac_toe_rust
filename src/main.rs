use tic_tac_toe::game::{Move, Player, TicTacToe};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = TicTacToe::new();

    println!("Tic-Tac-Toe");

    loop {
        let _move: Option<Move>;
        let player = board.current_player();
        println!("\n{:?}'s turn\n", board.current_player());
        println!("{}", board);

        if board.current_player() == Player::O {
            _move = Some(player.generate_ai_move(&mut board)).unwrap();

        } else {
            _move = Some(player.get_move()).unwrap();
        }

        match _move {
            Some(_move)  => (),
            _  => continue,
        }

        board.apply_move(_move.unwrap());

        if board.is_game_over() {
            break;
        }
    }

    Ok(())
}
