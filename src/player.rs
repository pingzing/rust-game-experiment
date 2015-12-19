use unit_base::*;

struct Player {
    pub health: i32,
	pub name: String,
	pub base_damage_reduction: f64,
	pub base_attack_damage: i32
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
	fn take_damage(&mut self, incoming_damage: i32) {
		let total_damage = incoming_damage as f64 * self.base_damage_reduction;
		self.health = self.health - total_damage as i32;
		if self.health <= 0  {
			self.die();
		}
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
	fn attack_target<T: IsUnit>(&self, target: &mut T) {
		let outgoing_damage = self.get_attack_damage();
		target.take_damage(outgoing_damage);
	}
}