mod tools;
use tools::*;


fn main() {

    let mut field = Field {
        party: vec![EntityType::Hunter.instance(), 
    EntityType::Orbat.instance()],

        gamemaster: vec![EntityType::Butler.instance(),
    EntityType::Butler.instance()]
    };

    field.render();

    

}


