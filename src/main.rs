extern crate rand;

mod player;
mod game_state;
mod unit_base;
mod combat_action;
mod enemy;
mod battle_coordinator;
mod player_actions;
mod enemy_actions;
mod parsing;

use std::io;
use player::Player;
use game_state::*;
use battle_coordinator::BattleCoordinator;
use enemy::Enemy;   

fn main() {
    let starting_option_pair = game_state::OptionPair {
            option_number: 1,
            option_description: "Enter 1 to continue.".to_string(),
            option_action: None
        };                                       
    
    let mut current_state = game_state::State {
        state_description: "This is the beginning state.".to_string(),
        state_options: vec![starting_option_pair],
        is_combat_state: true
    };    
    
    let start_player = initialize_new_player();
    let start_enemy = initialize_new_enemy();        
    
    let mut battle_coordinator = BattleCoordinator {
        player: start_player,
        enemy: Some(start_enemy)
    };
    
    print_new_state(&current_state);        
    
    loop {
        let input_result = get_input();
        match input_result {
            Some(input) => {
                let result = update_state(input, &current_state, &mut battle_coordinator);
                match result {
                    Some(new_state) => {
                        current_state = new_state; //new_state no longer owns the data, now belongs to current_state
                        print_new_state(&current_state);                
                    },
                    None => {
                        invalid_input_hadler(&current_state);
                    }  
                }
            }
            None => {
                invalid_input_hadler(&current_state);
            }
        };        
    }
}

fn get_input() -> Option<u32> {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => return Some(i),
        Err(..) => return None
    };    
}

fn update_state(input: u32, curr_state: &game_state::State, batt_coord: &mut BattleCoordinator) -> Option<game_state::State> {
    for pair in &curr_state.state_options {
        if pair.option_number == input {            
            if curr_state.is_combat_state {
                let result = batt_coord.take_turn(input, curr_state);
                return Some(result);
            }               
            else {
                let end_battle_options = OptionPair {
                        option_number: 1,
                        option_description: "Press 1 to end.".to_string(),
                        option_action: None
                    };
                let result = State {
                    state_description: "Battle complete!".to_string(),
                    state_options: vec![end_battle_options],
                    is_combat_state: false                    
                };
                return Some(result);
            }                                                        
        }        
    }
    return None;
}

fn print_new_state(result: &game_state::State) {    
    println!("{}", result.state_description);
    for pair in &result.state_options {
        println!("{}. {}", pair.option_number, pair.option_description);
    }
}

fn invalid_input_hadler(current_state: &State) {
    println!("Invalid input.");
    print_new_state(&current_state);
}

fn initialize_new_player() -> Player {
    let player = Player {
        health: 100,
        name: "Ferrous".to_string(),
        base_damage_reduction: 10f64,
        base_attack_damage: 5
    };
    
    return player;
}

fn initialize_new_enemy() -> Enemy {
    let default_actions = enemy_actions::get_enemy_actions();
    let enemy = Enemy {
        health: 100,
        name: "Hydrofiend".to_string(),
        base_damage_reduction: 5f64,
        base_attack_damage: 10,
        actions: default_actions
    };
    
    return enemy;
}