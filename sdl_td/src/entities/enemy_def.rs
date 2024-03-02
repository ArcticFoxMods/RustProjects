

pub struct EnemyTypeDef {
    pub name: String,
    pub speed: f32,
    pub health: f32,
    pub color: Color,
    pub size: f32,
    pub gold_on_kill: i32,
    pub damage: i32,
    pub damage_multipliers: Vec<DamageMultiplier>
}