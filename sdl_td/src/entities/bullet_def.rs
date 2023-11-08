use super::damage_type_def::DamageType;

pub enum BulletShapeType { Projectile, Beam }

pub struct BulletTypeDef {
    pub name: String,
    pub speed: i32,
    pub color: Color,
    pub bulletType: BulletShapeType,
    pub dmg_type: DamageType
}