#[derive(Debug, Clone, Copy)]
pub struct Rule {
    pub first: u8,
    pub second: u8,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        // e.g. 69|26
        let (first, second) = value.split_once('|').unwrap();
        let (first, second) = (first.parse::<u8>().unwrap(), second.parse::<u8>().unwrap());
        Self { first, second }
    }
}

pub type Sequence = Vec<u8>;
pub trait RuleOps {
    fn find_rule(&self, rule: Rule) -> Option<(usize, usize)>;
    fn follows_rule(&self, rule: Rule) -> bool;
    fn sort_by_rule(&mut self, rule: Rule);
}
impl RuleOps for Sequence {
    fn find_rule(&self, rule: Rule) -> Option<(usize, usize)> {
        let mut first = None;
        let mut second = None;
        for (i, &n) in self.iter().enumerate() {
            if n == rule.first {
                first = Some(i);
            }
            if n == rule.second {
                second = Some(i);
            }
            if first.is_some() && second.is_some() {
                break;
            }
        }
        match (first, second) {
            (Some(first), Some(second)) => Some((first, second)),
            _ => None,
        }
    }
    fn follows_rule(&self, rule: Rule) -> bool {
        let Some((first, second)) = self.find_rule(rule) else {
            return true;
        };
        first < second
    }
    fn sort_by_rule(&mut self, rule: Rule) {
        let Some((first, second)) = self.find_rule(rule) else {
            return;
        };
        if first > second {
            let item = self.remove(first);
            self.insert(second, item);
        }
    }
}

pub trait RulesOps {
    fn follows_rules(&self, rules: &[Rule]) -> bool;
    fn sort_by_rules(&mut self, rules: &[Rule]);
}
impl RulesOps for Sequence {
    fn follows_rules(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|r| self.follows_rule(*r))
    }
    fn sort_by_rules(&mut self, rules: &[Rule]) {
        while !self.follows_rules(rules) {
            rules.iter().for_each(|r| self.sort_by_rule(*r))
        }
    }
}
