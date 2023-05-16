// Purpose: the main file for the project.

// import the necessary libraries
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::stdin;
use std::process::exit;
use serde_json;


// the load_team function loads and returns data from the file stakes.json
// the data is a mapping of team member names to their stakes.
// the data is in json format.
// the data is loaded into a HashMap.
// the HashMap is returned.
fn load_team() -> HashMap<String, u32> {
    // open the file
    let file = File::open("stakes.json").unwrap();
    // create a buffered reader
    let reader = BufReader::new(file);
    // load the data into a HashMap
    let team: HashMap<String, u32> = serde_json::from_reader(reader).unwrap();
    // return the HashMap
    team
}


fn store_team(team: &HashMap<String, u32>) ->() {
    // open the file
    let file = File::create("stakes.json").unwrap();
    // create a buffered writer
    let writer = BufWriter::new(file);
    // write the data to the file
    serde_json::to_writer(writer, team).unwrap();
}

fn print_team(team: &HashMap<String, u32>) -> () {
    for (name, stake) in team {
        println!("{}: {}", name, stake);
    }
}

// the choose_chairman function chooses a chairman from the team.
// the choice process is a random pick on the HashMap keys
// the individual odds of being selected are the HasMap values.
// the function returns the name of the chairman.
fn choose_chairman(team: &HashMap<String, u32>) -> String {
    let mut names: Vec<&String> = Vec::new();
    let mut stakes: Vec<u32> = Vec::new();
    let mut total_stakes = 0;
    for (name, stake) in team {
        names.push(name);
        stakes.push(*stake);
        total_stakes += stake;
    }
    
    // pick the chairman from the names vector.
    let mut chairman = String::new();
    let mut rng = rand::thread_rng();

    let mut random_stake = rng.gen_range(0, total_stakes);
    for (index, stake) in stakes.iter().enumerate() {
        if random_stake < *stake {
            chairman = names[index].clone();
            break;
        }
        else {
            random_stake -= *stake;
        }
    }
    // return the chairman name
    chairman
}

// the update_stakes function increment the stake of all the team members
// and then resets to 0 the stake of the chairman.
// the function takes as input the team HashMap and the name of the chairman.
// the update is made in-place; the function returns nothing.
fn update_stakes(team: &mut HashMap<String, u32>, chairman: &String) -> () {
    // increment the stakes of all the team members
    for (name, stake) in team.iter_mut() {
        *stake += 1;
    }
    // reset the stake of the chairman
    team.insert(chairman.clone(), 0);
}


fn main() {
    // load the team data
    let mut team = load_team();
    // print the team data
    print_team(&team);
    // choose the chairman
    let chairman = choose_chairman(&team);
    println!("The chairman is: {}", chairman);
    // update the stakes
    update_stakes(&mut team, &chairman);
    // print the updated team data
    print_team(&team);
    // store the updated team data
    store_team(&team);
}