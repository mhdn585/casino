use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

use crate::roulette::{RouletteWheel, Color};
use crate::player::Player;
use crate::bet::{BetType, BetPlacement, EvenOddChoice, LowHighChoice};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

pub fn color_text(text: &str, color: &str) -> String {
    let code = match color {
        "rojo" => "\x1B[31m",
        "verde" => "\x1B[32m",
        "amarillo" => "\x1B[33m",
        "morado" => "\x1B[35m",
        "gris" => "\x1B[90m",
        "blanco" => "\x1B[37m",
        _ => "\x1B[37m",
    };
    format!("{}{}\x1B[0m", code, text)
}

pub fn show_message(message: &str, color: &str) {
    println!("{}", color_text(message, color));
}

pub fn show_header(player: &Player) {
    println!("{}", color_text("=", "blanco").repeat(80));
    println!(
        "{} {}",
        color_text("RULETA EUROPEA", "verde"),
        color_text(&format!("Saldo: ${:.2}", player.get_balance()), "amarillo")
    );
    println!("{}", color_text("=", "blanco").repeat(80));
    println!(
        "Victorias: {}  Derrotas: {}  Tasa: {:.1}%",
        player.get_bets_won(),
        player.get_bets_lost(),
        player.get_win_rate()
    );
    println!("{}", color_text("-", "blanco").repeat(80));
}

pub fn show_main_menu() -> u32 {
    println!("\n{}", color_text("TIPOS DE APUESTA:", "blanco"));
    println!(" 1. {} - Un solo numero (35:1)", color_text("PLENO", "verde"));
    println!(" 2. {} - Rojo/Negro (1:1)", color_text("COLOR", "rojo"));
    println!(" 3. {} - Par/Impar (1:1)", color_text("PAR/IMPAR", "blanco"));
    println!(" 4. {} - Falta(1-18)/Pasa(19-36) (1:1)", color_text("FALTA/PASA", "blanco"));
    println!(" 5. {} - 1-12/13-24/25-36 (2:1)", color_text("DOCENA", "blanco"));
    println!(" 6. {} - Columnas verticales (2:1)", color_text("COLUMNA", "blanco"));
    println!(" 7. {} - Fila de 3 numeros (11:1)", color_text("CALLE", "blanco"));
    println!(" 8. {} - Cuadro de 4 numeros (8:1)", color_text("CUADRO", "blanco"));
    println!(" 9. {} - 6 numeros consecutivos (5:1)", color_text("SEISENA", "blanco"));
    println!(" 0. {} - Salir", color_text("SALIR", "morado"));
    println!("{}", color_text("-", "blanco").repeat(80));
    
    print!("Seleccione una opcion: ");
    let _ = io::stdout().flush();
    
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    
    input.trim().parse().unwrap_or(99)
}

pub fn get_bet_amount(player: &Player) -> f64 {
    if player.get_balance() < 1.0 {
        return 0.0;
    }
    loop {
        print!("Monto a apostar (min 1, max {:.2}): $", player.get_balance());
        let _ = io::stdout().flush();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            show_message("Error de entrada", "morado");
            continue;
        }
        
        match input.trim().parse::<f64>() {
            Ok(amount) => {
                if amount >= 1.0 && amount <= player.get_balance() {
                    return amount;
                } else {
                    show_message(&format!("Monto invalido. Debe ser entre 1 y {:.2}", player.get_balance()), "morado");
                }
            }
            Err(_) => {
                show_message("Por favor ingrese un numero valido", "morado");
            }
        }
    }
}

pub fn get_bet_details(bet_type: &BetType, _roulette: &RouletteWheel) -> Option<BetPlacement> {
    match bet_type {
        BetType::Straight => get_straight_details(),
        BetType::Color => get_color_details(),
        BetType::EvenOdd => get_evenodd_details(),
        BetType::LowHigh => get_lowhigh_details(),
        BetType::Dozen => get_dozen_details(),
        BetType::Column => get_column_details(),
        BetType::Street => get_street_details(),
        BetType::Corner => get_corner_details(),
        BetType::SixLine => get_sixline_details(),
    }
}

