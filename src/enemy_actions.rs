use combat_action::Action;
use unit_base::*;

pub fn get_enemy_actions() -> Vec<Action> {
    let enemy_fight = Action {
        id: 1,
        action: Box::new(move |player, enemy| {
            enemy.attack_target(player);
        }),
        description: "The {} deals {} damage to you!".to_string()
    };
    
    let enemy_actions = vec![enemy_fight];
    return enemy_actions;
}