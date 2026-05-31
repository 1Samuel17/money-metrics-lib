use std::fmt::{Display, Formatter, Result};

// TODO:

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Frequency {
    Monthly,
    #[default]
    Biweekly,
    Random,
}

impl Frequency {
    pub fn as_string(&self) -> String {
        match self {
            Self::Monthly => "Monthly".to_string(),
            Self::Biweekly => "Biweekly".to_string(),
            Self::Random => "Random".to_string(),
        }
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Frequency::Monthly => write!(f, "{}", self.as_string()),
            Frequency::Biweekly => write!(f, "{}", self.as_string()),
            Frequency::Random => write!(f, "{}", self.as_string()),
        }
    }
}

// Functionality Tests ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_as_string() {
        let frequency = Frequency::Monthly.as_string();

        assert_eq!(frequency, "Monthly");
    }

    #[test]
    fn test_default_frequency() {
        let frequency = Frequency::default();

        assert_eq!(frequency, Frequency::Biweekly);
    }
}
