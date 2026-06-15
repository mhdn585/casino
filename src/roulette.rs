use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Verde,
    Rojo,
    Negro,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Verde => write!(f, "verde"),
            Color::Rojo => write!(f, "rojo"),
            Color::Negro => write!(f, "negro"),
        }
    }
}

pub struct RouletteWheel {
    numbers: Vec<u32>,
    colors: Vec<Color>,
}

impl RouletteWheel {
    pub fn new() -> Self {
        let numbers = vec![
            0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10, 5,
            24, 16, 33, 1, 20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26
        ];
        
        let colors = vec![
            Color::Verde, Color::Rojo, Color::Negro, Color::Rojo, Color::Negro, Color::Rojo,
            Color::Negro, Color::Rojo, Color::Negro, Color::Rojo, Color::Negro, Color::Rojo,
            Color::Negro, Color::Rojo, Color::Negro, Color::Rojo, Color::Negro, Color::Rojo,
            Color::Negro, Color::Rojo, Color::Negro, Color::Rojo, Color::Negro, Color::Rojo,
            Color::Negro, Color::Rojo, Color::Negro, Color::Rojo, Color::Negro, Color::Rojo,
            Color::Negro, Color::Rojo, Color::Negro, Color::Rojo, Color::Negro, Color::Rojo,
            Color::Negro
        ];
        
        RouletteWheel { numbers, colors }
    }
    
    pub fn spin(&self) -> RouletteResult {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..37);
        let number = self.numbers[index];
        let color = self.colors[index].clone();
        
        RouletteResult::new(number, color)
    }
    
    pub fn get_number_color(&self, number: u32) -> Option<Color> {
        for i in 0..self.numbers.len() {
            if self.numbers[i] == number {
                return Some(self.colors[i].clone());
            }
        }
        None
    }
    
    pub fn corner_numbers(corner: u32) -> Vec<u32> {
        match corner {
            1 => vec![1, 2, 4, 5],
            2 => vec![2, 3, 5, 6],
            3 => vec![4, 5, 7, 8],
            4 => vec![5, 6, 8, 9],
            5 => vec![7, 8, 10, 11],
            6 => vec![8, 9, 11, 12],
            7 => vec![10, 11, 13, 14],
            8 => vec![11, 12, 14, 15],
            9 => vec![13, 14, 16, 17],
            10 => vec![14, 15, 17, 18],
            11 => vec![16, 17, 19, 20],
            12 => vec![17, 18, 20, 21],
            13 => vec![19, 20, 22, 23],
            14 => vec![20, 21, 23, 24],
            15 => vec![22, 23, 25, 26],
            16 => vec![23, 24, 26, 27],
            17 => vec![25, 26, 28, 29],
            18 => vec![26, 27, 29, 30],
            19 => vec![28, 29, 31, 32],
            20 => vec![29, 30, 32, 33],
            21 => vec![31, 32, 34, 35],
            22 => vec![32, 33, 35, 36],
            _ => Vec::new(),
        }
    }
    
    pub fn sixline_numbers(sixline: u32) -> Vec<u32> {
        match sixline {
            1 => (1..=6).collect(),
            2 => (4..=9).collect(),
            3 => (7..=12).collect(),
            4 => (10..=15).collect(),
            5 => (13..=18).collect(),
            6 => (16..=21).collect(),
            7 => (19..=24).collect(),
            8 => (22..=27).collect(),
            9 => (25..=30).collect(),
            10 => (28..=33).collect(),
            11 => (31..=36).collect(),
            _ => Vec::new(),
        }
    }
}

pub struct RouletteResult {
    pub number: u32,
    pub color: Color,
}

impl RouletteResult {
    pub fn new(number: u32, color: Color) -> Self {
        RouletteResult { number, color }
    }
    
    pub fn is_even(&self) -> bool {
        self.number != 0 && self.number % 2 == 0
    }
    
    pub fn is_odd(&self) -> bool {
        self.number != 0 && self.number % 2 != 0
    }
    
    pub fn is_low(&self) -> bool {
        self.number >= 1 && self.number <= 18
    }
    
    pub fn is_high(&self) -> bool {
        self.number >= 19 && self.number <= 36
    }
    
