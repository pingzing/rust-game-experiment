use player::Player;
use enemy::Enemy;

pub struct Action {
    pub group_id: u32,    
    pub action: Box<Fn(&mut Player, &mut Enemy) -> Option<u32>>,
    pub description: String
}
