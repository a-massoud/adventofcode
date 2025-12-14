#[derive(Debug, Clone, Copy)]
pub struct Round {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Round {
    pub fn new(red: i32, green: i32, blue: i32) -> Self {
        Self { red, green, blue }
    }

    pub fn fits(&self, red: i32, green: i32, blue: i32) -> bool {
        red >= self.red && green >= self.green && blue >= self.blue
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn new() -> Self {
        Self { rounds: Vec::new() }
    }

    pub fn fits(&self, red: i32, green: i32, blue: i32) -> bool {
        self.rounds.iter().all(|x| x.fits(red, green, blue))
    }

    pub fn min_power(&self) -> i32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for round in &self.rounds {
            if round.red > red {
                red = round.red;
            }
            if round.green > green {
                green = round.green;
            }
            if round.blue > blue {
                blue = round.blue;
            }
        }

        red * green * blue
    }
}
