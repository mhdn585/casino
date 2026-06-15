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
            if let Err(e) = fs::create_dir_all(DATA_DIR) {
                eprintln!("Error al crear la carpeta data: {}", e);
                return Player::new();
            }
        }
        
        if Path::new(SALDO_FILE).exists() {
            match File::open(SALDO_FILE) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        eprintln!("Error al leer saldo.json: {}", e);
                        return Player::new();
                    }
                    
                    match serde_json::from_str(&contents) {
                        Ok(player) => player,
                        Err(e) => {
                            eprintln!("Archivo de saldo corrupto ({}). Creando nuevo.", e);
                            Player::new()
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error al abrir saldo.json: {}", e);
                    Player::new()
                }
            }
        } else {
            Player::new()
        }
    }
    
    pub fn save(&self) {
        if !Path::new(DATA_DIR).exists() {
            if let Err(e) = fs::create_dir_all(DATA_DIR) {
                eprintln!("Error al crear la carpeta data para guardar: {}", e);
                return;
            }
        }
        
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                match File::create(SALDO_FILE) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(json.as_bytes()) {
                            eprintln!("Error al escribir saldo.json: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error al crear saldo.json: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error al serializar el jugador: {}", e);
            }
        }
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

impl Drop for Player {
    fn drop(&mut self) {
        self.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;
    use std::path::PathBuf;
    
    fn setup_test_env() -> PathBuf {
        let temp_dir = tempdir().unwrap();
        let data_dir = temp_dir.path().join("data");
        let original_data_dir = DATA_DIR;
        std::fs::create_dir_all(&data_dir).unwrap();
        data_dir
    }
    
    #[test]
    fn test_new_player() {
        let player = Player::new();
        assert_eq!(player.get_balance(), INITIAL_BALANCE);
        assert_eq!(player.get_total_won(), 0.0);
        assert_eq!(player.get_total_lost(), 0.0);
        assert_eq!(player.get_bets_placed(), 0);
        assert_eq!(player.get_bets_won(), 0);
        assert_eq!(player.get_win_rate(), 0.0);
    }
    
    #[test]
    fn test_can_afford() {
        let player = Player::new();
        assert!(player.can_afford(100.0));
        assert!(player.can_afford(INITIAL_BALANCE));
        assert!(!player.can_afford(INITIAL_BALANCE + 1.0));
        assert!(!player.can_afford(0.0));
        assert!(!player.can_afford(-10.0));
    }
    
    #[test]
    fn test_place_bet() {
        let mut player = Player::new();
        let amount = 100.0;
        let initial_balance = player.get_balance();
        
        player.place_bet(amount);
        
        assert_eq!(player.get_balance(), initial_balance - amount);
        assert_eq!(player.get_bets_placed(), 1);
        assert_eq!(player.get_total_lost(), amount);
        assert_eq!(player.get_bets_won(), 0);
    }
    
    #[test]
    fn test_add_winnings() {
        let mut player = Player::new();
        let bet_amount = 100.0;
        let win_amount = 350.0;
        let initial_balance = player.get_balance();
        
        player.place_bet(bet_amount);
        player.add_winnings(win_amount);
        
        assert_eq!(player.get_balance(), initial_balance - bet_amount + win_amount);
        assert_eq!(player.get_bets_placed(), 1);
        assert_eq!(player.get_bets_won(), 1);
        assert_eq!(player.get_total_won(), win_amount);
        assert_eq!(player.get_total_lost(), bet_amount);
    }
    
    #[test]
    fn test_win_rate() {
        let mut player = Player::new();
        assert_eq!(player.get_win_rate(), 0.0);
        
        player.place_bet(10.0);
        player.add_winnings(20.0);
        assert_eq!(player.get_win_rate(), 100.0);
        
        player.place_bet(10.0);
        assert_eq!(player.get_win_rate(), 50.0);
        
        player.place_bet(10.0);
        player.add_winnings(20.0);
        assert_eq!(player.get_win_rate(), 66.66666666666666);
    }
    
    #[test]
    fn test_reset_stats() {
        let mut player = Player::new();
        player.place_bet(100.0);
        player.add_winnings(350.0);
        player.place_bet(50.0);
        
        player.reset_stats();
        
        assert_eq!(player.get_total_won(), 0.0);
        assert_eq!(player.get_total_lost(), 0.0);
        assert_eq!(player.get_bets_placed(), 0);
        assert_eq!(player.get_bets_won(), 0);
        assert_eq!(player.get_balance(), INITIAL_BALANCE);
    }
    
    #[test]
    fn test_reset_balance() {
        let mut player = Player::new();
        player.place_bet(500.0);
        player.add_winnings(1000.0);
        
        player.reset_balance();
        
        assert_eq!(player.get_balance(), INITIAL_BALANCE);
        assert_eq!(player.get_total_won(), 0.0);
        assert_eq!(player.get_total_lost(), 0.0);
        assert_eq!(player.get_bets_placed(), 0);
        assert_eq!(player.get_bets_won(), 0);
    }
    
    #[test]
    fn test_get_bets_lost() {
        let mut player = Player::new();
        assert_eq!(player.get_bets_lost(), 0);
        
        player.place_bet(10.0);
        assert_eq!(player.get_bets_lost(), 1);
        
        player.add_winnings(20.0);
        assert_eq!(player.get_bets_lost(), 0);
        
        player.place_bet(10.0);
        player.place_bet(10.0);
        assert_eq!(player.get_bets_lost(), 2);
    }
    
    #[test]
    fn test_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let original_data_dir = DATA_DIR;
        
        let mut player = Player::new();
        player.place_bet(100.0);
        player.add_winnings(350.0);
        
        let serialized = serde_json::to_string(&player).unwrap();
        let deserialized: Player = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(deserialized.get_balance(), player.get_balance());
        assert_eq!(deserialized.get_total_won(), player.get_total_won());
        assert_eq!(deserialized.get_total_lost(), player.get_total_lost());
        assert_eq!(deserialized.get_bets_placed(), player.get_bets_placed());
        assert_eq!(deserialized.get_bets_won(), player.get_bets_won());
    }
}