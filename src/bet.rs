use crate::roulette::{RouletteResult, RouletteWheel, Color};

pub struct Bet {
    pub amount: f64,
    pub bet_type: BetType,
    pub placement: BetPlacement,
}

impl Bet {
    pub fn new(amount: f64, bet_type: BetType, placement: BetPlacement) -> Self {
        Bet {
            amount,
            bet_type,
            placement,
        }
    }
    
    pub fn calculate_win(&self, result: &RouletteResult, wheel: &RouletteWheel) -> f64 {
        if self.is_winner(result, wheel) {
            self.amount * self.get_payout_multiplier()
        } else {
            0.0
        }
    }
    
    fn is_winner(&self, result: &RouletteResult, wheel: &RouletteWheel) -> bool {
        match &self.bet_type {
            BetType::Straight => {
                if let BetPlacement::Number(num) = self.placement {
                    result.number == num
                } else {
                    false
                }
            }
            BetType::Color => {
                if let BetPlacement::Color(color) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        result.get_color() == color
                    }
                } else {
                    false
                }
            }
            BetType::EvenOdd => {
                if let BetPlacement::EvenOdd(choice) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        match choice {
                            EvenOddChoice::Even => result.is_even(),
                            EvenOddChoice::Odd => result.is_odd(),
                        }
                    }
                } else {
                    false
                }
            }
            BetType::LowHigh => {
                if let BetPlacement::LowHigh(choice) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        match choice {
                            LowHighChoice::Low => result.is_low(),
                            LowHighChoice::High => result.is_high(),
                        }
                    }
                } else {
                    false
                }
            }
            BetType::Dozen => {
                if let BetPlacement::Dozen(dozen) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        result.dozen() == Some(*dozen)
                    }
                } else {
                    false
                }
            }
            BetType::Column => {
                if let BetPlacement::Column(col) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        result.column() == Some(*col)
                    }
                } else {
                    false
                }
            }
            BetType::Street => {
                if let BetPlacement::Street(street) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        result.street() == Some(*street)
                    }
                } else {
                    false
                }
            }
            BetType::Corner => {
                if let BetPlacement::Corner(corner_num) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        let numbers = RouletteWheel::corner_numbers(*corner_num);
                        numbers.contains(&result.number)
                    }
                } else {
                    false
                }
            }
            BetType::SixLine => {
                if let BetPlacement::SixLine(sixline_num) = &self.placement {
                    if result.number == 0 {
                        false
                    } else {
                        let numbers = RouletteWheel::sixline_numbers(*sixline_num);
                        numbers.contains(&result.number)
                    }
                } else {
                    false
                }
            }
        }
    }
    
    fn get_payout_multiplier(&self) -> f64 {
        match self.bet_type {
            BetType::Straight => 35.0,
            BetType::Color => 1.0,
            BetType::EvenOdd => 1.0,
            BetType::LowHigh => 1.0,
            BetType::Dozen => 2.0,
            BetType::Column => 2.0,
            BetType::Street => 11.0,
            BetType::Corner => 8.0,
            BetType::SixLine => 5.0,
        }
    }
}

#[derive(Clone)]
pub enum BetType {
    Straight,
    Color,
    EvenOdd,
    LowHigh,
    Dozen,
    Column,
    Street,
    Corner,
    SixLine,
}

#[derive(Clone)]
pub enum BetPlacement {
    Number(u32),
    Color(Color),
    EvenOdd(EvenOddChoice),
    LowHigh(LowHighChoice),
    Dozen(u32),
    Column(u32),
    Street(u32),
    Corner(u32),
    SixLine(u32),
}

#[derive(Clone)]
pub enum EvenOddChoice {
    Even,
    Odd,
}

#[derive(Clone)]
pub enum LowHighChoice {
    Low,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roulette::{RouletteResult, Color, RouletteWheel};
    
    fn create_test_wheel() -> RouletteWheel {
        RouletteWheel::new()
    }
    
    #[test]
    fn test_straight_win_pays_35_to_1() {
        let bet = Bet::new(10.0, BetType::Straight, BetPlacement::Number(7));
        let result = RouletteResult::new(7, Color::Rojo);
        let wheel = create_test_wheel();
        assert_eq!(bet.calculate_win(&result, &wheel), 350.0);
    }
    
    #[test]
    fn test_straight_loss_returns_zero() {
        let bet = Bet::new(10.0, BetType::Straight, BetPlacement::Number(7));
        let result = RouletteResult::new(15, Color::Negro);
        let wheel = create_test_wheel();
        assert_eq!(bet.calculate_win(&result, &wheel), 0.0);
    }
    
