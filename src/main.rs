use clap::{App, Arg};
use std::io::{self, Write};
use tic_tac_toe::game::{empty_plays, Config, MiniMaxResult, Play, Player, TicTacToe};

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
        main_menu()?;
    }
    Ok(())
}

fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Tic-Tac-Toe\n");
        println!("1) Human vs Computer");
        println!("2) Human vs Human");
        println!("3) Computer vs Human");
        println!("4) Computer vs Computer");

        print!("\nEnter a number: ");
        io::stdout().flush().unwrap();
        let mut game_type = String::new();
        io::stdin()
            .read_line(&mut game_type)
            .expect("Failed to get input!");
        let game_type = game_type.trim().parse::<usize>();

        let game_config: Config;
        match game_type {
            Ok(1) => {
                game_config = Config {
                    player_x_ai: false,
                    player_o_ai: true,
                }
            }
            Ok(2) => {
                game_config = Config {
                    player_x_ai: false,
                    player_o_ai: false,
                }
            }
            Ok(3) => {
                game_config = Config {
                    player_x_ai: true,
                    player_o_ai: false,
                }
            }
            Ok(4) => {
                game_config = Config {
                    player_x_ai: true,
                    player_o_ai: true,
                }
            }
            _ => continue,
        }
        game_loop(game_config)?
    }
}

// Main game loop
fn game_loop(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut game = TicTacToe::new();

    println!("\n{:?}'s turn\n", game.current_player());
    println!("{}", game);

    // Main game loop
    loop {
        let moves: usize = empty_plays(game.board()).len();
        let player: Player = game.current_player();
        let mut play: Option<Play> = None;
        let mini_max_result: Option<MiniMaxResult>;

        if player == Player::X {
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
                play = player.get_play(game.board());
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
                play = player.get_play(game.board());
            }
        }

        // Try to apply the play to the game board
        if game.apply_play(play) {
            if game.game_over() {
                std::process::exit(0);
            }
        } else {
            println!("Bad move...{:?}", play);
            continue;
        }
        println!("\n{:?}'s turn\n", game.current_player());
        println!("{}", game);
    }
}
