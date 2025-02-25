use crate::Sign;
use crate::Move;


pub struct Entity {                             // TODO: create container class for Entity, which represents it render
    pub head: EntityHead,
    pub body: EntityBody
}

impl Entity {
    pub fn new(head: EntityHead) -> Entity {
        return head.instance();
    }

    pub fn width(&self) -> usize {
        let nw: usize = self.name_width();
        let hw: usize = self.health_width();
        let sw: usize = self.signs_width();       // Future Works: signs_width needs to be redesigned for different sign shortname sizes

        if nw > hw && nw > sw {
            nw
        } else if hw > sw {
            hw
        } else {
            sw
        }

    }

    pub fn name_width(&self) -> usize {
        self.body.name.chars().count() + 3
    }

    pub fn health_width(&self) -> usize {
        self.body.health.to_string().len() + 4
    }

    pub fn signs_width(&self) -> usize {
        self.body.signs.len()*3
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
    UltraViolence,
    Atrocity,
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
            EntityHead::UltraViolence => Entity {
                body: EntityBody::new("UltraViolence", 50),
                head: EntityHead::UltraViolence
            },
            _ => Entity {
                body: EntityBody::new("NoEntity", 1),
                head: EntityHead::NoEntity
            }
        }
    }
}
