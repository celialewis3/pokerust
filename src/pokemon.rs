#[macro_use(lazy_static)]
use std::collections::HashMap;
use rand::Rng;
use lazy_static::lazy_static;
use std::io::*;



#[derive(Debug, Clone)]
struct PokeGroup {
    pokemon: Vec<Pokemon>,
    size: u32,
}

#[derive(Debug, Clone)]
struct Pokemon {
    name: String,
    _type: Type,
    level: u32,
    health: u32,
    moveset: MoveSet,
}

#[derive(Debug, Clone)]
struct MoveSet {
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
struct Move {
    name: String,
    _type: Type,
    damage: u32,
}


lazy_static! {
    static ref DEX: HashMap<u32, Pokemon> = {
        let mut m = HashMap::new();
        m.insert(0, Pokemon::new("Charmander".to_string(),Type::Fire, 5, 50, moveset()));
        m.insert(1, Pokemon::new("Squirtle".to_string(),Type::Water, 6, 52, moveset()));
        m.insert(2, Pokemon::new("Ivysaur".to_string(),Type::Grass, 4, 24, moveset()));
        m
    };
}

fn moveset() -> MoveSet {
    return MoveSet { moves: vec![Move {name: String::from("Ember"), _type: Type::Fire, damage: 10},
                                          Move {name: String::from("Tackle"), _type: Type::Normal, damage: 5},
                                          Move {name: String::from("Coals"), _type: Type::Fire, damage: 7}]
                                     };
}


impl PokeGroup {

    // Supply pokemon indexes, and it will populate the pokemon party using the DEX HashMap
    pub fn new(pokemon: Vec<u32>) -> Self {


        let mut group: Vec<Pokemon> = Vec::new();
        for pkmn in pokemon.iter() {
            group.push(DEX.get(&pkmn).unwrap().clone())
        }

        Self {
            pokemon: group,
            size: 2,
        }
    }
    
}

impl Pokemon {

    pub fn new(name: String, _type: Type, level: u32, health: u32, moveset: MoveSet) -> Self {
        Self {
            name,
            _type,
            level,
            health,
            moveset,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Type {
    Fire,
    Grass,
    Water,
    Normal,
}

pub fn simulate() {

    let mut party = PokeGroup::new(vec![1,2]);

    let encounter_group = PokeGroup {
        pokemon: vec![Pokemon::new("Ivysaur".to_string(),Type::Grass, 4, 24, moveset()), 
                      Pokemon::new("Charmeleon".to_string(),Type::Fire, 5, 40, moveset()), 
                      Pokemon::new("Cyndaquil".to_string(),Type::Fire, 7, 45, moveset())],
        size: 3,
    };

    for x in 0..10 {
        grass_step(&encounter_group, &mut party);
    }
}

// Pick a random pokemon from a given group size and return its index in the group
fn rand_pkmn(group_size: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..group_size)
    
}


// Simulates a player stepping through a grass tile
fn grass_step(possible_encounters: &PokeGroup, party: &mut PokeGroup) {
    // Check if I encounter something
    if encounter() {
        let pkmn_index = rand_pkmn(possible_encounters.size) as usize;
        let party_pkmn_index = rand_pkmn(party.size) as usize;

        let mut encountered_pkmn = possible_encounters.pokemon[pkmn_index].clone();

        println!("");
        println!("A level {} {} jumps out of the grass!", encountered_pkmn.level, encountered_pkmn.name);
        println!("Your level {} {} attacks", party.pokemon[party_pkmn_index].level, party.pokemon[party_pkmn_index].name);

        // Battle happens here

        battle(&mut (party.pokemon[party_pkmn_index]), &mut encountered_pkmn);

        println!("");

    } else {
        println!("No pokemon jump out.");
    }
    // If I do, grab a random pokemon to encounter,
    // print what pokemon I encounter
}

// Check if I encounter something
fn encounter() -> bool {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..3) {
        0 => true,
        1 => true,
        2 => false,
        _ => false,
    }
}

fn battle(active: &mut Pokemon, encountered: &mut Pokemon) {
    println!("{}: {:?}", active.name, active.moveset.moves);
    println!("{}: {:?}", encountered.name, encountered.moveset.moves);

}

