pub struct Policy {
    character: char,
    value1: usize,
    value2: usize,
}

impl Policy {
    fn password_meets_range_requirements(&self, password: &str) -> bool {
        let count: usize = password.chars().filter(|c| *c == self.character).count();
        if count < self.value1 || count > self.value2 {
            false
        } else {
            true
        }
    }

    fn password_meets_position_requirements(&self, password: &str) -> bool {
        let position1 = self.value1 - 1;
        let position2 = self.value2 - 1;

        let char1 = password.chars().nth(position1).expect(
            format!(
                "failed to get char at position1: {} for password: {}",
                position1, password
            )
            .as_str(),
        );
        let char2 = password.chars().nth(position2).expect(
            format!(
                "failed to get char at position2: {} for password: {}",
                position2, password
            )
            .as_str(),
        );

        if char1 != char2 && (char1 == self.character || char2 == self.character) {
            true
        } else {
            false
        }
    }
}

pub struct PasswordAndPolicy {
    password: String,
    policy: Policy,
}

impl From<&str> for PasswordAndPolicy {
    fn from(value: &str) -> Self {
        // e.g.
        // "15-16 m: mhmjmzrmmlmmmmmm"
        let mut parts = value.split_whitespace();
        let (value1_str, value2_str) = parts
            .next()
            .expect(format!("failed to get range string from {}", value).as_str())
            .split_once("-")
            .expect(
                format!(
                    "failed to split on hyphen to get value1_str and value2_str of {}",
                    value
                )
                .as_str(),
            );
        let (value1, value2) = (
            value1_str
                .parse::<usize>()
                .expect(format!("failed to parse usize for value1 for {}", value).as_str()),
            value2_str
                .parse::<usize>()
                .expect(format!("failed to parse usize for value2 for {}", value).as_str()),
        );
        let character = parts
            .next()
            .expect(format!("failed to get character string from {}", value).as_str())
            .chars()
            .next()
            .expect(
                format!(
                    "failed to get first char from character string from {}",
                    value
                )
                .as_str(),
            );
        let password = parts
            .next()
            .expect(format!("failed to get password string from {}", value).as_str())
            .to_string();
        Self {
            password,
            policy: Policy {
                character,
                value1,
                value2,
            },
        }
    }
}

impl PasswordAndPolicy {
    pub fn range_requirements_met(&self) -> bool {
        self.policy
            .password_meets_range_requirements(&self.password)
    }

    pub fn position_requirements_met(&self) -> bool {
        self.policy
            .password_meets_position_requirements(&self.password)
    }
}
