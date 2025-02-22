const KEY: [char; 10] = ['q', 'w', 'e', 'a', 's', 'd', 'z', 'x', 'c', 'v'];






pub struct Entity {
    pub name: &'static str,
    pub health: u8,
    pub signs: Vec<Sign>,
    pub moves: Vec<Move>,
}

impl Entity {
    pub fn render(&self) {
        println!("    {}", self.name);
        println!("      HP: {}", self.health);
        print!("      ");
        for sign in &self.signs {
            sign.render();
        }
        print!("\n");
        print!("\n");
    }
}

pub enum EntityType {
    Hunter,
    Orbat,
    Vampire,
    Butler
}

impl EntityType {
    pub fn instance(&self) -> Entity {
        match &self {
            EntityType::Hunter => Entity {
                name: "Hunter",
                health: 18,
                signs: Vec::new(),
                moves: vec![MoveType::Slash.instance()]

            },
            EntityType::Orbat => Entity {
                name: "Orbat",
                health: 8,
                signs: Vec::new(),
                moves: vec![MoveType::Pull.instance()]
            },
            EntityType::Butler => Entity {
                name: "Butler",
                health: 13,
                signs: Vec::new(),
                moves: vec![]
            },
            _ => Entity{
                name: "Unknown",
                health: 1,
                signs: Vec::new(),
                moves: Vec::new()
            }
        }
    }
}






pub struct Field {
    pub party: Vec<Entity>,
    pub gamemaster: Vec<Entity>
}

impl Field {
    pub fn render(&self) {
        let mut count: usize = 0;

        for entity in &self.gamemaster {
            print!("{})", KEY[count]);
            count += 1;
            entity.render();
        }

        print!("--------------------------\n");
        print!("\n");
        print!("\n");

        for entity in &self.party {
            print!("{})", KEY[count]);
            count += 1;
            entity.render();
        }

    }
}






pub struct Sign {
    pub name: &'static str,
    pub shortname: &'static str,
    pub color: Color,
    pub class: Class
}

impl Sign {
    pub fn render(&self) {
        print!("\x1b[{}m{}\x1b[0m", self.color.get_code(), self.shortname);  // Future Works: implement Display trait
    }
}

pub enum SignType {
    NoEffect,
    Burning,
    Wet,
    Poisoned,
    Cut
}

impl SignType {
    pub fn instance(&self) -> Sign {                    // Future Works: possibility of adding HashMap
        match &self {
            SignType::NoEffect => Sign {
                name: "No effect",
                shortname: "Nef",
                color: Color::White,
                class: Class::NoClass
            },
            SignType::Burning => Sign {
                name: "Burning",
                shortname: "Brn",
                color: Color::Red,
                class: Class::Elemental
            },
            SignType::Wet => Sign {
                name: "Wet",
                shortname: "Wet",
                color: Color::Blue,
                class: Class::Elemental
            },
            SignType::Poisoned => Sign {
                name: "Poisoned",
                shortname: "Psn",
                color: Color::Green,
                class: Class::Physical
            },
            SignType::Cut => Sign {
                name: "Cut",
                shortname: "Cut",
                color: Color::Gray,
                class: Class::Physical
            }
        }
    }
}

pub enum Color {
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
    pub fn get_code(&self) -> &str {
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






pub struct Move {
    pub name: &'static str,
    pub damage: Dice,
    pub effect: Sign,
    pub strength: Application,
    pub class: Class
}

pub enum MoveType {
    Slash,
    Pull
}

impl MoveType {
    pub fn instance(&self) -> Move {
        match &self {
            MoveType::Slash => Move {
                name: "Slash",
                damage: Dice::D4,
                effect: SignType::Cut.instance(),
                strength: Application::Weak,
                class: Class::Physical
            },
            MoveType::Pull => Move {
                name: "Pull",
                damage: Dice::D4,
                effect: SignType::NoEffect.instance(),
                strength: Application::Intensifier,
                class: Class::Physical
            }
        }
    }
}






pub enum Class {
    NoClass,
    Physical,
    Elemental
}






pub enum Dice {
    Coin,
    D4,
    D6,
}

pub enum Application {
    Weak,
    Strong,
    Intensifier
}