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

/// A simple record type that defines the "crunch line" of a model.
struct UnitStats {
    // Hopefully none of these are greater than 256 at any point? AFAIK a warlord
    // titan is the maximum with a whole 70 wounds.
    ws: u8,
    bs: u8,
    s: u8,
    t: u8,
    w: u8,
    a: u8,
    ld: u8,
    sv: u8,
}
