use game_state::*;
use enemy::*;
use player::*;
use parsing::strings::*;
use rand::{thread_rng, Rng};

pub struct BattleCoordinator {
   pub player: Player,
   pub enemy: Option<Enemy>
}

impl BattleCoordinator {    
    pub fn take_turn(&mut self, input: u32, curr_state: &State) -> State {
        
        self.player_turn(input, curr_state);
        self.enemy_turn(curr_state);              
                
        let fight_action = super::player_actions::get_player_actions().remove(0);
        let next_turn_option = OptionPair {
            option_number: 1,
            option_description: "Fight".to_string(),
            option_action: Some(fight_action)
        };
        let new_state = State{
            state_description: format!("You have {} hp. The enemy has {} hp.", &self.player.health, self.enemy.as_ref().unwrap().health),
            state_options: vec![next_turn_option],
            is_combat_state: true
        };        
        return new_state;        
    }      
    
    //todo: properly print for different actions
    fn player_turn(&mut self, input: u32, curr_state: &State) {
        for pair in &curr_state.state_options {
            if pair.option_number == input {
                match pair.option_action.as_ref(){
                    Some(act) => {
                        let action_result = (act.action)(&mut self.player, self.enemy.as_mut().unwrap());
                        match action_result {
                            Some(result) => {
                                let desc_string = ordered_inject(&act.description, vec![&self.enemy.as_ref().unwrap().name, &result.to_string()]);
                                println!("{}", desc_string);
                            }
                            None => {
                                let desc_string = ordered_inject(&"You missed the {0}.".to_string(), vec![&self.enemy.as_ref().unwrap().name]);
                                println!("{}", desc_string);
                            }
                        }                                                                       
                    }
                    None => println!("You didn't do anything.")
                }                                                                    
            }
        }
    }   
    
    fn enemy_turn(&mut self, curr_state: &State) { 
        let mut rng = thread_rng();                                 
        let mut enemy_actions = super::enemy_actions::get_enemy_actions();
        let mut temp_index = 0;
        if(enemy_actions.len() > 1 as usize) {
            temp_index = rng.gen_range(1, enemy_actions.len() - 1 as usize);     
        }
        
        let chosen_index = temp_index;
        let chosen_action = enemy_actions.remove(chosen_index);
        let action_result = (chosen_action.action)(&mut self.player, self.enemy.as_mut().unwrap());
        match action_result {
            Some(result) => {
                let desc_string = ordered_inject(&chosen_action.description, vec![&self.enemy.as_ref().unwrap().name, &result.to_string()]);
                println!("{}", desc_string);
            }
            None => {
                let desc_string = ordered_inject(&"The {0} missed you!".to_string(), vec![&self.enemy.as_ref().unwrap().name]);
                println!("{}", desc_string);
            }
        }                
    }       
}
