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
        
        if player.get_balance() <= 0.0 {
            ui::show_message("Te has quedado sin saldo!", "morado");
            ui::show_message("Presiona Enter para reiniciar tu saldo o selecciona 0 para salir.", "amarillo");
            ui::wait_for_enter();
            player.reset_balance();
            player.save();
            continue;
        }
        
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
        
        let payout = bet.calculate_win(&result);
        
        if payout > 0.0 {
            player.add_winnings(payout);
            ui::show_win(&result, payout);
        } else {
            ui::show_loss(&result);
        }
        
        ui::show_message(&format!("Saldo actual: ${:.2}", player.get_balance()), "blanco");
        
        player.save();
        
        ui::wait_for_enter();
    }
}