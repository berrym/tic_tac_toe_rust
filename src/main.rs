use tic_tac_toe::game::{MiniMaxResult, Play, Player, TicTacToe};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = TicTacToe::new();

    game.current_player = Player::X;

    println!("Tic-Tac-Toe");
    println!("\n{:?}'s turn\n", game.current_player());
    println!("{}", game);

    // Main game loop
    loop {
        let moves: usize = game.empty_plays.len();
        let player: Player = game.current_player();
        let mut play: Option<Play> = None;
        let mini_max_result: Option<MiniMaxResult>;

        // Generate an AI play using the minimax or get a human play
        if game.current_player() == Player::O {
            mini_max_result = player.minimax(&mut game, moves, player);
            match mini_max_result {
                Some(p) => match p.play {
                    Some(pl) => play = Some(pl),
                    _ => (),
                },
                _ => continue,
            }
        } else {
            play = player.get_play();
        }

        // Try to apply the play to the game board
        if game.apply_play(play) {
            if game.game_over() {
                break;
            }
        } else {
            println!("Bad move...{:?}", play);
            continue;
        }
        println!("\n{:?}'s turn\n", game.current_player());
        println!("{}", game);
    }
    game.game_over();
    Ok(())
}
