mod assets;
mod simulation;
mod setup;
mod player;
mod game_state;

pub mod ui;

use clap::Parser;
use ui::cli::{Cli, Commands};

fn main() {
    let mut world = setup::initialize_world(
        "data/config/goods.yaml",
        "data/config/planets.yaml",
    );
    
    // Calculate initial planet positions
    world.initialize_positions();
    
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status {} => {
            ui::cli::display_game_status(&world);
            ui::cli::display_player_status(&world);
            ui::cli::display_market_status(&world);
            ui::cli::display_travel_options(&world);
        }
        Commands::Buy { good_id, quantity } => {
            println!("Buy command: {} x {} (Not yet implemented)", good_id, quantity);
        }
        Commands::Sell { good_id, quantity } => {
            println!("Sell command: {} x {} (Not yet implemented)", good_id, quantity);
        }
        Commands::Travel { destination_planet_id } => {
            println!("Travel command: {} (Not yet implemented)", destination_planet_id);
        }
        Commands::Wait { months } => {
            println!("Wait command: {} months (Not yet implemented)", months);
        }
    }
}
