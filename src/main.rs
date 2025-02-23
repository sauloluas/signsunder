mod tools;
use tools::*;

use std::io;


fn main() {
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
        

        field.render();

        // Entity::new(EntityHead::Orbat);

        

}


