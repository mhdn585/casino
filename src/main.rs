mod roulette;
mod bet;
mod player;
mod ui;

use roulette::RouletteWheel;
use bet::{Bet, BetType};
use player::Player;

fn main() {
    let mut player = Player::load_or_create();
    let roulette = RouletteWheel::new();
    
    loop {
        ui::clear_screen();
        ui::show_header(&player);
        
        let choice = ui::show_main_menu();
        
        if choice == 0 {
            ui::show_message(&format!("Gracias por jugar. Saldo final: ${:.2}", player.get_balance()), "amarillo");
            player.save();
            break;
        }
        
        let bet_type = match choice {
            1 => BetType::Straight,
            2 => BetType::Color,
            3 => BetType::EvenOdd,
            4 => BetType::LowHigh,
            5 => BetType::Dozen,
            6 => BetType::Column,
            7 => BetType::Street,
            8 => BetType::Corner,
            9 => BetType::SixLine,
            _ => {
                ui::show_message("Opcion invalida", "morado");
                continue;
            }
        };
        
        let placement = match ui::get_bet_details(&bet_type, &roulette) {
            Some(p) => p,
            None => {
                ui::show_message("Apuesta cancelada", "morado");
                ui::wait_for_enter();
                continue;
            }
        };
        
        let amount = ui::get_bet_amount(&player);
        if amount <= 0.0 {
            ui::show_message("Monto invalido", "morado");
            ui::wait_for_enter();
            continue;
        }
        
        if !player.can_afford(amount) {
            ui::show_message("Saldo insuficiente", "morado");
            ui::wait_for_enter();
            continue;
        }
        
        let bet = Bet::new(amount, bet_type, placement);
        
        player.place_bet(amount);
        
        ui::show_spinning();
        
        let result = roulette.spin();
        
        let win_amount = bet.calculate_win(&result, &roulette);
        
        if win_amount > 0.0 {
            player.add_winnings(win_amount);
            ui::show_win(&result, win_amount);
        } else {
            ui::show_loss(&result);
        }
        
        ui::show_message(&format!("Saldo actual: ${:.2}", player.get_balance()), "blanco");
        
        player.save();
        
        ui::wait_for_enter();
    }
}