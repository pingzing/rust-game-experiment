use combat_action::Action;
use unit_base::*;
use rand::{thread_rng, Rng};

pub fn get_enemy_actions() -> Vec<Action> {
    let enemy_fight = Action {
        group_id: 1,
        action: Box::new(move |player, enemy| {
            let mut rng = thread_rng();            
            let outgoing_damage = rng.gen_range(2,5);
            let damage_dealt = enemy.attack_target(player, outgoing_damage);
            return Some(damage_dealt);
        }),
        description: "The {0} deals {1} damage to you!".to_string()
    };
    
    let enemy_actions = vec![enemy_fight];
    return enemy_actions;
}