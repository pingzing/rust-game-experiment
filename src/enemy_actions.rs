use combat_action::Action;
use unit_base::*;

pub fn get_enemy_actions() -> Vec<Action> {
    let enemy_fight = Action {
        id: 1,
        action: Box::new(move |player, enemy| {
            let damage_dealt = enemy.attack_target(player);
            return Some(damage_dealt);
        }),
        description: "The {0} deals {1} damage to you!".to_string()
    };
    
    let enemy_actions = vec![enemy_fight];
    return enemy_actions;
}