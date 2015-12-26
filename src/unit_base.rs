pub trait IsUnit {
	fn get_health(&self) -> i32;
	fn get_name(&self) -> &str;
	fn get_damage_reduction(&self) -> f64;
	fn take_damage(&mut self, incoming_damage: i32) -> u32;
	fn die(&mut self);
}

pub trait CanAttack {
	fn get_attack_damage(&self) -> i32;
	fn attack_target<T: IsUnit>(&self, target: &mut T) -> u32;
} 