mod tools;
use tools::*;


fn main() {

    let mut field = Field {
        party: vec![EntityType::Hunter.instance(), 
    EntityType::Orbat.instance()],
    
        gamemaster: vec![EntityType::Butler.instance(),
    EntityType::Butler.instance()]
    };



    // field.party[0].signs.push(SignType::Wet.instance());

    // field.party[0].signs.push(SignType::Burning.instance());

    field.render();

}


