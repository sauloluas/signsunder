use crate::signs::*;
use crate::classes::*;


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