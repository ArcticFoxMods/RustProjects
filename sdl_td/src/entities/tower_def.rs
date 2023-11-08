pub struct TowerTypeDef {
    pub rate_of_fire: f32,
    pub bullets_fired: i32,
    pub tower_shoot_radius: f32,
    pub bullet_def: &BulletTypeDef,
    pub damage: i32,
    pub upgrades: Vec<&UpgradeTypeDef> 
}