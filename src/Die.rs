use regex::Regex;
use rand::distributions::{Distribution, Uniform};

pub struct Die {
    die_string: String,
    die_count: u16,
    die_value: u16,
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
    fn new_from_diestring(die_string: String) -> Option<Die> {
        if !validate_diestring(die_string.clone()) {
            return None;
        }
        let parts: Vec<String> = die_string.split("d").map(String::from).collect();
        let die_count = parts.get(0).unwrap().parse::<u16>().unwrap();
        let die_value = parts.get(1).unwrap().parse::<u16>().unwrap();
        Some(Die{die_string, die_count, die_value})
    }

    /// Roll a given (bag of) di(c)e and return the results.
    ///
    /// Since our die implementation treats any given die as potentially being
    /// a bag, we will return a vector of results that represent each and
    /// every die being rolled.
    /// This means that the result of rolling a 1d6 is a 1 length vector
    /// of results, and the result of rolling 3d6 is a 3-length vector.
    fn roll(&self) -> Vec<u16> {
        let mut rng = rand::thread_rng();
        let diemax = self.die_value + 1;
        let distrib = Uniform::from(1..diemax);
        let mut rolls: Vec<u16> = Vec::new();
        for _i in 1..self.die_count {
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
