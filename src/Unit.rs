use crate::Weapon::Weapon;

/// A model in 9th edition is a compound type of a stat line for the unit,
/// and a stat line for a weapon.
///
/// All models have at least one weapon in the current edition, and nobody
/// is unarmed.
struct Model {
    crunchline: UnitStats,
    weapon: Weapon,
}

struct UnitStats {
    ws: u8,
    bs: u8,
    s: u8,
    t: u8,
    w: u8,
    a: u8,
    ld: u8,
    sv: u8,
}
