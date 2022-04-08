use crate::Die::Die;


/// A record type that defines a Weapon's stats
///
/// A weapon in 9th edition 40k is comprised of both a [WeaponType](Weapon Tyoe)
/// which defines the broad "category" of a weapon, as well as how many "to hit"
/// die it rolls; the strength of the weapon, it's armour piercing characteristics,
/// and damage which defines how many wound rolls each successful hit roll causes.
pub struct Weapon {
    wt: WeaponType,
    s: u8,
    ap: u8,
    d: DamageType,
    // There should actually be one more field here, the Abilities field.
    // This field is problematic, as it generally represents special rules
    // that are unique to induvidual weapons, or at least categories.
}


/// Defines the category of Weapon for a given Weapon (Rapid Fire, Grenade etc)
///
/// WeaponType defines the category of weapon that is in use. When reading
/// from a codex, this would be something like Rapid Fire 1 (for a bolter),
/// or Grenade D6 for a frag grenade.
/// The number next to the type is how many rolls are made to hit, so a
/// Pistol 1 rolls 1 "To Hit", wheras a Grenade D6 involves rolling 1d6
/// to see how many "rolls to hit" there are, which are then actually
/// rolled to hit.
enum WeaponType {
    Assault(DamageType), Heavy(DamageType), RapidFire(DamageType),
    Grenade(DamageType), Pistol(DamageType), Blast(DamageType)
}


/// Damage is either an absolute value, or a die roll for wound count.
///
/// Generally there are two types of damage in 40k 9th edition. Die based
/// damage (Eg, roll 1d6 for how many wounds this hit does) and absolute damage
/// such as a boltgun which does 1 wound.
enum DamageType {
    DieDamage(Die), Absolute(u16)
}
