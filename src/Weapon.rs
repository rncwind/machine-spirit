use crate::Die::Die;


/// A record type that defines a Weapon's stats
///
/// A weapon in 9th edition 40k is comprised of both a [WeaponType](Weapon Tyoe)
/// which defines the broad "category" of a weapon, as well as how many "to hit"
/// die it rolls; the strength of the weapon, it's armour piercing characteristics,
/// and damage which defines how many wound rolls each successful hit roll causes.
pub struct Weapon {
    pub wt: WeaponType,
    pub s: u8,
    pub ap: u8,
    pub d: DamageType,
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
#[derive(PartialEq)]
pub enum WeaponType {
    Assault(DamageType), Heavy(DamageType), RapidFire(DamageType),
    Grenade(DamageType), Pistol(DamageType), Blast(DamageType)
}


/// Damage is either an absolute value, or a die roll for wound count.
///
/// Generally there are two types of damage in 40k 9th edition. Die based
/// damage (Eg, roll 1d6 for how many wounds this hit does) and absolute damage
/// such as a boltgun which does 1 wound.
#[derive(PartialEq)]
pub enum DamageType {
    DieDamage(Die), Absolute(u16)
}

#[cfg(test)]
mod WeaponTests {
    use super::*;

    #[test]
    fn construct_lasgun() {
        // Rapid fire 1
        let wt = WeaponType::RapidFire(DamageType::Absolute(1));
        // Damage stat of 1
        let damage = DamageType::Absolute(1);
        let lasgun = Weapon{wt, s: 3, ap: 0, d: damage};
        assert!(lasgun.wt == WeaponType::RapidFire(DamageType::Absolute(1)));
        assert!(lasgun.ap == 0);
        assert!(lasgun.s == 3);
        assert!(lasgun.d == DamageType::Absolute(1));
    }
}
