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

    pub fn get_diff(&self) -> i32 {
        match self {
            Self::Real { incomes, obligations, note } | 
            Self::Hypothetical { incomes, obligations, note } => {
                let total_income = incomes.iter().map(|i| i.get_amount()).fold(0, |acc, a| acc + a );
                let total_obligations = obligations.iter().map(|i| i.get_amount()).fold(0, |acc, a| acc + a );

                total_income - total_obligations
            }
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
    use crate::incomes::{Income, Incomes};
    use crate::obligations::{self, Obligations};
    use crate::utilities::Frequency::{self, Random};

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

    #[test]
    fn test_get_diff() {
        let mut incomes = Incomes::new();
        incomes.push(Income::Person { description: (String::from("Someone")), amount: (50), frequency: (Random), note: (String::from("Zelle")) });
        incomes.push(Income::Person { description: (String::from("Someone")), amount: (50), frequency: (Random), note: (String::from("Zelle")) });
        incomes.push(Income::Person { description: (String::from("Someone")), amount: (50), frequency: (Random), note: (String::from("Zelle")) });
        let mut obligations = Obligations::new();
        obligations.push(Obligation::CreditCard { amount: (51), frequency: (Random), note: (String::from("a dollar short")) });
        obligations.push(Obligation::CreditCard { amount: (51), frequency: (Random), note: (String::from("a dollar short")) });
        obligations.push(Obligation::CreditCard { amount: (51), frequency: (Random), note: (String::from("a dollar short")) });
        let scenario = Scenario::Hypothetical { incomes: (incomes), obligations: (obligations), note: (String::from("diff should be negative 3")) };

        println!("{}", scenario.get_diff());

        assert_eq!(scenario.get_diff(), -3);
    }
}
