use crate::utilities::Frequency;
use std::fmt::{Display, Formatter, Result};

// TODO: look into seeing if repeated code can be refactored into a generic

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Income {
    Employer {
        description: String,
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Property {
        description: String,
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Person {
        description: String,
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Project {
        description: String,
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Business {
        description: String,
        amount: i32,
        frequency: Frequency,
        note: String,
    },
}

impl Income {
    pub fn variant_as_string(&self) -> String {
        match self {
            Income::Employer { .. } => "Employer".to_string(),
            Income::Property { .. } => "Property".to_string(),
            Income::Person { .. } => "Person".to_string(),
            Income::Project { .. } => "Project".to_string(),
            Income::Business { .. } => "Business".to_string(),
        }
    }

    pub fn get_amount(&self) -> i32 {
        match self {
            Income::Employer { amount, .. }
            | Income::Property { amount, .. }
            | Income::Person { amount, .. }
            | Income::Project { amount, .. }
            | Income::Business { amount, .. } => *amount,
        }
    }
}

impl Display for Income {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Income::Employer {
                description,
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Description: {} -- Amount: {}, Frequency: {}, Note: {}",
                description, amount, frequency, note
            ),
            Income::Property {
                description,
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Description: {} -- Amount: {}, Frequency: {}, Note: {}",
                description, amount, frequency, note
            ),
            Income::Person {
                description,
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Description: {} -- Amount: {}, Frequency: {}, Note: {}",
                description, amount, frequency, note
            ),
            Income::Project {
                description,
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Description: {} -- Amount: {}, Frequency: {}, Note: {}",
                description, amount, frequency, note
            ),
            Income::Business {
                description,
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Description: {} -- Amount: {}, Frequency: {}, Note: {}",
                description, amount, frequency, note
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Incomes;

impl Incomes {
    pub fn new() -> Vec<Income> {
        Vec::<Income>::new()
    }
}

// Unit Tests ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_as_string() {
        let income = Income::Employer {
            description: (String::from("My Job")),
            amount: (1000),
            frequency: (Frequency::Biweekly),
            note: (String::from("main income. 40hrs/wk no OT")),
        };

        assert_eq!(income.variant_as_string(), "Employer");
    }

    #[test]
    fn test_display_income() {
        let income = Income::Employer {
            description: (String::from("My Job")),
            amount: (1000),
            frequency: (Frequency::Biweekly),
            note: (String::from("main income. 40hrs/wk no OT")),
        };

        assert_eq!(
            format!("{}", income),
            "Description: My Job -- Amount: 1000, Frequency: Biweekly, Note: main income. 40hrs/wk no OT"
        );
    }

    #[test]
    fn test_new_incomes() {
        let incomes = Incomes::new();

        assert_eq!(incomes.len(), 0);
    }

    #[test]
    fn test_get_amount() {
        let income = Income::Employer {
            description: (String::from("day job")),
            amount: (1000),
            frequency: (Frequency::Biweekly),
            note: (String::from("get amount")),
        };

        assert_eq!(income.get_amount(), 1000);
    }
}
