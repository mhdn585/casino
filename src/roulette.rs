use rand::Rng;

pub struct RouletteWheel {
    numbers: Vec<u32>,
    colors: Vec<&'static str>,
}

impl RouletteWheel {
    pub fn new() -> Self {
        let numbers = vec![
            0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10, 5,
            24, 16, 33, 1, 20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26
        ];
        
        let colors = vec![
            "verde", "rojo", "negro", "rojo", "negro", "rojo", "negro", "rojo", "negro", "rojo",
            "negro", "rojo", "negro", "rojo", "negro", "rojo", "negro", "rojo", "negro", "rojo",
            "negro", "rojo", "negro", "rojo", "negro", "rojo", "negro", "rojo", "negro", "rojo",
            "negro", "rojo", "negro", "rojo", "negro", "rojo", "negro"
        ];
        
        RouletteWheel { numbers, colors }
    }
    
    pub fn spin(&self) -> RouletteResult {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..37);
        let number = self.numbers[index];
        let color = self.colors[index];
        
        RouletteResult::new(number, color)
    }
    
    pub fn get_number_color(&self, number: u32) -> Option<&str> {
        for i in 0..self.numbers.len() {
            if self.numbers[i] == number {
                return Some(self.colors[i]);
            }
        }
        None
    }
    
    pub fn get_column(&self, number: u32) -> Option<u32> {
        if number == 0 {
            return None;
        }
        
        let col = match number % 3 {
            1 => 1,
            2 => 2,
            0 => 3,
            _ => return None,
        };
        
        Some(col)
    }
    
    pub fn get_dozen(&self, number: u32) -> Option<u32> {
        if number == 0 {
            return None;
        }
        
        match number {
            1..=12 => Some(1),
            13..=24 => Some(2),
            25..=36 => Some(3),
            _ => None,
        }
    }
    
    pub fn get_street(&self, number: u32) -> Option<u32> {
        if number == 0 {
            return None;
        }
        
        Some((number - 1) / 3 + 1)
    }
    
    pub fn numbers_in_street(&self, street: u32) -> Vec<u32> {
        let start = (street - 1) * 3 + 1;
        (start..start+3).collect()
    }
    
    pub fn numbers_in_column(&self, column: u32) -> Vec<u32> {
        let mut nums = Vec::new();
        for i in 0..12 {
            let num = column + (i * 3);
            if num <= 36 {
                nums.push(num);
            }
        }
        nums
    }
    
    pub fn numbers_in_dozen(&self, dozen: u32) -> Vec<u32> {
        match dozen {
            1 => (1..=12).collect(),
            2 => (13..=24).collect(),
            3 => (25..=36).collect(),
            _ => Vec::new(),
        }
    }
    
    pub fn numbers_in_corner(&self, corner_num: u32) -> Vec<u32> {
        match corner_num {
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
    
    pub fn numbers_in_sixline(&self, sixline_num: u32) -> Vec<u32> {
        match sixline_num {
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
    pub color: String,
}

impl RouletteResult {
    pub fn new(number: u32, color: &str) -> Self {
        RouletteResult {
            number,
            color: color.to_string(),
        }
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
    
    pub fn get_color(&self) -> &str {
        &self.color
    }
}