    pub fn get_color(&self) -> &Color {
        &self.color
    }
    
    pub fn column(&self) -> Option<u32> {
        if self.number == 0 {
            None
        } else {
            Some(match self.number % 3 {
                1 => 1,
                2 => 2,
                _ => 3,
            })
        }
    }
    
    pub fn dozen(&self) -> Option<u32> {
        if self.number == 0 {
            None
        } else {
            Some(match self.number {
                1..=12 => 1,
                13..=24 => 2,
                25..=36 => 3,
                _ => unreachable!(),
            })
        }
    }
    
    pub fn street(&self) -> Option<u32> {
        if self.number == 0 {
            None
        } else {
            Some((self.number - 1) / 3 + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_european_wheel_has_37_numbers() {
        let wheel = RouletteWheel::new();
        assert_eq!(wheel.numbers.len(), 37);
        assert_eq!(wheel.colors.len(), 37);
    }
    
    #[test]
    fn test_zero_is_green() {
        let wheel = RouletteWheel::new();
        assert_eq!(wheel.get_number_color(0), Some(Color::Verde));
    }
    
    #[test]
    fn test_even_odd() {
        let result = RouletteResult::new(10, Color::Negro);
        assert!(result.is_even());
        assert!(!result.is_odd());
        
        let result = RouletteResult::new(0, Color::Verde);
        assert!(!result.is_even());
        assert!(!result.is_odd());
    }
    
    #[test]
    fn test_low_high() {
        let result = RouletteResult::new(1, Color::Rojo);
        assert!(result.is_low());
        assert!(!result.is_high());
        
        let result = RouletteResult::new(19, Color::Rojo);
        assert!(!result.is_low());
        assert!(result.is_high());
        
        let result = RouletteResult::new(0, Color::Verde);
        assert!(!result.is_low());
        assert!(!result.is_high());
    }
    
    #[test]
    fn test_column() {
        let result = RouletteResult::new(1, Color::Rojo);
        assert_eq!(result.column(), Some(1));
        
        let result = RouletteResult::new(2, Color::Negro);
        assert_eq!(result.column(), Some(2));
        
        let result = RouletteResult::new(3, Color::Rojo);
        assert_eq!(result.column(), Some(3));
        
        let result = RouletteResult::new(0, Color::Verde);
        assert_eq!(result.column(), None);
    }
    
    #[test]
    fn test_dozen() {
        let result = RouletteResult::new(5, Color::Rojo);
        assert_eq!(result.dozen(), Some(1));
        
        let result = RouletteResult::new(20, Color::Negro);
        assert_eq!(result.dozen(), Some(2));
        
        let result = RouletteResult::new(30, Color::Rojo);
        assert_eq!(result.dozen(), Some(3));
        
        let result = RouletteResult::new(0, Color::Verde);
        assert_eq!(result.dozen(), None);
    }
    
    #[test]
    fn test_street() {
        let result = RouletteResult::new(1, Color::Rojo);
        assert_eq!(result.street(), Some(1));
        
        let result = RouletteResult::new(3, Color::Rojo);
        assert_eq!(result.street(), Some(1));
        
        let result = RouletteResult::new(4, Color::Negro);
        assert_eq!(result.street(), Some(2));
        
        let result = RouletteResult::new(36, Color::Rojo);
        assert_eq!(result.street(), Some(12));
        
        let result = RouletteResult::new(0, Color::Verde);
        assert_eq!(result.street(), None);
    }
    
    #[test]
    fn test_corner_numbers() {
        assert_eq!(RouletteWheel::corner_numbers(1), vec![1, 2, 4, 5]);
        assert_eq!(RouletteWheel::corner_numbers(22), vec![32, 33, 35, 36]);
        assert_eq!(RouletteWheel::corner_numbers(99), Vec::<u32>::new());
    }
    
    #[test]
    fn test_sixline_numbers() {
        assert_eq!(RouletteWheel::sixline_numbers(1), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(RouletteWheel::sixline_numbers(11), vec![31, 32, 33, 34, 35, 36]);
        assert_eq!(RouletteWheel::sixline_numbers(99), Vec::<u32>::new());
    }
}