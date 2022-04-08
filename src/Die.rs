use regex::Regex;
use rand::distributions::{Distribution, Uniform};

pub struct Die {
    pub die_string: String,
    pub die_count: u16,
    pub die_value: u16,
}

impl Die {
    /// Create a given die from the ad-hoc die string convention.
    ///
    /// A die string is typically comprised of two numbers, delimited
    /// by a "d" for die.
    ///
    /// For example, 1d6 means 1, 6-sided dice is rolled. 20d20 means that
    /// 20, 20-sided dice are rolled. This constructor produces a die from a given
    /// input diestring.
    ///
    /// If in the event that a given diestring cannot be parsed, this function
    /// will instead return a None type.
    ///
    /// # Example
    /// ```
    /// let d = Die::new_from_diestring("1d6").unwrap();
    /// ```
    pub fn new_from_diestring(die_string: &String) -> Option<Die> {
        if !validate_diestring(die_string.clone()) {
            return None;
        }
        let parts: Vec<String> = die_string.split("d").map(String::from).collect();
        let die_count = parts.get(0).unwrap().parse::<u16>().unwrap();
        let die_value = parts.get(1).unwrap().parse::<u16>().unwrap();
        let die_string = die_string.to_string();
        Some(Die{die_string, die_count, die_value})
    }

    /// Roll a given (bag of) di(c)e and return the results.
    ///
    /// Since our die implementation treats any given die as potentially being
    /// a bag, we will return a vector of results that represent each and
    /// every die being rolled.
    /// This means that the result of rolling a 1d6 is a 1 length vector
    /// of results, and the result of rolling 3d6 is a 3-length vector.
    pub fn roll(&self) -> Vec<u16> {
        let mut rng = rand::thread_rng();
        let diemax = self.die_value + 1;
        let distrib = Uniform::from(1..diemax);
        let mut rolls: Vec<u16> = Vec::new();
        for _i in 1..self.die_count+1 {
            rolls.push(distrib.sample(&mut rng));
        }
        rolls
    }
}


/// Validate a given input diestring
///
/// Since die strings are an ad-hoc "standard" for board games, we want to regex
/// match to ensure that the die strings we are given are actually valid.
///
/// We use the regex \d+d\d+ in order to parse the validity of a regex.
fn validate_diestring(die_string: String) -> bool {
    let re = Regex::new("\\d+d\\d+").unwrap();
    re.is_match(&die_string)
}

#[cfg(test)]
mod dice_tests {
    use super::Die;
    use super::validate_diestring;

    /// Check that our valid diestrings are being parsed properly.
    #[test]
    fn test_good_diestring() {
        let ds = String::from("1d6");
        let die = Die::new_from_diestring(&ds);
        assert!(die.is_some());
        let die = die.unwrap();
        assert!(die.die_count == 1);
        assert!(die.die_value == 6);
        assert!(die.die_string == ds);
    }

    /// Small check for bad die strings.
    #[test]
    fn test_bad_ds() {
        let ds = String::from("20abcd5");
        assert!(validate_diestring(ds) == false);
    }

    /// Ensure that our bag roller works.
    #[test]
    fn roll_5d20() {
        // Create a bag of 5 D20s
        let ds = String::from("5d20");
        let bag = Die::new_from_diestring(&ds);
        assert!(bag.is_some());
        let bag = bag.unwrap();

        // Roll our bag of d20s
        let rolls = bag.roll();
        // Chekc we got 5 results
        assert!(rolls.len() == 5);

        // Chekc that all of our results are within a d20's range.
        for result in rolls {
            assert!(result >= 1 && result <= 20);
        }
    }

    /// Ensure that all of our dice are generating witnin the correct
    /// range that we specify.
    #[test]
    fn roll_a_lot_of_d20s() {
        // Create a bag of 2000 D20s
        let ds = String::from("2000d20");
        let bag = Die::new_from_diestring(&ds);
        assert!(bag.is_some());
        let bag = bag.unwrap();

        // Roll our bag of d20s
        let rolls = bag.roll();
        // Chekc we got 5 results
        assert!(rolls.len() == 2000);

        // Chekc that all of our results are within a d20's range.
        // Very unlikeley that if there are range violations that this
        // amount of dice wont detect them.
        for result in rolls {
            assert!(result >= 1 && result <= 20);
        }
    }
}
