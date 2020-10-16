use clap::{App, Arg};
use tic_tac_toe::game::{Config, empty_plays, MiniMaxResult, Play, Player, TicTacToe};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line
    let cli = App::new("Tic-Tac-Toe")
        .version("0.1.1")
        .author("Michael Berry <trismegustis@gmail.com>")
        .about("Play a simple version of the classic Tic-Tac-Toe game.")
        .arg(
            Arg::with_name("Human vs Computer")
                .short("1")
                .long("human-vs-computer")
                .help("Human vs Computer")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("Human vs Human")
                .short("2")
                .long("human-vs-human")
                .help("Human vs Human")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("Computer vs Human")
                .short("3")
                .long("computer-vs-human")
                .help("Computer vs Human")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("Computer vs Computer")
                .short("4")
                .long("computer-vs-computer")
                .help("Computer vs Computer")
                .takes_value(false),
        )
        .get_matches();

    // Call game loop with player configuration
    if cli.is_present("Human vs Computer") {
        game_loop(Config {
            player_x_ai: false,
            player_o_ai: true,
        })?;
    } else if cli.is_present("Human vs Human") {
        game_loop(Config {
            player_x_ai: false,
            player_o_ai: false,
        })?;
    } else if cli.is_present("Computer vs Human") {
        game_loop(Config {
            player_x_ai: true,
            player_o_ai: false,
        })?;
    } else if cli.is_present("Computer vs Computer") {
        game_loop(Config {
            player_x_ai: true,
            player_o_ai: true,
        })?;
    } else {
        cli.usage();
    }
    Ok(())
}

// Main game loop
fn game_loop(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut game = TicTacToe::new();

    println!("Tic-Tac-Toe");
    println!("\n{:?}'s turn\n", game.current_player());
    println!("{}", game);

    // Main game loop
    loop {
        let moves: usize = empty_plays(game.board()).len();
        let player: Player = game.current_player();
        let mut play: Option<Play> = None;
        let mini_max_result: Option<MiniMaxResult>;

        if game.current_player() == Player::X {
            if config.player_x_ai {
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
        } else {
            if config.player_o_ai {
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
