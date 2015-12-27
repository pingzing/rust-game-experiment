use unit_base::*;

#[derive(Debug)]
pub struct Player {
    pub health: i32,
	pub name: String,
	pub base_damage_reduction: f64,
	pub base_attack_damage: i32
}

impl Player {
    pub fn get_blank_player() -> Player {
    Player {
        health: 1,
        name: "Blank".to_string(),
        base_damage_reduction: 0f64,
        base_attack_damage: 0
    }
}
}

impl IsUnit for Player {
	fn get_health(&self) -> i32 {
		return self.health;
	}
	fn get_name(&self) -> &str {					
		return &self.name;
	}
	fn get_damage_reduction(&self) -> f64 {
		return self.base_damage_reduction;
	}
	fn take_damage(&mut self, incoming_damage: i32) -> u32 {
		let total_damage = incoming_damage as f64 * 1f64 - (self.base_damage_reduction/100f64);
		self.health = self.health - total_damage as i32;
		if self.health <= 0  {
			self.die();
		}
        return total_damage as u32;
	}
	fn die(&mut self) {
		self.health = 0;
		//trigger game over somehow? Maybe also battle system's job
	}
}

impl CanAttack for Player {
	fn get_attack_damage(&self) -> i32 {
		return self.base_attack_damage;				
	}
	fn attack_target<T: IsUnit>(&self, target: &mut T, damage_value: i32) -> u32 {
		let outgoing_damage = self.get_attack_damage() + damage_value;
		let damage_dealt = target.take_damage(outgoing_damage);
        return damage_dealt;
	}
}