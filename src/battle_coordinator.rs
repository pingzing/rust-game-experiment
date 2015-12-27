use game_state::*;
use enemy::*;
use player::*;
use parsing::strings::*;
use rand::{thread_rng, Rng};
use std::process;
use combat_action::*;

pub struct BattleCoordinator {
    pub player: Player,
    pub enemy: Option<Enemy>,
}

impl BattleCoordinator {
    fn get_gameover_state() -> State {
        State {
            state_description: "Corrosion crawls up your body, reducing you to a brittle \
                                husk."
                                   .to_string(),
            state_options: vec![OptionPair {
                                    option_number: 1,
                                    option_description: "End game".to_string(),
                                    option_action: Some(Action {
                                        group_id: 0,
                                        action: None,
                                        description: "You crumble to dust.".to_string(),
                                    }),
                                }],
            is_combat_state: false,
        }
    }

    pub fn take_turn(&mut self, input: u32, curr_state: &State) -> State {

        self.player_turn(input, curr_state);
        self.enemy_turn(curr_state);

        if self.player.health == 0 {
            let return_state = BattleCoordinator::get_gameover_state();
            return return_state;
        }

        let fight_action = super::player_actions::get_player_actions().remove(0);
        let next_turn_option = OptionPair {
            option_number: 1,
            option_description: "Fight".to_string(),
            option_action: Some(fight_action),
        };
        let new_state = State {
            state_description: format!("You have {} hp. The enemy has {} hp.",
                                       &self.player.health,
                                       self.enemy.as_ref().unwrap().health),
            state_options: vec![next_turn_option],
            is_combat_state: true,
        };
        return new_state;
    }

    // todo: properly print for different actions
    fn player_turn(&mut self, input: u32, curr_state: &State) {
        for pair in &curr_state.state_options {
            if pair.option_number == input {
                match pair.option_action.as_ref() {
                    Some(act) => {
                        let action_result = (act.action)(&mut self.player,
                                                         self.enemy.as_mut().unwrap());
                        match action_result {
                            Some(result) => {
                                let desc_string = ordered_inject(&act.description,
                                                                 vec![&self.enemy
                                                                           .as_ref()
                                                                           .unwrap()
                                                                           .name,
                                                                      &result.to_string()]);
                                println!("{}", desc_string);
                            }
                            None => {
                                let desc_string = ordered_inject(&"You missed the {0}."
                                                                      .to_string(),
                                                                 vec![&self.enemy
                                                                           .as_ref()
                                                                           .unwrap()
                                                                           .name]);
                                println!("{}", desc_string);
                            }
                        }
                    }
                    None => println!("You didn't do anything."),
                }
            }
        }
    }

    fn enemy_turn(&mut self, curr_state: &State) {
        match self.enemy {
            Some(ref mut curr_enemy) => {
                let mut rng = thread_rng();
                let actions_length = curr_enemy.get_actions().len();
                let mut temp_index = 0;
                if actions_length > 1 as usize {
                    temp_index = rng.gen_range(1, actions_length - 1 as usize);
                }

                let chosen_index = temp_index;
                let chosen_action = &curr_enemy.get_actions()[chosen_index];
                let action_result = (chosen_action.action)(&mut self.player, curr_enemy);
                match action_result {
                    Some(result) => {
                        let desc_string = ordered_inject(&chosen_action.description,
                                                         vec![&curr_enemy.name,
                                                              &result.to_string()]);
                        println!("{}", desc_string);
                    }
                    None => {
                        let desc_string = ordered_inject(&"The {0} missed you!".to_string(),
                                                         vec![&curr_enemy.name]);
                        println!("{}", desc_string);
                    }
                }
            }            
            None => {}                     
        }
    }
}