    #[test]
    fn test_zero_loses_all_even_money_bets() {
        let result = RouletteResult::new(0, Color::Verde);
        let wheel = create_test_wheel();
        
        let color_bet = Bet::new(10.0, BetType::Color, BetPlacement::Color(Color::Rojo));
        assert_eq!(color_bet.calculate_win(&result, &wheel), 0.0);
        
        let even_bet = Bet::new(10.0, BetType::EvenOdd, BetPlacement::EvenOdd(EvenOddChoice::Even));
        assert_eq!(even_bet.calculate_win(&result, &wheel), 0.0);
        
        let low_bet = Bet::new(10.0, BetType::LowHigh, BetPlacement::LowHigh(LowHighChoice::Low));
        assert_eq!(low_bet.calculate_win(&result, &wheel), 0.0);
    }
    
    #[test]
    fn test_color_bet_wins() {
        let result = RouletteResult::new(1, Color::Rojo);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::Color, BetPlacement::Color(Color::Rojo));
        assert_eq!(bet.calculate_win(&result, &wheel), 20.0);
    }
    
    #[test]
    fn test_color_bet_loses() {
        let result = RouletteResult::new(1, Color::Rojo);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::Color, BetPlacement::Color(Color::Negro));
        assert_eq!(bet.calculate_win(&result, &wheel), 0.0);
    }
    
    #[test]
    fn test_even_bet_wins() {
        let result = RouletteResult::new(2, Color::Negro);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::EvenOdd, BetPlacement::EvenOdd(EvenOddChoice::Even));
        assert_eq!(bet.calculate_win(&result, &wheel), 20.0);
    }
    
    #[test]
    fn test_odd_bet_wins() {
        let result = RouletteResult::new(3, Color::Rojo);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::EvenOdd, BetPlacement::EvenOdd(EvenOddChoice::Odd));
        assert_eq!(bet.calculate_win(&result, &wheel), 20.0);
    }
    
    #[test]
    fn test_low_bet_wins() {
        let result = RouletteResult::new(10, Color::Negro);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::LowHigh, BetPlacement::LowHigh(LowHighChoice::Low));
        assert_eq!(bet.calculate_win(&result, &wheel), 20.0);
    }
    
    #[test]
    fn test_high_bet_wins() {
        let result = RouletteResult::new(25, Color::Rojo);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::LowHigh, BetPlacement::LowHigh(LowHighChoice::High));
        assert_eq!(bet.calculate_win(&result, &wheel), 20.0);
    }
    
    #[test]
    fn test_dozen_bet_wins() {
        let result = RouletteResult::new(15, Color::Negro);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::Dozen, BetPlacement::Dozen(2));
        assert_eq!(bet.calculate_win(&result, &wheel), 30.0);
    }
    
    #[test]
    fn test_column_bet_wins() {
        let result = RouletteResult::new(2, Color::Negro);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::Column, BetPlacement::Column(2));
        assert_eq!(bet.calculate_win(&result, &wheel), 30.0);
    }
    
    #[test]
    fn test_street_bet_wins() {
        let result = RouletteResult::new(4, Color::Negro);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::Street, BetPlacement::Street(2));
        assert_eq!(bet.calculate_win(&result, &wheel), 120.0);
    }
    
    #[test]
    fn test_corner_bet_wins() {
        let result = RouletteResult::new(5, Color::Rojo);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::Corner, BetPlacement::Corner(1));
        assert_eq!(bet.calculate_win(&result, &wheel), 90.0);
    }
    
    #[test]
    fn test_sixline_bet_wins() {
        let result = RouletteResult::new(5, Color::Rojo);
        let wheel = create_test_wheel();
        let bet = Bet::new(10.0, BetType::SixLine, BetPlacement::SixLine(1));
        assert_eq!(bet.calculate_win(&result, &wheel), 60.0);
    }
    
    #[test]
    fn test_payout_multipliers() {
        let wheel = create_test_wheel();
        let result = RouletteResult::new(10, Color::Negro);
        
        let straight_bet = Bet::new(1.0, BetType::Straight, BetPlacement::Number(10));
        assert_eq!(straight_bet.calculate_win(&result, &wheel), 35.0);
        
        let color_bet = Bet::new(1.0, BetType::Color, BetPlacement::Color(Color::Negro));
        assert_eq!(color_bet.calculate_win(&result, &wheel), 2.0);
        
        let dozen_bet = Bet::new(1.0, BetType::Dozen, BetPlacement::Dozen(1));
        assert_eq!(dozen_bet.calculate_win(&result, &wheel), 3.0);
        
        let street_bet = Bet::new(1.0, BetType::Street, BetPlacement::Street(4));
        assert_eq!(street_bet.calculate_win(&result, &wheel), 12.0);
        
        let corner_bet = Bet::new(1.0, BetType::Corner, BetPlacement::Corner(3));
        assert_eq!(corner_bet.calculate_win(&result, &wheel), 9.0);
        
        let sixline_bet = Bet::new(1.0, BetType::SixLine, BetPlacement::SixLine(2));
        assert_eq!(sixline_bet.calculate_win(&result, &wheel), 6.0);
    }
}