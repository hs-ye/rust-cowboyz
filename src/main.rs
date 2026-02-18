mod assets;
mod simulation;
mod setup;
mod player;
mod game_state;

pub mod ui;

use clap::{Parser, CommandFactory};
use ui::cli::{Cli, Commands};
use std::io::{self, Write};

fn main() {
    // Check if the program was called with --help or --version directly
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == "--help" || args[1] == "-h" {
            let mut cli = Cli::command();
            cli.print_help().unwrap();
            return;
        } else if args[1] == "--version" || args[1] == "-V" {
            println!("rust-cowboyz {}", env!("CARGO_PKG_VERSION"));
            return;
        }
    }

    let mut world = setup::initialize_world(
        "data/config/goods.yaml",
        "data/config/planets.yaml",
    );

    // Calculate initial planet positions
    world.initialize_positions();

    println!("Welcome to Rust Cowboyz! Type your commands below.");

    loop {
        // Check if game has ended
        if world.game_clock.current_turn >= world.game_clock.total_turns {
            println!("Game Over! You have reached the end of the game.");
            println!("Final Score: ${} after {} turns", world.player.money, world.game_clock.current_turn);
            break;
        }

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
                    Ok(message) => {
                        println!("{}", message);
                        // Automatically print status after travel
                        println!("{}", ui::cli::display_game_status(&world));
                        println!("{}", ui::cli::display_player_status(&world));
                        println!("{}", ui::cli::display_market_status(&world));
                        println!("{}", ui::cli::display_travel_options(&world));
                    },
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Commands::Wait { months } => {
                match player::actions::handle_wait(&mut world, *months) {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Commands::PlanetInfo { planet_id } => {
                println!("{}", ui::cli::display_planet_info(&world, planet_id));
            }
            Commands::Quit => {
                println!("Exiting game. Goodbye!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mvp_gameplay_flow() {
        // Initialize the game world
        let mut world = setup::initialize_world(
            "data/config/goods.yaml",
            "data/config/planets.yaml",
        );
        
        // Verify initial state matches MVP requirements
        assert_eq!(world.player.money, 1000);
        assert_eq!(world.player.location, "earth");
        assert_eq!(world.player.inventory.capacity, 10);
        assert_eq!(world.game_clock.current_turn, 1);
        assert_eq!(world.game_clock.total_turns, 10);
        
        // Test buying goods
        let initial_money = world.player.money;
        let result = player::actions::handle_buy(&mut world, "Ice", 2);
        assert!(result.is_ok());
        // Player should have spent money
        assert!(world.player.money < initial_money);
        // Player should have 2 units of Ice in cargo
        assert_eq!(world.player.inventory.goods.get("Ice").unwrap(), &2);
        
        // Test selling goods
        let money_before_sell = world.player.money;
        let result = player::actions::handle_sell(&mut world, "Ice", 1);
        assert!(result.is_ok());
        // Player should have more money after selling
        assert!(world.player.money > money_before_sell);
        // Player should have 1 unit of Ice left in cargo
        assert_eq!(world.player.inventory.goods.get("Ice").unwrap(), &1);
        
        // Test travel
        let initial_location = world.player.location.clone();
        let result = player::actions::handle_travel(&mut world, "mars");
        assert!(result.is_ok());
        // Player should now be on Mars
        assert_eq!(world.player.location, "mars");
        
        // Test wait command
        let turn_before_wait = world.game_clock.current_turn;
        let result = player::actions::handle_wait(&mut world, 5);
        assert!(result.is_ok());
        // Game turn should have advanced
        assert_eq!(world.game_clock.current_turn, turn_before_wait + 5);
    }
}
