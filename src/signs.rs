use crate::classes::*;

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
