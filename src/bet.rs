use crate::roulette::{RouletteWheel, RouletteResult};

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
    
    pub fn calculate_win(&self, result: &RouletteResult) -> f64 {
        if self.is_winner(result) {
            self.amount * self.get_payout_multiplier()
        } else {
            0.0
        }
    }
    
    fn is_winner(&self, result: &RouletteResult) -> bool {
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
                        let result_dozen = match result.number {
                            1..=12 => 1,
                            13..=24 => 2,
                            25..=36 => 3,
                            _ => 0,
                        };
                        result_dozen == *dozen
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
                        let result_col = result.number % 3;
                        match result_col {
                            1 => *col == 1,
                            2 => *col == 2,
                            0 => *col == 3,
                            _ => false,
                        }
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
                        let result_street = (result.number - 1) / 3 + 1;
                        result_street == *street
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
                        let wheel = RouletteWheel::new();
                        let numbers = wheel.numbers_in_corner(*corner_num);
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
                        let wheel = RouletteWheel::new();
                        let numbers = wheel.numbers_in_sixline(*sixline_num);
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
    Color(String),
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