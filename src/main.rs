mod classes;

mod signs;
use signs::*;

mod moves;
use moves::*;

mod entities;
use entities::*;

mod field;
use field::*;

use std::io;



fn main() {
    
    loop {
        print!("\x1B[2J\x1B[H"); // clears terminal and gets cursor to 0,0

        println!("                      ");
        println!("        signsunder    ");
        println!("                      ");
        println!("    s) Start game     ");
        println!("    q) Quit           ");
        println!("                      ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.chars().next() {
            Some('s') => {     
                print!("\x1B[2J\x1B[1;1H");     // temporary, needs reevaluation
                break;
            },
            Some('q') => return,
            Some(_) => continue,
            None => continue
        }


    }


    let mut field = Field {
        party: vec![

        Entity::new(EntityHead::Hunter), 
        Entity::new(EntityHead::Orbat),
        Entity::new(EntityHead::UltraViolence),
        Entity::new(EntityHead::Orbat)
        

        ],

        gamemaster: vec![

        Entity::new(EntityHead::Butler),
        Entity::new(EntityHead::Butler)

        ]
    };

    let queimando = Sign::new(SignHead::Burning);

    field.party[0].body.signs.push(queimando);
    field.party[0].body.signs.push(Sign::new(SignHead::Wet));
    field.party[1].body.signs.push(Sign::new(SignHead::Wet));
    field.party[1].body.signs.push(Sign::new(SignHead::Cut));
    field.party[2].body.signs.push(Sign::new(SignHead::Poisoned));
    
    

    field.render();

}


