use game_state::*;

pub struct BattleCoordinator {
   pub participants: Vec<T: IsUnit>
}

impl BattleCoordinator {
    fn take_turn(input: u32, curr_state: &State) -> State {
        for pair in &curr_state.state_options {
            if(pair.option_number == input && pair.option_action != None) {
                //call option_action.                                                
            }
        }                
    }
}