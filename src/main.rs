mod tools;
use tools::*;

use std::io;



fn main() {
    const WIDTH: u16 = 80;
    const HEIGHT: u16 = 23;

    loop {
        print!("\x1B[2J\x1B[H"); // clears terminal and gets cursor to 0,0

        println!("");
        println!("        signsunder");
        println!("");
        println!("    s) Start game");
        println!("    q) Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.chars().next() {
            Some('s') => {     
                print!("\x1B[2J\x1B[1;1H");
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
    field.party[0].body.signs.push(Sign::new(SignHead::Wet));
    
    

    field.render();

}


