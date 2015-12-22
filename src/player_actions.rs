use combat_action::Action;
use unit_base::CanAttack;

pub fn get_player_actions() -> Vec<Action> {
    let player_fight = Action {
        id: 0,
        action: Box::new(move |player, enemy| {
            player.attack_target(enemy);
        }),
        description: "You attack {}, and deal {} damage!".to_string()
    };
    
    let player_actions = vec![player_fight];
    return player_actions;
}