use game_state::*;
use enemy::*;
use player::*;
use unit_base::CanAttack;

pub struct BattleCoordinator {
   pub player: Player,
   pub enemy: Option<Enemy>
}

impl BattleCoordinator {    
    pub fn take_turn(&mut self, input: u32, curr_state: &State) -> State {        
        for pair in &curr_state.state_options {
            if pair.option_number == input {
                match pair.option_action.as_ref(){
                    Some(act) => {
                        (act.action)(&mut self.player, self.enemy.as_mut().unwrap());
                        println!("{}", &act.description);
                    }
                    None => println!("You didn't do anything.")
                }                                                                    
            }
        }
                
        self.enemy.as_ref().unwrap().attack_target(&mut self.player);        
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
}