fn get_straight_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA PLENO - Elija un numero del 0 al 36", "verde");
    print_flush("Numero: ");
    
    match read_u32() {
        Some(num) if num <= 36 => Some(BetPlacement::Number(num)),
        Some(_) => {
            show_message("Numero invalido", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_color_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA AL COLOR", "rojo");
    println!("1. Rojo");
    println!("2. Negro");
    print_flush("Seleccione (1/2): ");
    
    match read_u32() {
        Some(1) => Some(BetPlacement::Color(Color::Rojo)),
        Some(2) => Some(BetPlacement::Color(Color::Negro)),
        Some(_) => {
            show_message("Opcion invalida", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_evenodd_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA PAR/IMPAR", "blanco");
    println!("1. Par");
    println!("2. Impar");
    print_flush("Seleccione (1/2): ");
    
    match read_u32() {
        Some(1) => Some(BetPlacement::EvenOdd(EvenOddChoice::Even)),
        Some(2) => Some(BetPlacement::EvenOdd(EvenOddChoice::Odd)),
        Some(_) => {
            show_message("Opcion invalida", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_lowhigh_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA FALTA/PASA", "blanco");
    println!("1. Falta (1-18)");
    println!("2. Pasa (19-36)");
    print_flush("Seleccione (1/2): ");
    
    match read_u32() {
        Some(1) => Some(BetPlacement::LowHigh(LowHighChoice::Low)),
        Some(2) => Some(BetPlacement::LowHigh(LowHighChoice::High)),
        Some(_) => {
            show_message("Opcion invalida", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_dozen_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA DOCENA", "blanco");
    println!("1. Primera docena (1-12)");
    println!("2. Segunda docena (13-24)");
    println!("3. Tercera docena (25-36)");
    print_flush("Seleccione (1/2/3): ");
    
    match read_u32() {
        Some(choice) if (1..=3).contains(&choice) => Some(BetPlacement::Dozen(choice)),
        Some(_) => {
            show_message("Opcion invalida", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_column_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA COLUMNA", "blanco");
    println!("1. Columna 1 (1,4,7,10,13,16,19,22,25,28,31,34)");
    println!("2. Columna 2 (2,5,8,11,14,17,20,23,26,29,32,35)");
    println!("3. Columna 3 (3,6,9,12,15,18,21,24,27,30,33,36)");
    print_flush("Seleccione (1/2/3): ");
    
    match read_u32() {
        Some(choice) if (1..=3).contains(&choice) => Some(BetPlacement::Column(choice)),
        Some(_) => {
            show_message("Opcion invalida", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_street_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA CALLE (fila de 3 numeros)", "blanco");
    println!("Calles disponibles:");
    for i in 1..=12 {
        let start = (i - 1) * 3 + 1;
        let end = start + 2;
        print!("{}: {}-{}  ", i, start, end);
        if i % 4 == 0 {
            println!();
        }
    }
    println!();
    print_flush("Seleccione calle (1-12): ");
    
    match read_u32() {
        Some(choice) if (1..=12).contains(&choice) => Some(BetPlacement::Street(choice)),
        Some(_) => {
            show_message("Calle invalida", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_corner_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA CUADRO (4 numeros en cuadrado)", "blanco");
    println!("Cuadros disponibles:");
    let corners = vec![
        (1, "1,2,4,5"), (2, "2,3,5,6"), (3, "4,5,7,8"), (4, "5,6,8,9"),
        (5, "7,8,10,11"), (6, "8,9,11,12"), (7, "10,11,13,14"), (8, "11,12,14,15"),
        (9, "13,14,16,17"), (10, "14,15,17,18"), (11, "16,17,19,20"), (12, "17,18,20,21"),
        (13, "19,20,22,23"), (14, "20,21,23,24"), (15, "22,23,25,26"), (16, "23,24,26,27"),
        (17, "25,26,28,29"), (18, "26,27,29,30"), (19, "28,29,31,32"), (20, "29,30,32,33"),
        (21, "31,32,34,35"), (22, "32,33,35,36")
    ];
    for (num, nums) in &corners {
        print!("{}:{}  ", num, nums);
        if num % 4 == 0 {
            println!();
        }
    }
    println!();
    print_flush("Seleccione cuadro (1-22): ");
    
    match read_u32() {
        Some(choice) if (1..=22).contains(&choice) => Some(BetPlacement::Corner(choice)),
        Some(_) => {
            show_message("Cuadro invalido", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

fn get_sixline_details() -> Option<BetPlacement> {
    show_message("\nAPUESTA SEISENA (6 numeros)", "blanco");
    println!("Seisenas disponibles:");
    for i in 1..=11 {
        let start = match i {
            1 => 1, 2 => 4, 3 => 7, 4 => 10, 5 => 13,
            6 => 16, 7 => 19, 8 => 22, 9 => 25, 10 => 28, 11 => 31, _ => 0
        };
        let end = start + 5;
        print!("{}: {}-{}  ", i, start, end);
        if i % 4 == 0 {
            println!();
        }
    }
    println!();
    print_flush("Seleccione seisena (1-11): ");
    
    match read_u32() {
        Some(choice) if (1..=11).contains(&choice) => Some(BetPlacement::SixLine(choice)),
        Some(_) => {
            show_message("Seisena invalida", "morado");
            None
        }
        None => {
            show_message("Entrada invalida", "morado");
            None
        }
    }
}

pub fn show_spinning() {
    print!("Girando la ruleta");
    let _ = io::stdout().flush();
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(500));
        print!(".");
        let _ = io::stdout().flush();
    }
    thread::sleep(Duration::from_millis(500));
    println!();
}

pub fn show_win(result: &crate::roulette::RouletteResult, win_amount: f64) {
    let color_code = match result.get_color() {
        Color::Rojo => "rojo",
        Color::Negro => "gris",
        Color::Verde => "verde",
    };
    
    println!("\n{}", color_text("=", "amarillo").repeat(80));
    println!(
        "{} {}",
        color_text("¡RESULTADO!", "amarillo"),
        color_text(&format!("Numero {} - {}", result.number, result.get_color()), color_code)
    );
    println!(
        "{} ${:.2}",
        color_text("¡HAS GANADO!", "amarillo"),
        win_amount
    );
    println!("{}", color_text("=", "amarillo").repeat(80));
}

pub fn show_loss(result: &crate::roulette::RouletteResult) {
    let color_code = match result.get_color() {
        Color::Rojo => "rojo",
        Color::Negro => "gris",
        Color::Verde => "verde",
    };
    
    println!("\n{}", color_text("=", "morado").repeat(80));
    println!(
        "{} {}",
        color_text("RESULTADO:", "morado"),
        color_text(&format!("Numero {} - {}", result.number, result.get_color()), color_code)
    );
    println!("{}", color_text("HAS PERDIDO", "morado"));
    println!("{}", color_text("=", "morado").repeat(80));
}

pub fn wait_for_enter() {
    println!("\nPresione Enter para continuar...");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

fn print_flush(message: &str) {
    print!("{}", message);
    let _ = io::stdout().flush();
}

fn read_u32() -> Option<u32> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    input.trim().parse().ok()
}