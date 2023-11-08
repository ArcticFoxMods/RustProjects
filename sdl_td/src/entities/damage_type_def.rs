pub enum DamageType { Normal, Corrosive, Fire, Concussive, Shock}

pub struct DamageMultiplier {
    pub damage_type: DamageType,
    pub multiplier: f32,   
}