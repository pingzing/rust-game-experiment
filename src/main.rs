use std::io;
mod player;
mod game_state;
mod unit_base;
mod combat_action;

fn main() {
    let starting_option_pair = game_state::OptionPair {
            option_number: 1,
            option_description: "Enter 1 to continue.".to_string(),
            option_action: None
        };
        
    let mut current_state = game_state::State {
        state_description: "This is the beginning state.".to_string(),
        state_options: vec![starting_option_pair],
        is_combat_state: false
    };
    
    print_new_state(&current_state);
    
    loop {
        let input_result = get_input();
        match input_result {
            Some(input) => {
                let result = update_state(input, &current_state);
                match result {
                    Some(new_state) => {
                        current_state = new_state; //new_state no longer owns the data, now belongs to current_state
                        print_new_state(&current_state);                
                    },
                    None => {
                        println!("Invalid input.");
                        print_new_state(&current_state);
                    }  
                }
            }
            None => {
                println!("Invalid input.");
                print_new_state(&current_state);
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
            let secondary_option_pair = game_state::OptionPair {
                option_number: 1,
                option_description: "This is the secondary optionPair. Press 1.".to_string(),
                option_action: None
            };
            
            let result = game_state::State {
                state_description: "This is the new state.".to_string(),
                state_options: vec![secondary_option_pair],
                is_combat_state: false
            };
            return Some(result);
        }        
    }
    return None;
}

fn print_new_state(result: &game_state::State) {    
    println!("{}", result.state_description);
    for pair in &result.state_options {
        println!(" {}. {}", pair.option_number, pair.option_description);
    }
}