use crate::entities::*;

const KEY: [char; 10] = ['q', 'w', 'e', 'a', 's', 'd', 'z', 'x', 'c', 'v'];

const WIDTH: usize = 80;
const HEIGHT: usize = 23;

pub struct Field {
    pub party: Vec<Entity>,
    pub gamemaster: Vec<Entity>
}

impl Field {
    pub fn render(&self) {
        let mut count: usize = 0;           // defines the keyboard bind for the entity

        for entity in &self.gamemaster {
            // print!("{})", KEY[count]);       // old design: DEPRECATED
            count += 1;
            // entity.render();
        }

        let sep = vec!["-"; WIDTH];
        print!("{}\n", sep.join(""));           // --- <- hyphen separator between half-fields
        print!("\n");
        print!("\n");



        let entities = &self.party;         
        let n = entities.len();

        let mut pos_diff: Vec<usize> = vec![0; n];      // as many positions as entities

        pos_diff[0] = WIDTH/(n+1) - entities[1].width()/2;

        for i in 1..n {
            pos_diff[i] = WIDTH/(n+1) - entities[i].width()/2 - entities[i-1].width()/2;
        }

        for i in 0..n {

            let spaces = vec![" "; pos_diff[i]];
            print!("{}", spaces.join(""));

            print!("{}) ", KEY[count]);
            count += 1;

            print!("{}", entities[i].body.name);

            // entity.render();
        }


        println!("");


        for i in 0..n {

            let spaces = vec![" "; pos_diff[i]];
            print!("{}", spaces.join(""));

            print!("   HP: {}", entities[i].body.health);

        }


        println!("");


        for i in 0..n {
            let spaces = vec![" "; pos_diff[i]];
            print!("{}", spaces.join(""));

            print!("   ");
            for sign in &entities[i].body.signs {
                sign.render();
            }

        }


        println!("");
        println!("");

    }
}




