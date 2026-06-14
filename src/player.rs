use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};

const DATA_DIR: &str = "data";
const SALDO_FILE: &str = "data/saldo.json";
const INITIAL_BALANCE: f64 = 1000.0;

#[derive(Serialize, Deserialize)]
pub struct Player {
    balance: f64,
    total_won: f64,
    total_lost: f64,
    bets_placed: u32,
    bets_won: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            balance: INITIAL_BALANCE,
            total_won: 0.0,
            total_lost: 0.0,
            bets_placed: 0,
            bets_won: 0,
        }
    }
    
    pub fn load_or_create() -> Self {
        if !Path::new(DATA_DIR).exists() {
            fs::create_dir_all(DATA_DIR).expect("No se pudo crear la carpeta data");
        }
        
        if Path::new(SALDO_FILE).exists() {
            let mut file = File::open(SALDO_FILE).expect("No se pudo abrir saldo.json");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("No se pudo leer saldo.json");
            
            match serde_json::from_str(&contents) {
                Ok(player) => player,
                Err(_) => {
                    println!("Archivo de saldo corrupto. Creando nuevo.");
                    Player::new()
                }
            }
        } else {
            Player::new()
        }
    }
    
    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).expect("No se pudo serializar el jugador");
        let mut file = File::create(SALDO_FILE).expect("No se pudo crear saldo.json");
        file.write_all(json.as_bytes()).expect("No se pudo escribir saldo.json");
    }
    
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
    
    pub fn can_afford(&self, amount: f64) -> bool {
        amount > 0.0 && amount <= self.balance
    }
    
    pub fn place_bet(&mut self, amount: f64) {
        self.balance -= amount;
        self.bets_placed += 1;
        self.total_lost += amount;
    }
    
    pub fn add_winnings(&mut self, amount: f64) {
        self.balance += amount;
        self.total_won += amount;
        self.bets_won += 1;
        self.total_lost -= amount;
    }
    
    pub fn get_total_won(&self) -> f64 {
        self.total_won
    }
    
    pub fn get_total_lost(&self) -> f64 {
        self.total_lost
    }
    
    pub fn get_bets_placed(&self) -> u32 {
        self.bets_placed
    }
    
    pub fn get_bets_won(&self) -> u32 {
        self.bets_won
    }
    
    pub fn get_bets_lost(&self) -> u32 {
        self.bets_placed - self.bets_won
    }
    
    pub fn get_win_rate(&self) -> f64 {
        if self.bets_placed == 0 {
            0.0
        } else {
            (self.bets_won as f64 / self.bets_placed as f64) * 100.0
        }
    }
    
    pub fn reset_stats(&mut self) {
        self.total_won = 0.0;
        self.total_lost = 0.0;
        self.bets_placed = 0;
        self.bets_won = 0;
    }
    
    pub fn reset_balance(&mut self) {
        self.balance = INITIAL_BALANCE;
        self.reset_stats();
    }
}