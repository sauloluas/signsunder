const KEY: [char; 10] = ['q', 'w', 'e', 'a', 's', 'd', 'z', 'x', 'c', 'v'];




pub struct Entity {
    pub head: EntityHead,
    pub body: EntityBody
}

impl Entity {
    pub fn new(head: EntityHead) -> Entity {
        return head.instance();
    }

    pub fn render(&self) {
        println!("    {}", self.body.name);
        println!("      HP: {}", self.body.health);
        print!("      ");
        for sign in &self.body.signs {
            sign.render();
        }
        print!("\n");
        print!("\n");
    }
}

pub struct EntityBody {
    pub name: &'static str,
    pub health: u8,
    pub signs: Vec<Sign>,
    pub moves: Vec<Move>,
}

impl EntityBody {
    pub fn new(name: &'static str, health: u8) -> EntityBody {
        EntityBody {
            name,
            health,
            signs: Vec::new(),
            moves: Vec::new()   
        }
    }
}

pub enum EntityHead {
    Hunter,
    Orbat,
    Vampire,
    Butler,
    NoEntity
}

impl EntityHead {
    pub fn instance(&self) -> Entity {
        match &self {
            EntityHead::Hunter => Entity {
                body: EntityBody::new("Hunter", 18),
                head: EntityHead::Hunter
            },
            EntityHead::Orbat => Entity {
                body: EntityBody::new("Orbat", 8),
                head: EntityHead::Orbat
            },
            EntityHead::Butler => Entity {
                body: EntityBody::new("Butler", 13),
                head: EntityHead::Butler
            },
            _ => Entity {
                body: EntityBody::new("NoEntity", 1),
                head: EntityHead::NoEntity
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
    head: SignHead,
    body: SignBody
}

impl Sign {
    pub fn new(head: SignHead) -> Sign {
        return head.instance();
    }

    pub fn render(&self) {
        print!("\x1b[{}m{}\x1b[0m", self.body.color.get_code(), self.body.shortname);  // Future Works: implement Display trait
    }
}

pub struct SignBody {
    pub name: &'static str,
    pub shortname: &'static str,
    pub color: Color,
    pub class: Class
}

pub enum SignHead {
    NoEffect,
    Burning,
    Wet,
    Poisoned,
    Cut
}

impl SignHead {
    pub fn instance(&self) -> Sign {                    // Future Works: possibility of adding HashMap
        match &self {
            SignHead::NoEffect => Sign {
                body: SignBody {
                    name: "No effect",
                    shortname: "Nef",
                    color: Color::White,
                    class: ClassHead::NoClass.instance()
                },
                head: SignHead::NoEffect
            },
            SignHead::Burning => Sign {
                body: SignBody {
                    name: "Burning",
                    shortname: "Brn",
                    color: Color::Red,
                    class: ClassHead::Elemental.instance()
                },
                head: SignHead::Burning
            },
            SignHead::Wet => Sign {
                body: SignBody {
                    name: "Wet",
                    shortname: "Wet",
                    color: Color::Blue,
                    class: ClassHead::Elemental.instance()
                },
                head: SignHead::Wet
            },
            SignHead::Poisoned => Sign {
                body: SignBody {
                    name: "Poisoned",
                    shortname: "Psn",
                    color: Color::Green,
                    class: ClassHead::Physical.instance()
                },
                head: SignHead::Poisoned
            },
            SignHead::Cut => Sign {
                body: SignBody {
                    name: "Cut",
                    shortname: "Cut",
                    color: Color::Gray,
                    class: ClassHead::Physical.instance()
                },
                head: SignHead::Cut
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
    body: MoveBody,
    head: MoveHead
}

impl Move {
    pub fn new(head: MoveHead) -> Move {
        return head.instance();
    }
}

pub struct MoveBody {
    pub name: &'static str,
    pub damage: Dice,
    pub effect: Sign,
    pub strength: Application,
    pub class: Class
}

pub enum MoveHead {
    Slash,
    Pull
}

impl MoveHead {
    pub fn instance(&self) -> Move {
        match &self {
            MoveHead::Slash => Move {
                body: MoveBody {
                    name: "Slash",
                    damage: Dice::D4,
                    effect: SignHead::Cut.instance(),
                    strength: Application::Weak,
                    class: ClassHead::Physical.instance()
                },
                head: MoveHead::Slash
            },
            MoveHead::Pull => Move {
                body: MoveBody {
                    name: "Pull",
                    damage: Dice::D4,
                    effect: SignHead::NoEffect.instance(),
                    strength: Application::Intensifier,
                    class: ClassHead::Physical.instance()
                },
                head: MoveHead::Pull
            }
        }
    }
}




pub struct Class {
    body: ClassBody,
    head: ClassHead
}

impl Class {
    pub fn new(head: ClassHead) -> Class {
        return head.instance();
    }

    pub fn render(&self) {
        print!("\x1b[{}m{}\x1b[0m", self.body.color.get_code(), self.body.shortname);
    }
}

pub struct ClassBody {
    name: &'static str,
    shortname: &'static str,
    color: Color
}

pub enum ClassHead {
    NoClass,
    Physical,
    Elemental
}

impl ClassHead {
    pub fn instance(&self) -> Class {
        match &self {
            ClassHead::NoClass => Class {
                body: ClassBody {
                    name: "No Class",
                    shortname: "Ncl",
                    color: Color::White
                },
                head: ClassHead::NoClass
            },
            ClassHead::Physical => Class {
                body: ClassBody {
                    name: "Physical",
                    shortname: "Phys",
                    color: Color::Gray
                },
                head: ClassHead::Physical
            },
            ClassHead::Elemental => Class {
                body: ClassBody {
                    name: "Elemental",
                    shortname: "Elem",
                    color: Color::Blue
                },
                head: ClassHead::Elemental
            }
        }
    }
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