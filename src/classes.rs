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
    Elemental,
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
