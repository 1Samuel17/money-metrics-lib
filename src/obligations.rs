use crate::utilities::Frequency;
use std::fmt::{Display, Formatter, Result};

// TODO: look into seeing if repeated code can be refactored into a generic

#[derive(Debug, PartialEq, Eq)]
pub enum Obligation {
    Mortgage {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Hoa {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Electricity {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Gas {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Water {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Phone {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Internet {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Vehicle {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    StudentLoan {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    CreditCard {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    InstallmentPlan {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Groceries {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
    Tithe {
        amount: i32,
        frequency: Frequency,
        note: String,
    },
}

impl Obligation {
    pub fn variant_as_string(&self) -> String {
        match self {
            Obligation::Mortgage { .. } => String::from("Mortgage"),
            Obligation::Hoa { .. } => String::from("HOA"),
            Obligation::Electricity { .. } => String::from("Electrity"),
            Obligation::Gas { .. } => String::from("Gas"),
            Obligation::Water { .. } => String::from("Water"),
            Obligation::Phone { .. } => String::from("Phone"),
            Obligation::Internet { .. } => String::from("Internet"),
            Obligation::Vehicle { .. } => String::from("Vehicle"),
            Obligation::StudentLoan { .. } => String::from("Student Loan"),
            Obligation::CreditCard { .. } => String::from("Credit Card"),
            Obligation::InstallmentPlan { .. } => String::from("Installment Plan"),
            Obligation::Groceries { .. } => String::from("Groceries"),
            Obligation::Tithe { .. } => String::from("Tithe"),
        }
    }

    pub fn get_amount(&self) -> i32 {
        match self {
            Obligation::Mortgage { amount, .. }
            | Obligation::Hoa { amount, .. }
            | Obligation::Electricity { amount, .. }
            | Obligation::Gas { amount, .. }
            | Obligation::Water { amount, .. }
            | Obligation::Phone { amount, .. }
            | Obligation::Internet { amount, .. }
            | Obligation::Vehicle { amount, .. }
            | Obligation::StudentLoan { amount, .. }
            | Obligation::CreditCard { amount, .. }
            | Obligation::InstallmentPlan { amount, .. }
            | Obligation::Groceries { amount, .. }
            | Obligation::Tithe { amount, .. } => *amount,
        }
    }
}

impl Display for Obligation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Obligation::Mortgage {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Hoa {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Electricity {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Gas {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Water {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Phone {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Internet {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Vehicle {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::StudentLoan {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::CreditCard {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::InstallmentPlan {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Groceries {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
            Obligation::Tithe {
                amount,
                frequency,
                note,
            } => write!(
                f,
                "Obligation: {} -- Amount: {}, Frequency: {}, Note: {}",
                self.variant_as_string(),
                amount,
                frequency,
                note
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Obligations;

impl Obligations {
    pub fn new() -> Vec<Obligation> {
        Vec::<Obligation>::new()
    }
}

// Unit Tests ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_obligation() {
        let obligation = Obligation::Mortgage {
            amount: (3000),
            frequency: (Frequency::Monthly),
            note: (String::from("rounded up")),
        };

        match obligation {
            Obligation::Mortgage {
                amount,
                frequency,
                note,
            } => {
                assert_eq!(amount, 3000);
                assert_eq!(frequency, Frequency::Monthly);
                assert_eq!(note, "rounded up");
            }
            _ => panic!("Something went wrong"),
        }
    }

    #[test]
    fn test_name_as_string() {
        let obligation = Obligation::CreditCard {
            amount: (50),
            frequency: (Frequency::Monthly),
            note: (String::from("minimum payment")),
        };

        assert_eq!(obligation.variant_as_string(), "Credit Card");
    }

    #[test]
    fn test_display_obligation() {
        let obligation = Obligation::InstallmentPlan {
            amount: (110),
            frequency: (Frequency::Biweekly),
            note: (String::from("Klarna pay in four biweekly on payday")),
        };

        assert_eq!(
            format!("{}", obligation),
            "Obligation: Installment Plan -- Amount: 110, Frequency: Biweekly, Note: Klarna pay in four biweekly on payday"
        );
    }

    #[test]
    fn test_new_obliations() {
        let obligations = Obligations::new();

        assert_eq!(obligations.len(), 0);
    }

    #[test]
    fn test_get_amount() {
        let obligation = Obligation::CreditCard {
            amount: (50),
            frequency: (Frequency::Monthly),
            note: (String::from("minimum payment")),
        };

        assert_eq!(obligation.get_amount(), 50);
    }
}
