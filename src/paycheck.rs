use crate::utilities::Frequency;

// TODO: finish building up this module
// - create deductions enum
// - create withholdings enum
// - implement paycheck impl block

#[derive(Debug, PartialEq)]
pub enum Paycheck {
    Real {
        salary: Option<f32>,
        hourly: Option<f32>,
        hours: Option<f32>,
        frequency: Frequency,
        pretax: Option<Vec<PreTaxDeduction>>,
        posttax: Option<Vec<PostTaxDeduction>>,
    },
    Hypothetical {
        salary: Option<f32>,
        hourly: Option<f32>,
        hours: Option<f32>,
        frequency: Frequency,
        pretax: Option<Vec<PreTaxDeduction>>,
        posttax: Option<Vec<PostTaxDeduction>>,
    },
}

impl Paycheck {
    pub fn variant_as_string(&self) -> String {
        match self {
            Self::Real { .. } => "Real".to_string(),
            Self::Hypothetical { .. } => "Hypothetical".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PreTaxDeduction {
    Medical(Option<f32>),
    Dental(Option<f32>),
    Vision(Option<f32>),
    Traditional401K(Option<DeductionType>),
    HSA(Option<DeductionType>),
    FSA(Option<DeductionType>),
}

impl PreTaxDeduction {
    pub fn variant_as_string(&self) -> String {
        match self {
            Self::Medical(_) => String::from("Medical"),
            Self::Dental(_) => String::from("Dental"),
            Self::Vision(_) => String::from("Vision"),
            Self::Traditional401K(_) => String::from("Traditional 401k"),
            Self::HSA(_) => String::from("Health Savings Account (HSA)"),
            Self::FSA(_) => String::from("Flexible Spending Account (FSA)"),
        }
    }

    pub fn get_amount(&self, gross_check: Option<f32>) -> f32 {
        match self {
            Self::Medical(amount) | Self::Dental(amount) | Self::Vision(amount) => amount.unwrap(),
            Self::Traditional401K(deduction) | Self::HSA(deduction) | Self::FSA(deduction) => {
                match deduction.as_ref().unwrap() {
                    DeductionType::Percentage(percent) => {
                        gross_check.unwrap_or_default() * (percent.unwrap() / 100.0)
                    }
                    DeductionType::DollarAmount(dollars) => {
                        gross_check.unwrap_or_default() - dollars.unwrap()
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PreTaxDeductions;

impl PreTaxDeductions {
    pub fn new() -> Vec<PreTaxDeduction> {
        Vec::<PreTaxDeduction>::new()
    }
}

#[derive(Debug, PartialEq)]
pub enum PostTaxDeduction {
    Roth401K(Option<DeductionType>),
    VoluntaryLife(Option<f32>),
    VoluntaryADD(Option<f32>),
    VoluntaryLTD(Option<f32>),
    VoluntarySTD(Option<f32>),
    Garnishment(Option<f32>),
}

impl PostTaxDeduction {
    pub fn variant_as_string(&self) -> String {
        match self {
            Self::Roth401K(_) => String::from("Roth 401k"),
            Self::VoluntaryLife(_) => String::from("Voluntary Life Insurance"),
            Self::VoluntaryADD(_) => {
                String::from("Voluntary Accidental Death & Dismemberment (ADD)")
            }
            Self::VoluntaryLTD(_) => String::from("Voluntary Long Term Disability (LTD)"),
            Self::VoluntarySTD(_) => String::from("Voluntary Short Term Disability (STD)"),
            Self::Garnishment(_) => String::from("Wage Garnishment"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PostTaxDeductions;

impl PostTaxDeductions {
    pub fn new() -> Vec<PostTaxDeduction> {
        Vec::<PostTaxDeduction>::new()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeductionType {
    Percentage(Option<f32>),
    DollarAmount(Option<f32>),
}

impl DeductionType {
    pub fn variant_as_string(&self) -> String {
        match self {
            Self::Percentage(_) => String::from("Percentage"),
            Self::DollarAmount(_) => String::from("Dollar Amount"),
        }
    }
}

// Functionality Tests ----------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::Frequency::{Biweekly};

    #[test]
    fn test_new_deduction_type() {
        todo!()
    }

    #[test]
    fn test_new_paycheck() {
        let check = Paycheck::Hypothetical {
            salary: (None),
            hourly: (Some(25.0)),
            hours: (Some(40.0)),
            frequency: (Frequency::Biweekly),
            pretax: (None),
            posttax: (None),
        };

        match check {
            Paycheck::Hypothetical {
                salary,
                hourly,
                hours,
                frequency,
                pretax,
                posttax,
            } => {
                assert_eq!(salary, None);
                assert_eq!(hourly.unwrap(), 25.0);
                assert_eq!(hours.unwrap(), 40.0);
                assert_eq!(frequency, Biweekly);
                assert_eq!(pretax, None);
                assert_eq!(posttax, None);
            }
            _ => panic!("Semething went wrong"),
        }
    }

    #[test]
    fn test_paycheck_variant_as_string() {
        let check = Paycheck::Hypothetical {
            salary: (None),
            hourly: (Some(25.0)),
            hours: (Some(40.0)),
            frequency: (Frequency::Biweekly),
            pretax: (None),
            posttax: (None),
        };

        assert_eq!(check.variant_as_string(), "Hypothetical");
    }

    #[test]
    fn test_new_pretax_deductions() {
        let pretax = PreTaxDeductions::new();

        assert_eq!(pretax.len(), 0);
    }

    #[test]
    fn test_new_pretax_deduction() {
        let pre_deduct = PreTaxDeduction::Dental(Some(32.0));

        if let PreTaxDeduction::Dental(Some(32.0)) = pre_deduct {
            assert!(true)
        }
    }

    #[test]
    fn test_pretax_variant_as_string() {
        todo!()
    }

    #[test]
    fn test_get_pretax_deduction_amount() {
        todo!()
    }

    #[test]
    fn test_new_posttax_deductions() {
        todo!()
    }

    #[test]
    fn test_new_posttax_deduction() {
        todo!()
    }

    #[test]
    fn test_posttax_variant_as_string() {
        todo!()
    }

    #[test]
    fn test_get_posttax_deduction_amount() {
        todo!()
    }
}
