mod assets;
mod simulation;
mod setup;
mod player;
mod game_state;

pub mod ui;

use clap::Parser;
use ui::cli::{Cli, Commands};
use std::io::{self, Write};

fn main() {
    let mut world = setup::initialize_world(
        "data/config/goods.yaml",
        "data/config/planets.yaml",
    );
    
    // Calculate initial planet positions
    world.initialize_positions();
    
    println!("Welcome to Rust Cowboyz! Type your commands below.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut args = vec!["rust-cowboyz".to_string()]; // Dummy program name
        args.extend(shlex::split(input).unwrap_or_else(|| {
            eprintln!("Error: Invalid input");
            return Vec::new();
        }));

        if args.len() == 1 {
            continue;
        }

        let cli = match Cli::try_parse_from(args.iter()) {
            Ok(cli) => cli,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        match &cli.command {
            Commands::Status {} => {
                println!("{}", ui::cli::display_game_status(&world));
                println!("{}", ui::cli::display_player_status(&world));
                println!("{}", ui::cli::display_market_status(&world));
                println!("{}", ui::cli::display_travel_options(&world));
            }
            Commands::Buy { good_id, quantity } => {
                match player::actions::handle_buy(&mut world, good_id, *quantity) {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Commands::Sell { good_id, quantity } => {
                match player::actions::handle_sell(&mut world, good_id, *quantity) {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Commands::Travel { destination_planet_id } => {
                match player::actions::handle_travel(&mut world, destination_planet_id) {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Commands::Wait { months } => {
                match player::actions::handle_wait(&mut world, *months) {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Commands::Quit => {
                println!("Exiting game. Goodbye!");
                break;
            }
        }
    }
}
