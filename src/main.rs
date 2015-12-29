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
        
    let starting_option_pair_2 = game_state::OptionPair {
            option_number: 1,
            option_description: "Enter 1 to continue.".to_string(),
            option_action: None
        };                                               
    
    let mut current_state = game_state::State {
        state_description: "This is the beginning state.".to_string(),
        state_options: vec![starting_option_pair],
        is_combat_state: true,
        player: initialize_new_player(),
        enemy: None
    };    
    
    let mut previous_state = game_state::State {
        state_description: "Starting previous state".to_string(),
        state_options: vec![starting_option_pair_2],
        is_combat_state: false,
        player: current_state.player.clone(),
        enemy: None
    };       
    
    print_new_state(&current_state);        
    
    loop {
        let input_result = get_input();
        match input_result {
            Some(input) => {
                let result = update_state(input, &current_state);
                match result {
                    Some(new_state) => {
                        previous_state = current_state;
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

fn update_state(input: u32, curr_state: &game_state::State) -> Option<game_state::State> {
    for pair in &curr_state.state_options {
        if pair.option_number == input {            
            if curr_state.is_combat_state {
                if let Some(ref curr_enemy) = curr_state.enemy {
                let mut batt_coord = battle_coordinator::BattleCoordinator {
                    player: curr_state.player.clone(),
                    enemy: Some(curr_enemy.clone())
                };
                let result = batt_coord.take_turn(input, curr_state);
                return Some(result);
                }
                else {
                    let mut batt_coord = battle_coordinator::BattleCoordinator {
                        player: curr_state.player.clone(),
                        enemy: Some(initialize_new_enemy())
                    };
                    let result = batt_coord.take_turn(input, curr_state);
                    return Some(result);
                }
            }               
            else {
                //todo: make this return a state
                if let Some(ref act) = pair.option_action {               
                    let action = &act.action;
                    (action)(&mut Player::get_blank_player(), &mut Enemy::get_blank_enemy());                    
                }
                let battle_over_state = State {
                    state_description: "The battle has ended.".to_string(),
                    state_options: vec![OptionPair {
                        option_number: 1,
                        option_description: "End game".to_string(),
                        option_action: None
                    }],
                    is_combat_state: false,
                    player: curr_state.player.clone(),
                    enemy: None
                };
                return Some(battle_over_state);
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
        base_damage_reduction: 0f64,
        base_attack_damage: 5
    };
    
    return player;
}

fn initialize_new_enemy() -> Enemy {    
    let enemy = Enemy {
        id: 1,
        health: 100,
        name: "Hydrofiend".to_string(),
        base_damage_reduction: 5f64,
        base_attack_damage: 5,        
    };
    
    return enemy;
}