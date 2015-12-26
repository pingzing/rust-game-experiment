use player::Player;
use enemy::Enemy;

pub struct Action {
    pub id: u32,    
    pub action: Box<Fn(&mut Player, &mut Enemy) -> Option<u32>>,
    pub description: String
}
