pub struct EnemyWaveTypeDef {
    pub enemy: &EnemyTypeDef,
    pub count: i32,
    pub rate: f32,
}

pub struct MapTypeDef {
    pub waves: Vec<Vec<EnemeyWaveTypeDef>>, // Each Wave can spawn multiple enemy types
    pub health: u8,
    pub map: &[u8],
}