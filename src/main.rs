struct Entity {
    name: String,
    health: u8,
    signs: Vec<Sign>,
    moves: Vec<Move>,
}

impl Entity {
    fn new(name: String, health: u8) -> Self {
        Self {
            name,
            health,
            signs: Vec::new(),
            moves: Vec::new(),
        }
    }

    fn render(&self) {
        println!("    {}", self.name);
        println!("    HP: {}", self.health);
        print!("    ");
        for sign in &self.signs {
            // print!("{}", sign.shortname);
            sign.render();
        }
        print!("\n");
        print!("\n");
    }
}

struct Sign {
    name: String,
    shortname: String,
    color: Color,
}

impl Sign {
    fn new(name: String, shortname: String, color: Color) -> Self {
        Self {
            name,
            shortname,
            color,
        }
    }

    fn render(&self) {
        print!("\x1b[{}m{}\x1b[0m", self.color.get_code(), self.shortname);
    }
}

struct Move {
    name: String,
    damage: Dice,
    effect: Sign,
    strength: Application,
}

enum Dice {
    D4,
    D6,
}

enum Color {
    NoColor,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    BrightRed,
    DodgerBlue
}

impl Color {
    fn get_code(&self) -> &str {
        match &self {
            Self::Black => "30",
            Self::Red => "31",
            Self::Green => "32",
            Self::Yellow => "33",
            Self::Blue => "34",
            Self::Magenta => "35",
            Self::Cyan => "36",
            Self::White => "37",
            Self::Gray => "90",
            Self::BrightRed => "91",
            Self::DodgerBlue => "38;2;30;144;255",
            Self::NoColor => "Not implemented",
        }
    }
}

enum Application {
    Weak,
    Strong,
}

fn main() {
    let mut gunter = Entity::new(String::from("Hunter"), 19);

    let queimado = Sign::new(String::from("Burning"), String::from("Brn"), Color::Red);

    gunter.signs.push(queimado);

    let molhado = Sign::new(String::from("Wet"), String::from("Wet"), Color::Blue);

    gunter.signs.push(molhado);

    let envenenado = Sign::new(String::from("Poisoned"), String::from("Psn"), Color::Green);

    gunter.signs.push(envenenado);

    gunter.render();

    let mut orbat = Entity::new(String::from("Orbat"), 5);

    orbat.render();
}
