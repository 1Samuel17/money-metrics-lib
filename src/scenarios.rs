use crate::{incomes::Income, obligations::Obligation};
use std::fmt::{Display, Formatter, Result};

// TODO:

#[derive(Debug, PartialEq, Eq)]
pub enum Scenario {
    Real {
        incomes: Vec<Income>,
        obligations: Vec<Obligation>,
        note: String,
    },
    Hypothetical {
        incomes: Vec<Income>,
        obligations: Vec<Obligation>,
        note: String,
    },
}

impl Scenario {
    pub fn variant_as_string(&self) -> String {
        match self {
            Self::Real { .. } => "Real".to_string(),
            Self::Hypothetical { .. } => "Hypothetical".to_string(),
        }
    }
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Scenario::Real {
                incomes,
                obligations,
                note,
            } => write!(
                f,
                "Scenario: {} -- Incomes: {:?}, Obligations: {:? }, Note: {}",
                self.variant_as_string(),
                incomes,
                obligations,
                note
            ),
            Scenario::Hypothetical {
                incomes,
                obligations,
                note,
            } => write!(
                f,
                "Scenario: {} -- Incomes: {:?}, Obligations: {:? }, Note: {}",
                self.variant_as_string(),
                incomes,
                obligations,
                note
            ),
        }
    }
}

pub struct Scenarios;

impl Scenarios {
    pub fn new() -> Vec<Scenario> {
        Vec::<Scenario>::new()
    }
}

// Unit Tests ----------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incomes::Incomes;
    use crate::obligations::Obligations;

    #[test]
    fn test_new_scenario() {
        let incomes = Incomes::new();
        let obligations = Obligations::new();
        let scenario = Scenario::Real {
            incomes: (incomes),
            obligations: (obligations),
            note: (String::from("Test Note")),
        };

        match scenario {
            Scenario::Real {
                incomes,
                obligations,
                note,
            } => {
                assert_eq!(incomes.len(), 0);
                assert_eq!(obligations.len(), 0);
                assert_eq!(note, "Test Note");
            }
            _ => panic!("test failed"),
        }
    }
}
