bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct PublicFlags: u8 {
        const BULLET = 0x01;
        const TOWER = 0x02;
        const DAMAGE = 0x04;
        const ENEMY = 0x08;
        const UPGRADE = 0x10;
    }
}
