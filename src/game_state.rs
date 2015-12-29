use combat_action::Action;
use player::Player;
use enemy::Enemy;

pub struct State{
	pub state_description: String,
	pub state_options: Vec<OptionPair>,
	pub is_combat_state: bool,
    pub player: Player,
    pub enemy: Option<Enemy>    
}

pub struct OptionPair {
	pub option_number: u32,
	pub option_description: String,
    pub option_action: Option<Action>
}