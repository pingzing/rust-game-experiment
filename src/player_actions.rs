use combat_action::Action;
use unit_base::CanAttack;

pub fn get_player_actions() -> Vec<Action> {
    let player_fight = Action {
        id: 0,
        action: Box::new(move |player, enemy| {
            let damage_dealt = player.attack_target(enemy);
            return Some(damage_dealt);
        }),
        description: "You attack {0}, and deal {1} damage!".to_string()
    };
    
    let player_actions = vec![player_fight];
    return player_actions;
}