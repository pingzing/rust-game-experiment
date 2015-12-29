use combat_action::Action;
use unit_base::CanAttack;
use rand::{thread_rng, Rng};


pub fn get_player_actions() -> Vec<Action> {
    let player_fight = Action {
        group_id: 0,
        action: Box::new(move |player, enemy| {
            let mut rng = thread_rng();
            let outgoing_damage = rng.gen_range(2,5);
            let damage_dealt = player.attack_target(enemy, outgoing_damage);
            return Some(damage_dealt);
        }),
        description: "You attack {0}, and deal {1} damage!".to_string()
    };
    
    let player_actions = vec![player_fight];
    return player_actions;
}